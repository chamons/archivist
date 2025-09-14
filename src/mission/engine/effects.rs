use macroquad::rand::ChooseRandom;
use macroquad::rand::gen_range;

use crate::mission::*;
use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum Effect {
    ApplyDamage {
        damage: i32,

        #[serde(default)]
        on_hit: Option<Box<Effect>>,

        #[serde(default)]
        on_hit_self: Option<Box<Effect>>,

        #[serde(default)]
        pierce: DamagePierce,
    },
    AddStatus {
        effect: StatusEffect,
    },
    Heal {
        amount: i32,
    },
}

pub fn move_character(state: &mut MissionState, id: CharacterId, dest: Point, screen: &mut Screen) {
    if state.level.find_character_at_position(dest).is_none() && state.level.map.can_enter(dest) {
        let actor = state.level.find_character_mut(id);
        let has_quick = actor.has_status_effect(StatusEffectKind::Quick);
        let has_slow = actor.has_status_effect(StatusEffectKind::Slow);

        let skip_move = actor.has_status_effect(StatusEffectKind::Rooted)
            && gen_range(0.0, 1.0) < STATUS_EFFECT_CHANCE_ROOT_STAY_STILL;

        if !skip_move {
            actor.position = dest;
            if actor.is_player() {
                state.level.update_visibility();
                pickup_any_items(state, id, dest, screen);
            }
        } else {
            let log = format!("{} was unable to move", actor.name.clone());
            state.level.push_turn_log(log);
        }

        let mut tick_cost = if has_quick {
            TICKS_MOVEMENT / 2
        } else {
            TICKS_MOVEMENT
        };
        tick_cost = if has_slow { tick_cost * 2 } else { tick_cost };

        spend_ticks(state, id, tick_cost);
    }
}

fn pickup_any_items(state: &mut MissionState, id: CharacterId, dest: Point, screen: &mut Screen) {
    let items_at_new_position: Vec<Item> = state
        .level
        .items
        .extract_if(.., |(position, _)| *position == dest)
        .map(|(_, item)| item)
        .collect();

    let actor = state.level.find_character_mut(id);
    for item in items_at_new_position {
        screen.push_floating_text(&format!("Picked up {}", item.name));
        actor.carried_items.push(item);
    }
}

pub fn weapon_attack<S: ScreenInterface>(
    state: &mut MissionState,
    source: CharacterId,
    target: CharacterId,
    weapon: Weapon,
    screen: &mut S,
) {
    apply_damage(
        &mut state.level,
        &EffectSource::Character(source),
        target,
        weapon.damage,
        weapon.pierce,
    );

    match gen_range(0, 3) {
        0 => screen.play_sound("impact_a"),
        1 => screen.play_sound("impact_b"),
        _ => screen.play_sound("attack_b"),
    }

    if let Some(on_hit) = &weapon.on_hit {
        apply_effect(
            &mut state.level,
            &EffectSource::Character(source),
            target,
            on_hit,
        );
    }
    spend_ticks(state, source, TICKS_TO_ACT);
}

fn calculate_damage(
    level: &LevelState,
    source: &EffectSource,
    target: CharacterId,
    base_damage: i32,
    pierce: DamagePierce,
) -> (i32, String) {
    let mut damage_description = String::new();
    let source_name = source.name(level);
    let target_name = level.find_character(target).name.clone();

    // First check if we dodged due to Blind or Agile
    if source.has_status_effect(StatusEffectKind::Blind, level)
        && gen_range(0.0, 1.0) < STATUS_EFFECT_CHANCE_BLIND_MISS
    {
        return (
            0,
            format!("{source_name} missed their attack on {target_name}"),
        );
    } else if level
        .find_character(target)
        .has_status_effect(StatusEffectKind::Agile)
        && gen_range(0.0, 1.0) < STATUS_EFFECT_CHANCE_DODGE_MISS
    {
        return (0, format!("{target_name} dodged {source_name}'s attack"));
    }

    // Start with the base damage
    let mut damage = base_damage;
    damage_description.push_str(&format!("{source_name} deals {base_damage}"));

    // Add Might and Subtract Weakness
    if source.has_status_effect(StatusEffectKind::Might, level) {
        damage += STATUS_EFFECT_MIGHT_DAMAGE_BOOST;
        damage_description.push_str(&format!(" + {STATUS_EFFECT_MIGHT_DAMAGE_BOOST}(Might)"));
    }
    if source.has_status_effect(StatusEffectKind::Weakness, level) {
        damage -= STATUS_EFFECT_WEAKNESS_DAMAGE_REDUCTION;
        damage_description.push_str(&format!(
            " - {STATUS_EFFECT_WEAKNESS_DAMAGE_REDUCTION}(Weakness)"
        ));
    }

    // Roll advantage/disadvantage
    let roll = get_advantage_roll(level, source);
    damage += roll;

    match roll {
        i32::MIN..0 => damage_description.push_str(&format!(" - {}(Advantage)", roll.abs())),
        0 => damage_description.push_str(" + 0(Advantage)"),
        1_i32..=i32::MAX => damage_description.push_str(&format!(" + {roll}(Advantage)")),
    }

    // Then finally subtract the defense (plus any protection) from the damage
    let mut defense = get_target_defense(level, target);
    match pierce {
        DamagePierce::None => {}
        DamagePierce::Some => {
            defense -= DEFENSE_IGNORED_SOME_PIERCE;
            defense = defense.max(0);
        }
        DamagePierce::Full => defense = 0,
    }
    damage -= defense;

    damage_description.push_str(&format!(" - {defense}(defense)"));

    // Make sure damage is never negative (no healing wacks)
    if damage < 0 {
        damage = 0;
    }
    damage_description.push_str(&format!(" = {damage} to {target_name}"));

    (damage, damage_description)
}

fn get_target_defense(level: &LevelState, target: CharacterId) -> i32 {
    let target_character = level.find_character(target);
    let mut defense = target_character.defense;
    if target_character.has_status_effect(StatusEffectKind::Protection) {
        defense += STATUS_EFFECT_PROTECTION_DEFENSE_BOOST;
    }
    defense += get_luck_defensive_rolls(level, target);
    defense
}

fn get_advantage_roll(level: &LevelState, source: &EffectSource) -> i32 {
    let die = if source.has_status_effect(StatusEffectKind::Lucky, level) {
        vec![0, 1]
    } else if source.has_status_effect(StatusEffectKind::Cursed, level) {
        vec![-1, 0]
    } else {
        vec![-1, 0, 1]
    };
    *die.choose().unwrap()
}

fn get_luck_defensive_rolls(level: &LevelState, target: CharacterId) -> i32 {
    let target = level.find_character(target);
    let die = if target.has_status_effect(StatusEffectKind::Lucky) {
        vec![0, 1]
    } else if target.has_status_effect(StatusEffectKind::Cursed) {
        vec![-1, 0]
    } else {
        vec![0]
    };
    *die.choose().unwrap()
}

fn apply_damage(
    level: &mut LevelState,
    source: &EffectSource,
    target: CharacterId,
    damage: i32,
    pierce: DamagePierce,
) {
    let (final_damage, damage_description) =
        calculate_damage(level, &source, target, damage, pierce);
    level.push_turn_log(damage_description);

    let target_character = level.find_character_mut(target);

    target_character.health.current -= final_damage;

    // We do not remove the player character, death checks will happen after action resolution
    if target_character.health.is_dead() && !target_character.is_player() {
        level.remove_character(target);
    }

    if source.has_status_effect(StatusEffectKind::Lifesteal, level) {
        if let EffectSource::Character(source) = source {
            apply_healing(level, source, 2);
        }
    }
}

fn add_status(level: &mut LevelState, target: CharacterId, status: StatusEffect) {
    let target_character = level.find_character_mut(target);
    let name = target_character.name.clone();
    let status_name = status.name.clone();
    target_character.status_effects.push(status);

    level.push_turn_log(format!("{name} gains {}", status_name));
}

fn apply_healing(level: &mut LevelState, target: &CharacterId, amount: i32) {
    let target_character = level.find_character_mut(*target);
    let name = target_character.name.clone();
    target_character.health.increase(amount);

    level.push_turn_log(format!("{name} is healed for {amount}"));
}

pub fn character_wait(state: &mut MissionState, id: CharacterId, screen: &mut Screen) {
    if !is_hostile_nearby(state, id) {
        rest(state, id, screen);
    }
    spend_ticks(state, id, TICKS_TO_ACT);
}

fn rest(state: &mut MissionState, id: CharacterId, screen: &mut Screen) {
    let actor = state.level.find_character_mut(id);
    let mut rested = false;
    if actor.health.percentage() < REST_HEALTH_PERCENTAGE {
        actor.health.current += 1;
        rested = true;
    }
    if actor.will.percentage() < REST_WILL_PERCENTAGE {
        actor.will.current += 1;
        rested = true;
    }
    if rested {
        screen.push_floating_text("Resting...");
    }
}

fn is_hostile_nearby(state: &MissionState, id: CharacterId) -> bool {
    let actor = state.level.find_character(id);
    let visibility = state.level.map.compute_visibility(actor.position);
    if actor.is_player() {
        state
            .level
            .characters
            .iter()
            .filter(|c| !c.is_player())
            .any(|c| visibility.get(c.position))
    } else {
        visibility.get(state.get_player().position)
    }
}

pub fn is_hostile_near_player(level: &LevelState) -> bool {
    let player = level.get_player();
    let visibility = level.map.compute_visibility(player.position);

    level
        .characters
        .iter()
        .filter(|c| !c.is_player())
        .any(|c| visibility.get(c.position))
}

pub fn apply_skill(
    state: &mut MissionState,
    source: CharacterId,
    target: CharacterId,
    skill_name: &str,
    screen: &mut Screen,
) {
    if source == target {
        state.level.push_turn_log(format!(
            "{} uses {}",
            state.level.find_character(source).name,
            skill_name
        ));
    } else {
        state.level.push_turn_log(format!(
            "{} uses {} on {}",
            state.level.find_character(source).name,
            skill_name,
            state.level.find_character(target).name,
        ));
    }

    let actor = state.level.find_character_mut(source);

    let skill = actor
        .skills
        .iter_mut()
        .find(|s| s.name == skill_name)
        .expect("Unable to find requested skill");
    match &mut skill.cost {
        SkillCost::None => {}
        SkillCost::Will(cost) => actor.will.current -= *cost,
        SkillCost::Charges { remaining, .. } => *remaining -= 1,
        SkillCost::Cooldown { ticks, cost } => *ticks = *cost,
    }
    let effect = skill.effect.clone();

    match &skill.effect {
        Effect::ApplyDamage { .. } => screen.play_sound("curse"),
        Effect::AddStatus { .. } => screen.play_sound("swing"),
        Effect::Heal { .. } => screen.play_sound("drip"),
    }

    apply_effect(
        &mut state.level,
        &EffectSource::Character(source),
        target,
        &effect,
    );

    spend_ticks(state, source, TICKS_TO_ACT);
}

#[derive(Debug)]
pub enum EffectSource {
    Character(CharacterId),
    StatusEffect(String),
}

impl EffectSource {
    pub fn has_status_effect(&self, kind: StatusEffectKind, level: &LevelState) -> bool {
        match self {
            EffectSource::Character(character_id) => {
                level.find_character(*character_id).has_status_effect(kind)
            }
            EffectSource::StatusEffect(_) => false,
        }
    }

    pub fn name(&self, level: &LevelState) -> String {
        match self {
            EffectSource::Character(character_id) => {
                level.find_character(*character_id).name.clone()
            }
            EffectSource::StatusEffect(name) => name.clone(),
        }
    }
}

pub fn apply_effect(
    level: &mut LevelState,
    source: &EffectSource,
    target: CharacterId,
    effect: &Effect,
) {
    match effect {
        Effect::ApplyDamage {
            damage,
            on_hit,
            on_hit_self,
            pierce,
        } => {
            apply_damage(level, &source, target, *damage, *pierce);
            if let Some(on_hit) = &on_hit {
                apply_effect(level, source, target, on_hit);
            }
            if let Some(on_hit_self) = &on_hit_self {
                if let EffectSource::Character(id) = source {
                    apply_effect(level, source, *id, on_hit_self);
                }
            }
        }
        Effect::Heal { amount } => {
            apply_healing(level, &target, *amount);
        }
        Effect::AddStatus { effect } => {
            add_status(level, target, effect.clone());
        }
    }
}

pub fn ascend_stars(state: &mut MissionState, screen: &mut Screen) {
    let player = state.get_player();
    let on_exit = state.level.map.get(player.position).kind == TileKind::Exit;
    let has_runestone = player.carried_items.iter().any(|i| i.name == "Runestone");
    if on_exit {
        if has_runestone {
            spend_ticks(state, player.id, TICKS_TO_ACT);
            state.mission_complete = true;
        } else {
            screen.push_floating_text("Retrieve the Runestone first!");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::campaign::{CampaignState, RuneKinds};
    use crate::mission::*;
    use crate::screen::EmptyScreen;

    #[test]
    fn on_hit() {
        let (id, mut level) = create_test_map();

        level.find_character_mut(id).weapon.on_hit = Some(Effect::AddStatus {
            effect: StatusEffect {
                name: "Weakness".to_string(),
                kind: StatusEffectKind::Weakness,
                duration: Some(200),
                on_complete: None,
            },
        });

        let character = level.get_player().clone();
        let mut mission_state = MissionState {
            level,
            frame: 0,
            current_actor: CurrentActor::PlayerStandardAction,
            mission_complete: false,
            campaign: CampaignState::new(character),
            active_rune: RuneKinds::Fire,
        };

        let player_id = mission_state.level.get_player().id;
        let weapon = mission_state.level.find_character(id).weapon.clone();

        weapon_attack(
            &mut mission_state,
            id,
            player_id,
            weapon,
            &mut EmptyScreen {},
        );

        assert!(
            mission_state
                .level
                .get_player()
                .has_status_effect(StatusEffectKind::Weakness)
        );
    }
}
