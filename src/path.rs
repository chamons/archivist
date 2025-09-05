use crate::prelude::*;

use pathfinding::prelude::bfs;

pub fn can_see_player(level: &LevelState, id: CharacterId) -> bool {
    let enemy = level.find_character(id).position;
    let player = level.get_player().position;
    level.map.compute_visibility(enemy).get(player)
}

pub fn distance_to_player(level: &LevelState, id: CharacterId) -> Option<usize> {
    let enemy = level.find_character(id).position;
    let player = level.get_player().position;

    bfs(
        &enemy,
        |p| adjacent_squares(level, *p, PathCharacterOptions::AllowEmptyOrPlayer),
        |p| *p == player,
    )
    .map(|path| path.iter().len())
}

#[derive(Debug, Copy, Clone)]
pub enum PathCharacterOptions {
    AllowEmptyOrPlayer,
    AllowEmptyOrEnemies,
    All,
}

pub fn path_between_points(
    start: Point,
    end: Point,
    level: &LevelState,
    options: PathCharacterOptions,
) -> Option<Vec<Point>> {
    bfs(
        &start,
        |p| adjacent_squares(level, *p, options),
        |p| *p == end,
    )
}

pub fn adjacent_squares(
    level: &LevelState,
    point: Point,
    options: PathCharacterOptions,
) -> Vec<Point> {
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
        let acceptable_occupants = match level.find_character_at_position(*p) {
            Some(c) => match options {
                PathCharacterOptions::AllowEmptyOrPlayer => c.is_player(),
                PathCharacterOptions::AllowEmptyOrEnemies => !c.is_player(),
                PathCharacterOptions::All => true,
            },
            None => true,
        };
        can_enter && acceptable_occupants
    })
    .collect()
}
