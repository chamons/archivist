use std::sync::atomic::AtomicU32;

use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
}

impl Character {
    pub fn new(position: Point, kind: CharacterKind) -> Self {
        Self {
            position,
            kind,
            id: CharacterId::next(),
        }
    }

    pub fn render(&self, screen: &mut Screen) {
        screen.set_active(ScreenLayer::Creatures);
        screen.set_sprite(self.position, self.get_glyph(screen.bounce()));
    }

    fn get_glyph(&self, bounce: bool) -> u16 {
        let base = match self.kind {
            CharacterKind::Player => 21,
            CharacterKind::Bat => 263,
            CharacterKind::Slime => 262,
            CharacterKind::Rat => 268,
            CharacterKind::Spider => 267,
        };

        // Each row is 20 wide
        if bounce { base + 20 } else { base }
    }

    pub fn is_player(&self) -> bool {
        self.kind == CharacterKind::Player
    }
}
