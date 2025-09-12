use crate::mission::enemy_set::get_enemy_set_for_difficulty;
use crate::mission::*;
use crate::prelude::*;

use adam_fov_rs::GridPoint;

mod utils;
use macroquad::rand::ChooseRandom;
pub use utils::*;

mod rooms;
pub use rooms::*;

mod cells;
pub use cells::*;

mod drunk_digger;
pub use drunk_digger::*;

pub mod enemy_set;

pub fn generate_random_map(player: Character, difficulty: u32) -> LevelState {
    let mut rng = RandGenerator::new();
    let seed = macroquad::miniquad::date::now() as u64;

    if cfg!(debug_assertions) {
        println!("Generating map with seed {seed}");
    }

    rng.srand(seed);

    let level = match rng.gen_range(0, 3) {
        0 => RoomsMapBuilder::build(&mut rng, difficulty, player),
        1 => CellsMapBuilder::build(&mut rng, difficulty, player),
        _ => DrunkDigger::build(&mut rng, difficulty, player),
    };

    // level.map.dump_map_to_console();
    level
}

pub fn setup_entrance(
    mut player: Character,
    characters: &mut Vec<Character>,
    map: &mut Map,
    center: Point,
) {
    player.position = center;
    characters.push(player);

    map.set(
        center,
        MapTile {
            kind: TileKind::Exit,
            known: true,
            variation: 0,
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
    rng: &mut RandGenerator,
    map: &Map,
    count: usize,
    center: Point,
    difficulty: u32,
    data: &Data,
) -> Vec<Character> {
    let enemies = get_enemy_set_for_difficulty(&data, difficulty);

    let floors = find_all_floors(map);
    floors
        .into_iter()
        .filter(|f| f.king_dist(center) > 10)
        .collect::<Vec<_>>()
        .choose_multiple_with_state(rng, count)
        .map(|position| {
            let name = enemies.choose_with_state(rng).unwrap();
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

pub fn fix_map_border(map: &mut Map, rng: &mut RandGenerator) {
    for x in 0..SCREEN_WIDTH {
        map.set(Point::new(x, 0), MapTile::wall(rng));
        map.set(Point::new(x, SCREEN_HEIGHT - 1), MapTile::wall(rng));
    }
    for y in 0..SCREEN_HEIGHT {
        map.set(Point::new(0, y), MapTile::wall(rng));
        map.set(Point::new(SCREEN_WIDTH - 1, y), MapTile::wall(rng));
    }
}

pub fn find_map_center(map: &Map) -> Point {
    let center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
    let floors = find_all_floors(map);
    floors
        .into_iter()
        .min_by_key(|p| p.king_dist(center))
        .expect("One should be closest")
}
