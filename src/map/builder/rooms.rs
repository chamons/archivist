use std::cmp::{max, min};

use adam_fov_rs::GridPoint;
use rand::Rng;

use crate::{prelude::*, util::RandExt};

pub struct RoomsMapBuilder {
    map: Map,
    rooms: Vec<Rect>,
    data: Data,
}

impl RoomsMapBuilder {
    pub fn build(rng: &mut StdRng) -> LevelState {
        let mut builder = RoomsMapBuilder {
            map: Map::new(),
            rooms: vec![],
            data: Data::load().expect("Able to load data"),
        };

        loop {
            builder.fill(TileKind::Wall);
            builder.build_random_rooms(rng, 20);
            builder.build_corridors(rng);
            if check_map_connectivity(&builder.map, builder.rooms[0].center()) {
                break;
            } else {
                builder.clear();
            }
        }

        let mut characters = builder.spawn_monsters(rng, 1);

        setup_entrance(
            &mut characters,
            &mut builder.map,
            &builder.data,
            builder.rooms[0].center(),
        );

        let items = builder.place_items();

        LevelState::new(builder.map, characters, items)
    }

    fn clear(&mut self) {
        self.map = Map::new();
        self.rooms = vec![];
    }

    fn place_items(&mut self) -> Vec<(Point, Item)> {
        let start = self.rooms[0].center();

        let farthest = self
            .rooms
            .iter()
            .max_by_key(|r| r.center().king_dist(start));

        vec![(
            farthest.expect("Find farthest room").center(),
            self.data.get_item("Runestone"),
        )]
    }

    fn fill(&mut self, tile: TileKind) {
        self.map.tiles.iter_mut().for_each(|t| {
            *t = MapTile {
                kind: tile,
                known: false,
            }
        });
    }

    fn spawn_monsters(&self, rng: &mut StdRng, difficulty: u32) -> Vec<Character> {
        let enemies = self.data.get_enemies_at_level(difficulty);
        self.rooms
            .iter()
            .skip(1)
            .map(|r| {
                let name = enemies.choose(rng).unwrap();
                let mut enemy = self.data.get_character(name);
                enemy.position = r.center();
                enemy
            })
            .collect()
    }

    fn build_random_rooms(&mut self, rng: &mut StdRng, desired_room_count: usize) {
        while self.rooms.len() < desired_room_count {
            let room = Rect::with_size(
                rng.random_range(1..SCREEN_WIDTH - 10),
                rng.random_range(1..SCREEN_HEIGHT - 10),
                rng.random_range(2..10),
                rng.random_range(2..10),
            );
            if !self.rooms.iter().any(|r| r.intersect(&room)) {
                room.for_each(|p| {
                    self.map.set(p, MapTile::floor());
                });
                self.rooms.push(room);
            }
        }
    }

    fn build_vert_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            self.map.set(Point::new(x, y), MapTile::floor());
        }
    }

    fn build_horz_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            self.map.set(Point::new(x, y), MapTile::floor());
        }
    }

    fn build_corridors(&mut self, rng: &mut StdRng) {
        let mut rooms = self.rooms.clone();

        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let previous_room_center = self.rooms[i - 1].center();
            let next_room_center = room.center();
            if rng.flip() {
                self.build_horz_tunnel(
                    previous_room_center.x,
                    next_room_center.x,
                    previous_room_center.y,
                );
                self.build_vert_tunnel(
                    previous_room_center.y,
                    next_room_center.y,
                    next_room_center.x,
                );
            } else {
                self.build_vert_tunnel(
                    previous_room_center.y,
                    next_room_center.y,
                    next_room_center.x,
                );
                self.build_horz_tunnel(
                    previous_room_center.x,
                    next_room_center.x,
                    previous_room_center.y,
                );
            }
        }
    }
}
