use crate::prelude::*;

#[derive(Debug)]
pub struct Player {
    pub character: Character,
}

impl Player {
    pub fn new(position: Point) -> Self {
        Self {
            character: Character::new(position, CharacterKind::Player),
        }
    }

    pub fn update(&mut self, ctx: &mut BTerm, map: &Map) -> bool {
        if let Some(key) = ctx.key {
            let delta = match key {
                VirtualKeyCode::Left => Some(Point::new(-1, 0)),
                VirtualKeyCode::Right => Some(Point::new(1, 0)),
                VirtualKeyCode::Up => Some(Point::new(0, -1)),
                VirtualKeyCode::Down => Some(Point::new(0, 1)),
                _ => None,
            };

            if let Some(delta) = delta {
                let new_position = self.character.position + delta;
                if map.can_enter(new_position) {
                    self.character.position = new_position;
                    return true;
                }
            }
        }
        false
    }
}
