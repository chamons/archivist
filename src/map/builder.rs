use std::cmp::{max, min};

use crate::{prelude::*, util::RandExt};

pub struct MapBuilder {
    map: Map,
    rooms: Vec<Rect>,
}

impl MapBuilder {
    pub fn build(rng: &mut RandomNumberGenerator) -> (Map, Point) {
        let mut builder = MapBuilder {
            map: Map::new(),
            rooms: vec![],
        };

        builder.fill(TileKind::Wall);
        builder.build_random_rooms(rng, 20);
        builder.build_corridors(rng);
        (builder.map, builder.rooms[0].center())
    }

    fn fill(&mut self, tile: TileKind) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator, desired_room_count: usize) {
        while self.rooms.len() < desired_room_count {
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGHT - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );
            if !self.rooms.iter().any(|r| r.intersect(&room)) {
                room.for_each(|p| {
                    self.map.set(p, TileKind::Floor);
                });
                self.rooms.push(room);
            }
        }
    }

    fn build_vert_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            self.map.set(Point::new(x, y), TileKind::Floor);
        }
    }

    fn build_horz_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            self.map.set(Point::new(x, y), TileKind::Floor);
        }
    }

    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
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
