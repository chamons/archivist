use crate::prelude::*;

mod builder;
pub use builder::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TileKind {
    Wall,
    Floor,
}

#[derive(Debug)]
pub struct Map {
    tiles: Vec<TileKind>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileKind::Floor; NUM_TILES],
        }
    }

    fn index(point: Point) -> usize {
        ((point.y * SCREEN_WIDTH) + point.x) as usize
    }

    pub fn get(&self, point: Point) -> TileKind {
        self.tiles[Self::index(point)]
    }

    pub fn set(&mut self, point: Point, tile: TileKind) {
        self.tiles[Self::index(point)] = tile;
    }

    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                match self.get(Point::new(x, y)) {
                    TileKind::Floor => ctx.set(x, y, YELLOW, BLACK, to_cp437('.')),
                    TileKind::Wall => ctx.set(x, y, GREEN, BLACK, to_cp437('#')),
                }
            }
        }
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub fn can_enter(&self, point: Point) -> bool {
        self.in_bounds(point) && self.get(point) == TileKind::Floor
    }
}
