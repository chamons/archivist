use crate::prelude::*;

mod builder;
use adam_fov_rs::compute_fov;
pub use builder::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

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
}

impl MapTile {
    pub fn floor() -> Self {
        MapTile {
            kind: TileKind::Floor,
            known: false,
        }
    }

    pub fn wall() -> Self {
        MapTile {
            kind: TileKind::Wall,
            known: false,
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
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![
                MapTile {
                    kind: TileKind::Floor,
                    known: false
                };
                NUM_TILES
            ],
        }
    }

    #[cfg(test)]
    pub fn new_filled() -> Self {
        Self {
            tiles: vec![
                MapTile {
                    kind: TileKind::Wall,
                    known: false,
                };
                NUM_TILES
            ],
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
                        let sprite_tile = match map_tile.kind {
                            // For unknown reasons world tiles are x/y flipped
                            TileKind::Wall => Point::new(1, 1),
                            TileKind::Floor => Point::new(4, 1),
                            TileKind::Exit => Point::new(8, 1),
                        };
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
    use crate::prelude::*;

    #[test]
    fn test_visibility() {
        let map = Map::new();
        let v = map.compute_visibility(Point::new(5, 5));
        for y in 6..14 {
            assert!(v.get(Point::new(5, y)));
        }
        assert!(!v.get(Point::new(5, 14)));
        assert!(!v.get(Point::new(5, 15)));
    }
}
