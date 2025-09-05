use crate::prelude::*;

// In a turn based game, only sometimes does the player get to move
// This contains what the current "thing takes it's turn" is
// which could be an animation for example
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CurrentActor {
    PlayerStandardAction,
    PlayerTargeting(TargetingInfo),
    EnemyAction(CharacterId),
    Animation(AnimationInfo),
}

pub enum HandleInputResponse {
    Action(Option<RequestedAction>),
    ChangeActor(CurrentActor),
}

impl CurrentActor {
    pub fn act(&mut self, level: &LevelState, screen: &Screen) -> Option<RequestedAction> {
        match self {
            CurrentActor::PlayerStandardAction => {
                let player = level.get_player();
                self.process_input_response(get_player_action(player, level))
            }
            CurrentActor::PlayerTargeting(targeting_info) => {
                let is_current_target_valid = Self::is_current_target_valid(targeting_info, level);
                let response = targeting_info.handle_input(level, screen, is_current_target_valid);
                self.process_input_response(response)
            }
            CurrentActor::EnemyAction(id) => Some(default_action(level, *id)),
            CurrentActor::Animation(animation_info) => {
                let response = animation_info.handle_input();
                self.process_input_response(response)
            }
        }
    }

    fn process_input_response(&mut self, response: HandleInputResponse) -> Option<RequestedAction> {
        match response {
            HandleInputResponse::Action(requested_action) => requested_action,
            HandleInputResponse::ChangeActor(current_actor) => {
                *self = current_actor.clone();
                None
            }
        }
    }

    pub fn render(&self, screen: &Screen, level: &LevelState) {
        match self {
            CurrentActor::PlayerTargeting(targeting_info) => {
                targeting_info.render(screen, level);
            }
            CurrentActor::Animation(animation_info) => animation_info.render(screen),
            _ => {}
        }
    }

    pub fn is_current_target_valid(targeting_info: &TargetingInfo, level: &LevelState) -> bool {
        let character_target_target = level.find_character_at_position(targeting_info.position);
        character_target_target.is_some() && !character_target_target.unwrap().is_player()
    }

    pub fn needs_to_wait(&self) -> bool {
        match self {
            CurrentActor::PlayerStandardAction => true,
            CurrentActor::PlayerTargeting(_) => true,
            CurrentActor::Animation(_) => true,
            CurrentActor::EnemyAction(_) => false,
        }
    }
}
