use std::sync::atomic::AtomicU32;

use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CharacterId(u32);

impl CharacterId {
    fn next() -> Self {
        static NEXT_ID: AtomicU32 = AtomicU32::new(1);
        CharacterId(NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterKind {
    Player,
    Slime,
    Bat,
    Rat,
    Spider,
}

#[derive(Debug)]
pub struct Character {
    pub position: Point,
    pub kind: CharacterKind,
    pub id: CharacterId,
    pub ticks: i32,
    pub health: Health,
}

impl Character {
    pub fn new(position: Point, kind: CharacterKind, max_health: i32) -> Self {
        Self {
            position,
            kind,
            id: CharacterId::next(),
            ticks: 0,
            health: Health::new(max_health),
        }
    }

    pub fn render(&self, screen: &Screen) {
        screen.draw_sprite(
            TileSet::Creatures,
            self.position,
            self.get_spite_tile(screen.camera.bounce),
        );
    }

    fn get_spite_tile(&self, bounce: bool) -> Point {
        let mut tile = match self.kind {
            CharacterKind::Player => Point::new(1, 1),
            CharacterKind::Bat => Point::new(3, 13),
            CharacterKind::Slime => Point::new(2, 13),
            CharacterKind::Rat => Point::new(8, 13),
            CharacterKind::Spider => Point::new(7, 13),
        };
        if bounce {
            tile.y += 1;
        }
        tile
    }

    pub fn is_player(&self) -> bool {
        self.kind == CharacterKind::Player
    }
}
