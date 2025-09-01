use crate::prelude::*;

pub struct Camera {
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub bottom_y: i32,
}

impl Camera {
    pub fn new(player: &Player) -> Self {
        Self {
            left_x: player.position.x - CAMERA_VIEWPORT_WIDTH / 2,
            right_x: player.position.x + CAMERA_VIEWPORT_WIDTH / 2,
            top_y: player.position.y - CAMERA_DISPLAY_HEIGHT / 2,
            bottom_y: player.position.y + CAMERA_DISPLAY_HEIGHT / 2,
        }
    }
}
