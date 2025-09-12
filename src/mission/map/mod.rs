use std::i32;

use crate::prelude::*;

mod builder;
use adam_fov_rs::compute_fov;
pub use builder::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MapTheme {
    Stone,
    BrownStone,
    Sand,
    GreenMetal,
    BlueMetal,
    GreyMetal,
    Rivets,
    Hedge,
    Bones,
    Clay,
}

impl MapTheme {
    pub fn random(rng: &mut RandGenerator) -> Self {
        match rng.gen_range(0, 10) {
            0 => MapTheme::Stone,
            1 => MapTheme::BrownStone,
            2 => MapTheme::Sand,
            3 => MapTheme::GreenMetal,
            4 => MapTheme::BlueMetal,
            5 => MapTheme::GreyMetal,
            6 => MapTheme::Rivets,
            7 => MapTheme::Hedge,
            8 => MapTheme::Clay,
            _ => MapTheme::Bones,
        }
    }
}

impl MapTheme {
    pub fn get_sprite(&self, tile: MapTile) -> Point {
        let mut sprite = match self {
            MapTheme::Stone => match tile.kind {
                TileKind::Wall => Point::new(1, 1),
                TileKind::Floor => Point::new(4, 1),
                TileKind::Exit => Point::new(8, 1),
            },
            MapTheme::BrownStone => match tile.kind {
                TileKind::Wall => Point::new(1, 3),
                TileKind::Floor => Point::new(4, 3),
                TileKind::Exit => Point::new(8, 3),
            },
            MapTheme::Sand => match tile.kind {
                TileKind::Wall => Point::new(1, 9),
                TileKind::Floor => Point::new(4, 9),
                TileKind::Exit => Point::new(8, 9),
            },
            MapTheme::GreenMetal => match tile.kind {
                TileKind::Wall => Point::new(1, 6),
                TileKind::Floor => Point::new(4, 6),
                TileKind::Exit => Point::new(8, 6),
            },
            MapTheme::BlueMetal => match tile.kind {
                TileKind::Wall => Point::new(1, 7),
                TileKind::Floor => Point::new(4, 7),
                TileKind::Exit => Point::new(8, 7),
            },
            MapTheme::GreyMetal => match tile.kind {
                TileKind::Wall => Point::new(1, 8),
                TileKind::Floor => Point::new(4, 8),
                TileKind::Exit => Point::new(8, 8),
            },
            MapTheme::Rivets => match tile.kind {
                TileKind::Wall => Point::new(1, 11),
                TileKind::Floor => Point::new(4, 11),
                TileKind::Exit => Point::new(8, 11),
            },
            MapTheme::Hedge => match tile.kind {
                TileKind::Wall => Point::new(1, 15),
                TileKind::Floor => Point::new(4, 15),
                TileKind::Exit => Point::new(8, 15),
            },
            MapTheme::Bones => match tile.kind {
                TileKind::Wall => Point::new(1, 16),
                TileKind::Floor => Point::new(4, 16),
                TileKind::Exit => Point::new(8, 16),
            },
            MapTheme::Clay => match tile.kind {
                TileKind::Wall => Point::new(1, 14),
                TileKind::Floor => Point::new(4, 14),
                TileKind::Exit => Point::new(8, 14),
            },
        };
        if !matches!(tile.kind, TileKind::Floor) || self.supports_floor_variations() {
            sprite.x += tile.variation;
        }
        sprite
    }

    fn supports_floor_variations(&self) -> bool {
        match self {
            MapTheme::GreenMetal => false,
            _ => true,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum TileKind {
    Wall,
    Floor,
    Exit,
}

#[derive(Debug, Serialize, Clone, Copy, Deserialize)]
pub struct MapTile {
    pub kind: TileKind,
    pub known: bool,
    pub variation: i32,
}

impl MapTile {
    pub fn floor(rng: &mut RandGenerator) -> Self {
        let variation = match rng.gen_range(0, 100) {
            i32::MIN..99 => 0,
            99..=i32::MAX => 2,
        };

        MapTile {
            kind: TileKind::Floor,
            known: false,
            variation,
        }
    }

    pub fn wall(rng: &mut RandGenerator) -> Self {
        let variation = match rng.gen_range(0, 100) {
            i32::MIN..96 => 0,
            96 | 97 => 1,
            98 | 99..=i32::MAX => 2,
        };
        MapTile {
            kind: TileKind::Wall,
            known: false,
            variation,
        }
    }

    pub fn can_enter(&self) -> bool {
        match self.kind {
            TileKind::Wall => false,
            TileKind::Floor | TileKind::Exit => true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Map {
    tiles: Vec<MapTile>,
    theme: MapTheme,
}

impl Map {
    pub fn new(theme: MapTheme) -> Self {
        Self {
            tiles: vec![
                MapTile {
                    kind: TileKind::Floor,
                    known: false,
                    variation: 0
                };
                NUM_TILES
            ],
            theme,
        }
    }

    pub fn new_filled(theme: MapTheme) -> Self {
        Self {
            tiles: vec![
                MapTile {
                    kind: TileKind::Wall,
                    known: false,
                    variation: 0
                };
                NUM_TILES
            ],
            theme,
        }
    }

    pub fn index(point: Point) -> usize {
        ((point.y * SCREEN_WIDTH) + point.x) as usize
    }

    pub fn get(&self, point: Point) -> MapTile {
        self.tiles[Self::index(point)]
    }

    pub fn set(&mut self, point: Point, tile: MapTile) {
        self.tiles[Self::index(point)] = tile;
    }

    pub fn set_known(&mut self, point: Point) {
        self.tiles[Self::index(point)].known = true
    }

    pub fn render(&self, screen: &Screen, visibility: &VisibilityMap) {
        let camera = &screen.camera;

        for y in camera.top_y..camera.bottom_y {
            for x in camera.left_x..camera.right_x {
                let position = Point::new(x, y);
                if self.in_bounds(position) {
                    let map_tile = self.get(position);
                    if map_tile.known {
                        let sprite_tile = self.theme.get_sprite(map_tile);
                        screen.draw_sprite(TileSet::World, position, sprite_tile);
                        if !visibility.get(position) {
                            screen.draw_fog(position);
                        }
                    }
                }
            }
        }
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub fn can_enter(&self, point: Point) -> bool {
        self.in_bounds(point) && self.get(point).can_enter()
    }

    pub fn compute_visibility(&self, vision_point: Point) -> VisibilityMap {
        let mut visibility = VisibilityMap::new();
        compute_fov(
            vision_point,
            VISION,
            [SCREEN_WIDTH, SCREEN_HEIGHT],
            |p| {
                let p = Point::new(p.x, p.y);
                self.in_bounds(p) && self.get(p).kind == TileKind::Wall
            },
            |p| {
                visibility.set_visible(Point::new(p.x, p.y));
            },
        );
        visibility
    }

    #[allow(dead_code)]
    pub fn dump_map_to_console(&self) {
        for x in 0..SCREEN_WIDTH {
            for y in 0..SCREEN_HEIGHT {
                match self.get(Point::new(x, y)).kind {
                    TileKind::Floor => print!("."),
                    TileKind::Wall => print!("#"),
                    TileKind::Exit => print!("<"),
                }
            }
            println!("");
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisibilityMap {
    tiles: Vec<bool>,
}

impl VisibilityMap {
    pub fn new() -> Self {
        Self {
            tiles: vec![false; NUM_TILES],
        }
    }

    pub fn get(&self, point: Point) -> bool {
        self.tiles[Map::index(point)]
    }

    pub fn set_visible(&mut self, point: Point) {
        self.tiles[Map::index(point)] = true;
    }
}

#[cfg(test)]
mod tests {
    use crate::mission::*;
    use crate::prelude::*;

    #[test]
    fn test_visibility() {
        let map = Map::new(MapTheme::Stone);
        let v = map.compute_visibility(Point::new(5, 5));
        for y in 6..14 {
            assert!(v.get(Point::new(5, y)));
        }
        assert!(!v.get(Point::new(5, 14)));
        assert!(!v.get(Point::new(5, 15)));
    }
}
