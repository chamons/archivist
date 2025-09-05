use crate::prelude::*;

mod effects;
mod level;

pub use effects::*;
pub use level::*;

pub fn spend_ticks(
    level: &mut LevelState,
    current_actor: &mut CurrentActor,
    id: CharacterId,
    amount: i32,
) {
    level.find_character_mut(id).ticks -= amount;

    if let Some(next) = find_next_actor(level) {
        if level.find_character(next).is_player() {
            *current_actor = CurrentActor::PlayerStandardAction;
        } else {
            *current_actor = CurrentActor::EnemyAction(next);
        }
    }
}

fn find_next_actor(level: &mut LevelState) -> Option<CharacterId> {
    // Sort by ticks with id as tiebreaker
    level.characters.sort_by_key(|c| (c.ticks, c.id));
    if let Some(actor) = level.characters.last() {
        let id = actor.id;
        if actor.ticks < TICKS_TO_ACT {
            let missing = TICKS_TO_ACT - actor.ticks;
            add_ticks(level, missing);
        }
        Some(id)
    } else {
        None
    }
}

fn add_ticks(level: &mut LevelState, amount: i32) {
    for character in &mut level.characters {
        character.ticks += amount;
    }
}
