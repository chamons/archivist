use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum Effect {
    ApplyWeaponDamage { weapon: Weapon },
    ApplyDamage { damage: i32 },
}

pub fn move_character(state: &mut State, id: CharacterId, dest: Point) {
    if state.level.find_character_at_position(dest).is_none() && state.level.map.can_enter(dest) {
        let actor = state.level.find_character_mut(id);
        actor.position = dest;
        if actor.is_player() {
            state.level.update_visibility();
        }

        spend_ticks(state, id, TICKS_MOVEMENT);
    }
}

pub fn weapon_attack(state: &mut State, source: CharacterId, target: CharacterId, weapon: Weapon) {
    apply_damage(&mut state.level, target, weapon.damage);
    spend_ticks(state, source, TICKS_TO_ACT);
}

fn apply_damage(level: &mut LevelState, target: CharacterId, damage: i32) {
    let target_character = level.find_character_mut(target);
    target_character.health.current -= damage;

    // We do not remove the player character, death checks will happen after action resolution
    if target_character.health.is_dead() && !target_character.is_player() {
        level.remove_character(target);
    }
}

pub fn character_wait(state: &mut State, id: CharacterId) {
    spend_ticks(state, id, TICKS_TO_ACT);
}

pub fn apply_effect(state: &mut State, source: CharacterId, target: CharacterId, effect: Effect) {
    match effect {
        Effect::ApplyWeaponDamage { weapon } => weapon_attack(state, source, target, weapon),
        Effect::ApplyDamage { damage } => {
            apply_damage(&mut state.level, target, damage);
        }
    }
    spend_ticks(state, source, TICKS_TO_ACT);
}
