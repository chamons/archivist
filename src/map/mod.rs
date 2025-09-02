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

    pub fn render(&self, screen: &mut Screen) {
        let tile_color = ColorPair::new(WHITE, BLACK);

        let mut draw_batch = DrawBatch::new();
        draw_batch.target(ScreenLayer::World.into());

        let camera = &screen.camera;

        for y in camera.top_y..camera.bottom_y {
            for x in camera.left_x..camera.right_x {
                let position = Point::new(x, y);
                if self.in_bounds(position) {
                    let draw_position = Point::new(x - camera.left_x, y - camera.top_y);
                    let tile = match self.get(position) {
                        TileKind::Wall => 1,
                        TileKind::Floor => 4,
                    };
                    draw_batch.set(draw_position, tile_color, tile);
                }
            }
        }

        draw_batch.submit(0).expect("Batch error");

        render_draw_buffer(screen.ctx).expect("Render error");
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub fn can_enter(&self, point: Point) -> bool {
        self.in_bounds(point) && self.get(point) == TileKind::Floor
    }
}
