mod actor;
mod ai;
mod camera;
mod character;
mod data;
mod health;
mod level;
mod map;
mod player;
mod screen;
mod state;
mod util;

pub mod prelude {
    pub use crate::actor::*;
    pub use crate::ai::*;
    pub use crate::camera::*;
    pub use crate::character::*;
    pub use crate::data::*;
    pub use crate::health::*;
    pub use crate::level::*;
    pub use crate::map::*;
    pub use crate::player::*;
    pub use crate::screen::*;
    pub use crate::state::*;
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

    pub const TICKS_TO_ACT: i32 = 100;
    pub const TICKS_MOVEMENT: i32 = 100;
    pub const TICKS_TO_BUMP: i32 = 100;
}
