use adam_fov_rs::GridPoint;
use rand::Rng;

use crate::prelude::*;

pub struct CellsMapBuilder {
    map: Map,
    data: Data,
}

impl CellsMapBuilder {
    pub fn build(rng: &mut StdRng) -> LevelState {
        let mut builder = CellsMapBuilder {
            map: Map::new(),
            data: Data::load().expect("Able to load data"),
        };

        builder.randomize_map(rng);
        for _ in 0..10 {
            builder.iterate();
        }
        builder.fix_border();

        let center = builder.find_center();

        let mut characters =
            spawn_monster_randomly(rng, &builder.map, 50, center, 1, &builder.data);
        setup_entrance(&mut characters, &mut builder.map, &builder.data, center);

        let items = spawn_rune_far_away(&builder.map, center, &builder.data);

        LevelState::new(builder.map, characters, items)
    }

    fn fix_border(&mut self) {
        for x in 0..SCREEN_WIDTH {
            self.map.set(Point::new(x, 0), MapTile::wall());
            self.map
                .set(Point::new(x, SCREEN_HEIGHT - 1), MapTile::wall());
        }
        for y in 0..SCREEN_HEIGHT {
            self.map.set(Point::new(0, y), MapTile::wall());
            self.map
                .set(Point::new(SCREEN_WIDTH - 1, y), MapTile::wall());
        }
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

    fn find_center(&self) -> Point {
        let center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        let floors = find_all_floors(&self.map);
        floors
            .into_iter()
            .min_by_key(|p| p.king_dist(center))
            .expect("One should be closest")
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
