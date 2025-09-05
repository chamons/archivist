use crate::prelude::*;

pub fn get_player_action(player: &Character, level: &LevelState) -> HandleInputResponse {
    if let Some(movement_delta) = handle_movement_key() {
        HandleInputResponse::Action(Some(handle_move_bump(
            player,
            player.position + movement_delta,
            level,
        )))
    } else if is_key_pressed(KeyCode::Period) || is_key_pressed(KeyCode::Kp5) {
        HandleInputResponse::Action(Some(RequestedAction::Wait(player.id)))
    } else if is_key_pressed(KeyCode::T) {
        HandleInputResponse::ChangeActor(CurrentActor::PlayerTargeting(TargetingInfo::new(
            player.position,
        )))
    } else if is_key_pressed(KeyCode::F1) {
        #[cfg(debug_assertions)]
        {
            HandleInputResponse::Action(Some(RequestedAction::DebugMenu(DebugRequest::Save)))
        }
        #[cfg(not(debug_assertions))]
        {
            HandleInputResponse::Action(None)
        }
    } else if is_key_pressed(KeyCode::F2) {
        #[cfg(debug_assertions)]
        {
            HandleInputResponse::Action(Some(RequestedAction::DebugMenu(DebugRequest::Load)))
        }
        #[cfg(not(debug_assertions))]
        {
            HandleInputResponse::Action(None)
        }
    } else {
        HandleInputResponse::Action(None)
    }
}
