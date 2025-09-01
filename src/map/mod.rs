use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TileKind {
    Wall,
    Floor,
}

pub struct Map {
    tiles: Vec<TileKind>,
}

impl Map {
    pub fn new() -> Map {
        Self {
            tiles: vec![TileKind::Floor; NUM_TILES],
        }
    }

    fn index(x: u32, y: u32) -> usize {
        ((y * SCREEN_WIDTH) + x) as usize
    }

    pub fn get(&self, x: u32, y: u32) -> TileKind {
        self.tiles[Self::index(x, y)]
    }

    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                match self.get(x, y) {
                    TileKind::Wall => ctx.set(x, y, YELLOW, BLACK, to_cp437('.')),
                    TileKind::Floor => ctx.set(x, y, GREEN, BLACK, to_cp437('#')),
                }
            }
        }
    }
}
