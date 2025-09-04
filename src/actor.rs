use crate::prelude::*;

// In a turn based game, only sometimes does the player get to move
// This contains what the current "thing takes it's turn" is
// which could be an animation for example
#[derive(Debug, Serialize, Deserialize)]
pub enum CurrentActor {
    PlayerStandardAction,
    PlayerTargeting(TargetingInfo),
    EnemyAction(CharacterId),
}

impl CurrentActor {
    pub fn act(&mut self, level: &LevelState, screen: &Screen) -> Option<RequestedAction> {
        match self {
            CurrentActor::PlayerStandardAction => {
                let player = level.get_player();
                get_player_action(player)
            }
            CurrentActor::PlayerTargeting(targeting_info) => {
                let is_current_target_valid = Self::is_current_target_valid(targeting_info, level);
                targeting_info.handle_input(level, screen, is_current_target_valid)
            }
            CurrentActor::EnemyAction(id) => Some(default_action(level, *id)),
        }
    }

    pub fn render(&mut self, screen: &Screen, level: &LevelState) {
        if let CurrentActor::PlayerTargeting(targeting_info) = self {
            let should_draw = targeting_info.blink.tick();

            if should_draw {
                let color = if Self::is_current_target_valid(&targeting_info, level) {
                    WHITE
                } else {
                    RED
                };
                screen.draw_targeting(targeting_info.position, color);
            }
        }
    }

    fn is_current_target_valid(targeting_info: &TargetingInfo, level: &LevelState) -> bool {
        let character_target_target = level.find_character_at_position(targeting_info.position);
        character_target_target.is_some() && !character_target_target.unwrap().is_player()
    }

    pub fn needs_to_wait(&self) -> bool {
        match self {
            CurrentActor::PlayerStandardAction => true,
            CurrentActor::PlayerTargeting(_) => true,
            CurrentActor::EnemyAction(_) => false,
        }
    }
}
