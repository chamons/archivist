use crate::prelude::*;

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
}

impl Character {
    pub fn new(position: Point, kind: CharacterKind) -> Self {
        Self { position, kind }
    }

    pub fn render(&self, screen: &mut Screen) {
        screen.set_active(ScreenLayer::Creatures);
        screen.set_sprite(self.position, self.get_glyph(screen.bounce()));
    }

    fn get_glyph(&self, bounce: bool) -> u16 {
        let base = match self.kind {
            CharacterKind::Player => 21,
            CharacterKind::Slime => 263,
            CharacterKind::Bat => 265,
            CharacterKind::Rat => 273,
            CharacterKind::Spider => 275,
        };

        // Each row is 20 wide
        if bounce { base + 20 } else { base }
    }
}
