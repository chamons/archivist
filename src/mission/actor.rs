use crate::mission::*;
use crate::prelude::*;

// In a turn based game, only sometimes does the player get to move
// This contains what the current "thing takes it's turn" is
// which could be an animation for example
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CurrentActor {
    PlayerStandardAction,
    PlayerTargeting(TargetingInfo),
    EnemyAction(CharacterId),
    Animation(AnimationInfo),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HandleInputResponse {
    Action(Option<RequestedAction>),
    ChangeActor(CurrentActor),
}

impl CurrentActor {
    pub fn act(&mut self, level: &mut LevelState, screen: &mut Screen) -> Option<RequestedAction> {
        match self {
            CurrentActor::PlayerStandardAction => {
                let player = level.get_player();
                let action = self.process_input_response(get_player_action(player, level, screen));
                // When the player makes a real action clear the per-turn log
                if action.is_some() {
                    level.turn_log.clear();
                }
                action
            }
            CurrentActor::PlayerTargeting(targeting_info) => {
                let is_current_target_valid = Self::is_current_target_valid(targeting_info, level);
                let response = targeting_info.handle_input(level, screen, is_current_target_valid);
                self.process_input_response(response)
            }
            CurrentActor::EnemyAction(id) => {
                let response = default_ai_action(level, *id);
                self.process_input_response(response)
            }
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

    pub fn render(&mut self, screen: &Screen, level: &LevelState) {
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
        let valid_target =
            character_target_target.is_some() && !character_target_target.unwrap().is_player();

        let within_distance = clear_line_between(
            level,
            targeting_info.source_position,
            targeting_info.position,
            targeting_info.max_range,
        );

        valid_target && within_distance
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
