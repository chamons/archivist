use rand::Rng;

use crate::mission::*;
use crate::prelude::*;

pub struct CellsMapBuilder {
    map: Map,
    data: Data,
}

impl CellsMapBuilder {
    pub fn build(rng: &mut StdRng) -> LevelState {
        let mut builder = CellsMapBuilder {
            map: Map::new(rng.random()),
            data: Data::load().expect("Able to load data"),
        };

        loop {
            builder.randomize_map(rng);
            for _ in 0..10 {
                builder.iterate();
            }
            fix_map_border(&mut builder.map);

            let center = find_map_center(&builder.map);
            if check_map_connectivity(&builder.map, center) {
                break;
            } else {
                builder.clear();
            }
        }

        let center = find_map_center(&builder.map);

        let mut characters =
            spawn_monster_randomly(rng, &builder.map, 50, center, 1, &builder.data);
        setup_entrance(&mut characters, &mut builder.map, &builder.data, center);

        let items = spawn_rune_far_away(&builder.map, center, &builder.data);

        LevelState::new(builder.map, characters, items)
    }

    fn clear(&mut self) {
        self.map = Map::new(self.map.theme);
    }

    fn iterate(&mut self) {
        let mut next_map = self.map.clone();
        for x in 1..SCREEN_WIDTH - 1 {
            for y in 1..SCREEN_HEIGHT - 1 {
                let position = Point::new(x, y);
                let neighbors = Self::neighbors(&self, position);
                if neighbors > 4 || neighbors == 0 {
                    next_map.set(position, MapTile::wall());
                } else {
                    next_map.set(position, MapTile::floor());
                }
            }
        }
        self.map = next_map;
    }

    fn neighbors(&self, position: Point) -> usize {
        position
            .adjacent()
            .into_iter()
            .filter(|p| self.map.get(*p).kind == TileKind::Wall)
            .count()
    }

    fn randomize_map(&mut self, rng: &mut StdRng) {
        for tile in self.map.tiles.iter_mut() {
            if rng.random_range(0..100) > 55 {
                tile.kind = TileKind::Floor;
            } else {
                tile.kind = TileKind::Wall;
            }
        }
    }
}
