use crate::mission::*;
use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum Effect {
    ApplyDamage { damage: i32 },
    AddStatus { effect: StatusEffect },
    Heal { amount: i32 },
}

pub fn move_character(state: &mut MissionState, id: CharacterId, dest: Point, screen: &mut Screen) {
    if state.level.find_character_at_position(dest).is_none() && state.level.map.can_enter(dest) {
        let actor = state.level.find_character_mut(id);
        actor.position = dest;
        if actor.is_player() {
            state.level.update_visibility();
            pickup_any_items(state, id, dest, screen);
        }

        spend_ticks(state, id, TICKS_MOVEMENT);
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

pub fn weapon_attack(
    state: &mut MissionState,
    source: CharacterId,
    target: CharacterId,
    weapon: Weapon,
) {
    apply_damage(&mut state.level, source, target, weapon.damage);
    spend_ticks(state, source, TICKS_TO_ACT);
}

fn apply_damage(level: &mut LevelState, source: CharacterId, target: CharacterId, damage: i32) {
    let mut damage = damage;
    if level
        .find_character(source)
        .has_status_effect(StatusEffectKind::Might)
    {
        damage += STATUS_EFFECT_MIGHT_DAMAGE_BOOST;
    }

    let target_character = level.find_character_mut(target);
    let mut defense = target_character.defense;
    if target_character.has_status_effect(StatusEffectKind::Protection) {
        defense += STATUS_EFFECT_PROTECTION_DEFENSE_BOOST;
    }
    damage -= defense;
    if damage == 0 {
        damage = 1;
    }

    target_character.health.current -= damage;

    // We do not remove the player character, death checks will happen after action resolution
    if target_character.health.is_dead() && !target_character.is_player() {
        level.remove_character(target);
    }
}

fn add_status(level: &mut LevelState, target: CharacterId, status: StatusEffect) {
    let target_character = level.find_character_mut(target);
    target_character.status_effects.push(status);
}

fn apply_healing(level: &mut LevelState, target: CharacterId, amount: i32) {
    let target_character = level.find_character_mut(target);
    target_character.health.increase(amount);
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

pub fn apply_skill(
    state: &mut MissionState,
    source: CharacterId,
    target: CharacterId,
    skill_name: &str,
) {
    let actor = state.level.find_character_mut(source);
    let skill = actor
        .skills
        .iter_mut()
        .find(|s| s.name == skill_name)
        .expect("Unable to find requested skill");
    match &mut skill.cost {
        SkillCost::Will(cost) => actor.will.current -= *cost,
        SkillCost::Charges { remaining, .. } => *remaining -= 1,
    }
    let effect = skill.effect.clone();
    apply_effect(&mut state.level, source, target, &effect);

    spend_ticks(state, source, TICKS_TO_ACT);
}

pub fn apply_effect(
    level: &mut LevelState,
    source: CharacterId,
    target: CharacterId,
    effect: &Effect,
) {
    match effect {
        Effect::ApplyDamage { damage } => {
            apply_damage(level, source, target, *damage);
        }
        Effect::Heal { amount } => {
            apply_healing(level, target, *amount);
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
