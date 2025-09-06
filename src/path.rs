use crate::prelude::*;

use bresenham::Bresenham;
use pathfinding::prelude::bfs;

pub fn clear_line_between(
    level: &LevelState,
    first: Point,
    second: Point,
    max_length: u32,
) -> bool {
    let first = (first.x as isize, first.y as isize);
    let second = (second.x as isize, second.y as isize);
    let path: Vec<_> = Bresenham::new(first, second).collect();

    let path_clear = path.iter().all(|p| {
        *p == first
            || *p == second
            || level
                .find_character_at_position(Point::new(p.0 as i32, p.1 as i32))
                .is_none()
    });

    let short_enough = path.len() < max_length as usize;

    path_clear && short_enough
}

pub fn can_see(level: &LevelState, first: CharacterId, second: CharacterId) -> bool {
    let first = level.find_character(first).position;
    let second = level.find_character(second).position;
    level.map.compute_visibility(first).get(second)
}

pub fn can_see_player(level: &LevelState, id: CharacterId) -> bool {
    can_see(level, id, level.get_player().id)
}

pub fn path_distance_between(
    level: &LevelState,
    first: Point,
    second: Point,
    options: PathCharacterOptions,
) -> Option<usize> {
    bfs(
        &first,
        |p| adjacent_squares(level, *p, options),
        |p| *p == second,
    )
    .map(|path| path.iter().len())
}

pub fn distance_to_player(level: &LevelState, id: CharacterId) -> Option<usize> {
    path_distance_between(
        level,
        level.find_character(id).position,
        level.get_player().position,
        PathCharacterOptions::AllowEmptyOrPlayer,
    )
}

#[derive(Debug, Copy, Clone)]
pub enum PathCharacterOptions {
    AllowEmptyOrPlayer,
    AllowEmptyOrEnemies,
    All,
    AllCharactersBlock,
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
        Point::new(1, 0),
        Point::new(0, -1),
        Point::new(0, 1),
        Point::new(-1, 1),
        Point::new(-1, -1),
        Point::new(1, 1),
        Point::new(1, -1),
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
                PathCharacterOptions::AllCharactersBlock => false,
            },
            None => true,
        };
        can_enter && acceptable_occupants
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn distance() {
        let (id, level) = create_test_map();

        assert_eq!(Some(5), distance_to_player(&level, id));
    }

    #[test]
    fn clear_line_between_point() {
        let (_, level) = create_test_map();

        assert!(clear_line_between(
            &level,
            Point::new(1, 1),
            Point::new(1, 5),
            6
        ));
    }
}
