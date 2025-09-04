use crate::prelude::*;

use pathfinding::prelude::bfs;

pub fn default_action(level: &LevelState, id: CharacterId) -> RequestedAction {
    // Later will be LOS
    if let Some(distance) = distance_to_player(level, id)
        && distance < 8
    {
        chase_attack_player(level, id)
    } else {
        wander_action(level, id)
    }
}

pub fn distance_to_player(level: &LevelState, id: CharacterId) -> Option<usize> {
    let enemy = level.find_character(id).position;
    let player = level.get_player().position;

    bfs(&enemy, |p| adjacent_squares(level, *p), |p| *p == player).map(|path| path.iter().len())
}

pub fn chase_attack_player(level: &LevelState, id: CharacterId) -> RequestedAction {
    let enemy = level.find_character(id).position;
    let player = level.get_player().position;

    let path = bfs(&enemy, |p| adjacent_squares(level, *p), |p| *p == player);
    if let Some(path) = path {
        // First position on path is current
        RequestedAction::Move(id, path[1])
    } else {
        wander_action(level, id)
    }
}

pub fn wander_action(level: &LevelState, id: CharacterId) -> RequestedAction {
    let enemy = level.find_character(id);
    let options = adjacent_squares(level, enemy.position);
    let selection = options.choose(&mut ::rand::rng());
    match selection {
        Some(position) => RequestedAction::Move(id, *position),
        None => RequestedAction::Wait(id),
    }
}

fn adjacent_squares(level: &LevelState, point: Point) -> Vec<Point> {
    [
        Point::new(-1, 0),
        Point::new(-1, 1),
        Point::new(-1, -1),
        Point::new(1, 0),
        Point::new(1, 1),
        Point::new(1, -1),
        Point::new(0, -1),
        Point::new(0, 1),
    ]
    .map(|offset| offset + point)
    .into_iter()
    .filter(|p| {
        let can_enter = level.character_can_enter(*p);
        let empty_or_enemy = match level.find_character_at_position(*p) {
            Some(c) => c.is_player(),
            None => true,
        };
        can_enter && empty_or_enemy
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    fn create_test_map() -> (CharacterId, LevelState) {
        let data = Data::load().unwrap();

        let mut player = data.get_character("Player");
        player.position = Point::new(1, 1);

        let mut bat = data.get_character("Bat");
        bat.position = Point::new(1, 5);
        let id = bat.id;

        let mut map = Map::new_filled();
        for y in 1..6 {
            map.set(
                Point::new(1, y),
                MapTile {
                    kind: TileKind::Floor,
                    known: true,
                },
            );
        }
        let level = LevelState::new(map, vec![player, bat]);
        (id, level)
    }

    #[test]
    fn distance() {
        let (id, level) = create_test_map();

        assert_eq!(Some(5), distance_to_player(&level, id));
    }

    #[test]
    fn chases_player() {
        let (id, level) = create_test_map();

        let action = chase_attack_player(&level, id);
        assert_eq!(action, RequestedAction::Move(id, Point::new(1, 4)));
    }
}
