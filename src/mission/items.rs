use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub sprite: Point,
}

impl Item {
    pub fn render(&self, screen: &Screen, position: Point) {
        screen.draw_tiny_sprite(TileSet::Items, position, self.sprite);
    }
}
