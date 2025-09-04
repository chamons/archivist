use crate::prelude::*;

pub fn get_player_action(player: &Character) -> Option<RequestedAction> {
    if is_key_pressed(KeyCode::Left) | is_key_pressed(KeyCode::Kp4) {
        Some(RequestedAction::Move(
            player.id,
            player.position + Point::new(-1, 0),
        ))
    } else if is_key_pressed(KeyCode::Right) | is_key_pressed(KeyCode::Kp6) {
        Some(RequestedAction::Move(
            player.id,
            player.position + (Point::new(1, 0)),
        ))
    } else if is_key_pressed(KeyCode::Up) | is_key_pressed(KeyCode::Kp8) {
        Some(RequestedAction::Move(
            player.id,
            player.position + Point::new(0, -1),
        ))
    } else if is_key_pressed(KeyCode::Down) | is_key_pressed(KeyCode::Kp2) {
        Some(RequestedAction::Move(
            player.id,
            player.position + Point::new(0, 1),
        ))
    } else if is_key_pressed(KeyCode::Kp1) {
        Some(RequestedAction::Move(
            player.id,
            player.position + Point::new(-1, 1),
        ))
    } else if is_key_pressed(KeyCode::Kp3) {
        Some(RequestedAction::Move(
            player.id,
            player.position + Point::new(1, 1),
        ))
    } else if is_key_pressed(KeyCode::Kp7) {
        Some(RequestedAction::Move(
            player.id,
            player.position + Point::new(-1, -1),
        ))
    } else if is_key_pressed(KeyCode::Kp9) {
        Some(RequestedAction::Move(
            player.id,
            player.position + Point::new(1, -1),
        ))
    } else if is_key_pressed(KeyCode::Period) || is_key_pressed(KeyCode::Kp5) {
        Some(RequestedAction::Wait(player.id))
    } else {
        None
    }
}
