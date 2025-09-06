use crate::prelude::*;

pub enum Effects {
    WeaponDamage(),
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
    let target_character = state.level.find_character_mut(target);
    target_character.health.current -= weapon.damage;

    // We do not remove the player character, death checks will happen after action resolution
    if target_character.health.is_dead() && !target_character.is_player() {
        state.level.remove_character(target);
    }
    spend_ticks(state, source, TICKS_TO_ACT);
}

pub fn character_wait(state: &mut State, id: CharacterId) {
    spend_ticks(state, id, TICKS_TO_ACT);
}
