use crate::prelude::*;

#[derive(Debug)]
pub struct Player {
    pub position: Point,
}

impl Player {
    pub fn new(position: Point) -> Self {
        Self { position }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set(
            self.position.x,
            self.position.y,
            WHITE,
            BLACK,
            to_cp437('@'),
        );
    }

    pub fn update(&mut self, ctx: &mut BTerm, map: &Map) {
        if let Some(key) = ctx.key {
            let delta = match key {
                VirtualKeyCode::Left => Some(Point::new(-1, 0)),
                VirtualKeyCode::Right => Some(Point::new(1, 0)),
                VirtualKeyCode::Up => Some(Point::new(0, -1)),
                VirtualKeyCode::Down => Some(Point::new(0, 1)),
                _ => None,
            };

            if let Some(delta) = delta {
                let new_position = self.position + delta;
                if map.can_enter(new_position) {
                    self.position = new_position;
                }
            }
        }
    }
}
