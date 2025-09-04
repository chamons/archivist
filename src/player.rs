use crate::prelude::*;

pub fn get_player_action(player: &Character) -> Option<RequestedAction> {
    if let Some(movement_delta) = handle_movement_key() {
        Some(RequestedAction::Move(
            player.id,
            player.position + movement_delta,
        ))
    } else if is_key_pressed(KeyCode::Period) || is_key_pressed(KeyCode::Kp5) {
        Some(RequestedAction::Wait(player.id))
    } else if is_key_pressed(KeyCode::T) {
        Some(RequestedAction::PlayerTargeting)
    } else if is_key_pressed(KeyCode::F1) {
        #[cfg(debug_assertions)]
        {
            Some(RequestedAction::DebugMenu(DebugRequest::Save))
        }
        #[cfg(not(debug_assertions))]
        {
            None
        }
    } else if is_key_pressed(KeyCode::F2) {
        #[cfg(debug_assertions)]
        {
            Some(RequestedAction::DebugMenu(DebugRequest::Load))
        }
        #[cfg(not(debug_assertions))]
        {
            None
        }
    } else {
        None
    }
}
