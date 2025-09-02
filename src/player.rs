use crate::prelude::*;

pub fn get_player_action(player: &Character, ctx: &mut BTerm) -> Option<RequestedAction> {
    if let Some(key) = ctx.key {
        match key {
            VirtualKeyCode::Left => {
                Some(RequestedAction::Move(player.position + Point::new(-1, 0)))
            }
            VirtualKeyCode::Right => {
                Some(RequestedAction::Move(player.position + (Point::new(1, 0))))
            }
            VirtualKeyCode::Up => Some(RequestedAction::Move(player.position + Point::new(0, -1))),
            VirtualKeyCode::Down => Some(RequestedAction::Move(player.position + Point::new(0, 1))),
            _ => None,
        }
    } else {
        None
    }
}
