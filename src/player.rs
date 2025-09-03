use crate::prelude::*;

pub fn get_player_action(player: &Character) -> Option<RequestedAction> {
    if is_key_pressed(KeyCode::Left) {
        Some(RequestedAction::Move(
            player.id,
            player.position + Point::new(-1, 0),
        ))
    } else if is_key_pressed(KeyCode::Right) {
        Some(RequestedAction::Move(
            player.id,
            player.position + (Point::new(1, 0)),
        ))
    } else if is_key_pressed(KeyCode::Up) {
        Some(RequestedAction::Move(
            player.id,
            player.position + Point::new(0, -1),
        ))
    } else if is_key_pressed(KeyCode::Down) {
        Some(RequestedAction::Move(
            player.id,
            player.position + Point::new(0, 1),
        ))
    } else if is_key_pressed(KeyCode::Period) {
        Some(RequestedAction::Wait(player.id))
    } else {
        None
    }
}
