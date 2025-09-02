use crate::prelude::*;

pub fn wander_action(level: &LevelState, id: CharacterId) -> RequestedAction {
    let enemy = level.find_character(id);
    let options = adjacent_squares(level, enemy.position);
    let mut rng = RandomNumberGenerator::new();
    let selection = rng.random_slice_entry(&options);
    match selection {
        Some(position) => RequestedAction::Move(id, *position),
        None => RequestedAction::Wait(id),
    }
}

fn adjacent_squares(level: &LevelState, point: Point) -> Vec<Point> {
    [
        Point::new(-1, 0),
        Point::new(1, 0),
        Point::new(0, -1),
        Point::new(0, 1),
    ]
    .map(|offset| offset + point)
    .into_iter()
    .filter(|p| level.character_can_enter(*p) && level.find_character_at_position(*p).is_none())
    .collect()
}
