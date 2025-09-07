mod utils;
pub use utils::*;

mod rooms;
pub use rooms::*;

mod cells;
pub use cells::*;

use crate::prelude::*;
use adam_fov_rs::GridPoint;
use rand::prelude::IteratorRandom;
use rand::rngs::StdRng;

pub fn setup_entrance(characters: &mut Vec<Character>, map: &mut Map, data: &Data, center: Point) {
    let mut player = data.get_character("Player");
    player.position = center;
    characters.push(player);

    map.set(
        center,
        MapTile {
            kind: TileKind::Exit,
            known: true,
        },
    );
}

pub fn find_all_floors(map: &Map) -> Vec<Point> {
    let mut floors = vec![];
    for x in 0..SCREEN_WIDTH {
        for y in 0..SCREEN_HEIGHT {
            let position = Point::new(x, y);
            if map.get(position).kind == TileKind::Floor {
                floors.push(position);
            }
        }
    }
    floors
}

pub fn spawn_monster_randomly(
    rng: &mut StdRng,
    map: &Map,
    count: usize,
    center: Point,
    difficulty: u32,
    data: &Data,
) -> Vec<Character> {
    let enemies = data.get_enemies_at_level(difficulty);

    let floors = find_all_floors(map);
    floors
        .into_iter()
        .filter(|f| f.king_dist(center) > 10)
        .choose_multiple(rng, count)
        .iter()
        .map(|position| {
            let name = enemies.choose(rng).unwrap();
            let mut enemy = data.get_character(name);
            enemy.position = *position;
            enemy
        })
        .collect()
}

pub fn spawn_rune_far_away(map: &Map, center: Point, data: &Data) -> Vec<(Point, Item)> {
    let floors = find_all_floors(map);

    let farthest = floors
        .into_iter()
        .max_by_key(|f| f.king_dist(center))
        .expect("One should be closest");

    vec![(farthest, data.get_item("Runestone"))]
}
