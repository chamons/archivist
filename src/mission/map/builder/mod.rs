use crate::mission::enemy_set::get_enemy_set_for_difficulty;
use crate::mission::*;
use crate::prelude::*;

use adam_fov_rs::GridPoint;
use log::debug;
use rand::prelude::*;

mod utils;
pub use utils::*;

mod rooms;
pub use rooms::*;

mod cells;
pub use cells::*;

mod drunk_digger;
pub use drunk_digger::*;

pub mod enemy_set;

pub fn generate_random_map(player: Character, difficulty: u32) -> LevelState {
    let seed = rand::rng().next_u64();
    let mut rng = StdRng::seed_from_u64(seed);
    debug!("Generating map with seed {seed}");

    let level = match rng.random_range(0..3) {
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
    let enemies = get_enemy_set_for_difficulty(&data, difficulty);

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
        .min_by_key(|f| f.king_dist(center))
        .expect("One should be closest");

    vec![(farthest, data.get_item("Runestone"))]
}

pub fn fix_map_border(map: &mut Map) {
    for x in 0..SCREEN_WIDTH {
        map.set(Point::new(x, 0), MapTile::wall());
        map.set(Point::new(x, SCREEN_HEIGHT - 1), MapTile::wall());
    }
    for y in 0..SCREEN_HEIGHT {
        map.set(Point::new(0, y), MapTile::wall());
        map.set(Point::new(SCREEN_WIDTH - 1, y), MapTile::wall());
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
