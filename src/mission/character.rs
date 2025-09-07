use std::sync::atomic::AtomicU32;

use macroquad::shapes::draw_rectangle;

use crate::mission::*;
use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CharacterId(u32);

impl CharacterId {
    pub fn next() -> Self {
        static NEXT_ID: AtomicU32 = AtomicU32::new(1);
        CharacterId(NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub position: Point,
    pub id: CharacterId,
    pub ticks: i32,
    pub health: Health,
    pub will: Will,
    pub base_sprite_tile: Point,
    pub weapon: Weapon,
    pub skills: Vec<Skill>,
    pub carried_items: Vec<Item>,
}

impl Character {
    pub fn render(&self, screen: &Screen) {
        screen.draw_sprite(
            TileSet::Creatures,
            self.position,
            self.get_spite_tile(screen.camera.bounce),
        );

        let health_percentage = self.health.percentage();
        if !self.is_player() && health_percentage < 1.0 {
            draw_rectangle(
                (self.position.x - screen.camera.left_x) as f32 * 24.0,
                (self.position.y - screen.camera.top_y) as f32 * 24.0,
                24.0 * health_percentage,
                4.0,
                color_for_health(health_percentage),
            );
        }
    }

    fn get_spite_tile(&self, bounce: bool) -> Point {
        let mut tile = self.base_sprite_tile.clone();
        if bounce {
            tile.y += 1;
        }
        tile
    }

    pub fn is_player(&self) -> bool {
        self.name == "Player"
    }
}
