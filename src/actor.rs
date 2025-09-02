use crate::prelude::*;

// In a turn based game, only sometimes does the player get to move
// This contains what the current "thing takes it's turn" is
// which could be an animation for example
#[derive(Debug)]
pub enum CurrentActor {
    PlayerAction,
    EnemyAction(CharacterId),
}

impl CurrentActor {
    pub fn act(&mut self, level: &LevelState, ctx: &mut BTerm) -> Option<RequestedAction> {
        match self {
            CurrentActor::PlayerAction => {
                let player = level.get_player();
                get_player_action(player, ctx)
            }
            CurrentActor::EnemyAction(id) => Some(wander_action(level, *id)),
        }
    }

    pub fn needs_to_wait(&self) -> bool {
        match self {
            CurrentActor::PlayerAction => true,
            CurrentActor::EnemyAction(_) => false,
        }
    }
}
