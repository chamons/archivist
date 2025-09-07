use rand::Rng;

use crate::prelude::*;

pub struct DrunkDigger {
    map: Map,
    data: Data,
}

impl DrunkDigger {
    pub fn build(rng: &mut StdRng) -> LevelState {
        let mut builder = DrunkDigger {
            map: Map::new_filled(rng.random()),
            data: Data::load().expect("Able to load data"),
        };

        builder.dig(rng);
        fix_map_border(&mut builder.map);

        let center = find_map_center(&builder.map);

        let mut characters =
            spawn_monster_randomly(rng, &builder.map, 50, center, 1, &builder.data);
        setup_entrance(&mut characters, &mut builder.map, &builder.data, center);

        let items = spawn_rune_far_away(&builder.map, center, &builder.data);

        LevelState::new(builder.map, characters, items)
    }

    fn dig(&mut self, rng: &mut StdRng) {
        let start = Point::new(
            rng.random_range(0..SCREEN_WIDTH),
            rng.random_range(0..SCREEN_HEIGHT),
        );
        self.stagger(start, rng);

        loop {
            let floors = find_all_floors(&self.map);
            if floors.len() >= DRUNK_DESIRED_FLOOR_AMOUNT as usize {
                break;
            }

            self.stagger(*floors.choose(rng).expect("At least one floor"), rng);
        }
    }

    fn stagger(&mut self, start: Point, rng: &mut StdRng) {
        let mut position = start;
        let mut distance = 0;

        loop {
            self.map.set(position, MapTile::floor());
            match rng.random_range(0..4) {
                0 => position.x -= 1,
                1 => position.x += 1,
                2 => position.y -= 1,
                _ => position.y += 1,
            }
            if !self.map.in_bounds(position) {
                break;
            }
            distance += 1;
            if distance > DRUNK_STAGGER_DISTANCE {
                break;
            }
        }
    }
}
