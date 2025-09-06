mod actor;
mod ai;
mod animation;
mod camera;
mod character;
mod damage;
mod data;
mod engine;
mod health;
mod items;
mod map;
mod path;
mod player;
mod screen;
mod skills;
mod state;
mod targeting;
mod util;

pub mod prelude {
    pub use crate::actor::*;
    pub use crate::ai::*;
    pub use crate::animation::*;
    pub use crate::camera::*;
    pub use crate::character::*;
    pub use crate::damage::*;
    pub use crate::data::*;
    pub use crate::engine::*;
    pub use crate::health::*;
    pub use crate::items::*;
    pub use crate::map::*;
    pub use crate::path::*;
    pub use crate::player::*;
    pub use crate::screen::*;
    pub use crate::skills::*;
    pub use crate::state::*;
    pub use crate::targeting::*;
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

    pub const REST_HEALTH_PERCENTAGE: f32 = 0.75;
    pub const REST_WILL_PERCENTAGE: f32 = 0.2;

    pub const VISION: usize = 8;
}
