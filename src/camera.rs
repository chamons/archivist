use crate::prelude::*;

#[derive(Debug)]
pub struct Camera {
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub bottom_y: i32,
    pub bounce: bool,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            left_x: 0,
            right_x: 0,
            top_y: 0,
            bottom_y: 0,
            bounce: false,
        }
    }

    pub fn update(&mut self, player_position: Point, frame: usize) -> bool {
        self.left_x = player_position.x - CAMERA_VIEWPORT_WIDTH / 2;
        self.right_x = player_position.x + CAMERA_VIEWPORT_WIDTH / 2;
        self.top_y = player_position.y - CAMERA_DISPLAY_HEIGHT / 2;
        self.bottom_y = player_position.y + CAMERA_DISPLAY_HEIGHT / 2;
        if frame % BOUNCE_FRAME == 0 {
            self.bounce = !self.bounce;
            true
        } else {
            false
        }
    }

    pub fn is_in_view(&self, point: Point) -> bool {
        let viewport = Rect {
            x1: self.left_x,
            x2: self.right_x,
            y1: self.top_y,
            y2: self.bottom_y,
        };
        viewport.point_in_rect(point)
    }
}
