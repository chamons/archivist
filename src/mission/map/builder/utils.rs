use std::collections::HashSet;

use crate::mission::*;
use crate::prelude::*;

pub fn check_map_connectivity(map: &Map, start: Point) -> bool {
    let mut visited = HashSet::new();
    let mut to_visit = vec![start];

    while let Some(next) = to_visit.pop() {
        let new = visited.insert(next);
        if new {
            for adj in next.adjacent() {
                if map.in_bounds(adj) && map.get(adj).can_enter() {
                    to_visit.push(adj);
                }
            }
        }
    }

    for x in 0..SCREEN_WIDTH {
        for y in 0..SCREEN_HEIGHT {
            let current = Point::new(x, y);
            if map.get(current).can_enter() {
                if !visited.contains(&current) {
                    return false;
                }
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::mission::*;
    use crate::prelude::*;

    #[test]
    fn check_connectivity() {
        let mut map = Map::new_filled(MapTheme::Stone);

        let mut rng = RandGenerator::new();
        map.set(Point::new(1, 1), MapTile::floor(&mut rng));
        map.set(Point::new(1, 2), MapTile::floor(&mut rng));
        map.set(Point::new(1, 3), MapTile::floor(&mut rng));

        assert!(check_map_connectivity(&map, Point::new(1, 1)));

        map.set(Point::new(1, 5), MapTile::floor(&mut rng));
        assert!(!check_map_connectivity(&map, Point::new(1, 1)));
    }
}
