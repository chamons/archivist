pub mod campaign;
mod flow;
pub mod mission;
pub mod screens;
mod util;

pub mod prelude {
    pub use crate::flow::*;
    pub use crate::util::*;

    pub use rand::RngCore;
    pub use rand::SeedableRng;
    pub use rand::rngs::StdRng;
    pub use rand::seq::IndexedRandom;

    pub use macroquad::color::*;
    pub use macroquad::input::{KeyCode, is_key_pressed};
    pub use macroquad::math::Rect as MRect;

    pub use serde::{Deserialize, Serialize};

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;

    pub const CAMERA_VIEWPORT_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const CAMERA_DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;

    pub const SPRITE_SIZE: usize = 24;

    pub const BOUNCE_FRAME: usize = 60;
    pub const TARGET_FRAME_BLINK: usize = 160;
    pub const TARGET_FRAME_PAUSE_WINDOW: usize = 35;

    pub const TICKS_TO_ACT: i32 = 100;
    pub const TICKS_MOVEMENT: i32 = 100;

    pub const TICKS_FLOATING_TEXT: u32 = 120;

    pub const ANIMATION_TICKS_PER_TILE: usize = 6;

    pub const REST_HEALTH_PERCENTAGE: f32 = 1.0;
    pub const REST_WILL_PERCENTAGE: f32 = 1.0;

    pub const DRUNK_STAGGER_DISTANCE: u32 = 400;
    pub const DRUNK_DESIRED_FLOOR_AMOUNT: i32 = (SCREEN_WIDTH * SCREEN_HEIGHT) / 3;

    pub const VISION: usize = 8;

    pub const MISSIONS_TO_VICTORY: u32 = 3;

    pub const STATUS_EFFECT_MIGHT_DAMAGE_BOOST: i32 = 1;
}
