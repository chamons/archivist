use std::path::PathBuf;

use serde::{Deserialize, Serialize};

pub mod campaign;
mod flow;
pub mod mission;
mod screen;
pub mod screens;
mod util;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Difficulty {
    Normal,
    Easy,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Options {
    pub music: f32,
    pub sound: f32,
    pub difficulty: Difficulty,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            music: 0.35,
            sound: 0.3,
            difficulty: Difficulty::Normal,
        }
    }
}

impl Options {
    pub fn load() -> Options {
        if let Ok(text) = std::fs::read(&Self::options_path()) {
            serde_json::from_slice(&text).unwrap_or_default()
        } else {
            Options::default()
        }
    }

    #[cfg(feature = "desktop")]
    pub fn save(&self) {
        let filename = Self::options_path();

        match std::fs::create_dir_all(filename.parent().expect("Project dir should be longer")) {
            Ok(()) => {
                if let Err(e) = std::fs::write(
                    filename,
                    serde_json::to_string(self).expect("Unable to save options"),
                ) {
                    eprintln!("Unable to save options: {e:?}");
                }
            }
            Err(e) => {
                eprintln!("Unable to create options location: {e:?}");
            }
        }
    }

    #[cfg(not(feature = "desktop"))]
    pub fn save(&self) {}

    #[cfg(feature = "desktop")]
    pub fn options_path() -> PathBuf {
        let dirs = directories::ProjectDirs::from("com", "", "Archivist")
            .expect("Unable to find project directory?");
        let mut path = dirs.data_dir().to_path_buf();
        path.push("options.json");
        path
    }

    #[cfg(not(feature = "desktop"))]
    pub fn options_path() -> PathBuf {
        PathBuf::new()
    }
}

pub mod prelude {
    pub use crate::flow::*;
    pub use crate::screen::*;
    pub use crate::util::*;

    pub use macroquad::color::*;
    pub use macroquad::input::{KeyCode, is_key_pressed};
    pub use macroquad::math::Rect as MRect;
    pub use macroquad::rand;
    pub use macroquad::rand::RandGenerator;

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

    pub const STATUS_EFFECT_MIGHT_DAMAGE_BOOST: i32 = 3;
    pub const STATUS_EFFECT_PROTECTION_DEFENSE_BOOST: i32 = 3;
    pub const STATUS_EFFECT_WEAKNESS_DAMAGE_REDUCTION: i32 = 2;
    pub const STATUS_EFFECT_CHANCE_BLIND_MISS: f64 = 0.25;
    pub const STATUS_EFFECT_CHANCE_DODGE_MISS: f64 = 0.25;
    pub const STATUS_EFFECT_CHANCE_ROOT_STAY_STILL: f64 = 0.33;

    pub const BASE_HEALTH_INCREASE_EVERY_MISSION: i32 = 8;
    pub const BASE_DEFENSE_INCREASE_EVERY_MISSION: i32 = 0;

    pub const DEFENSE_IGNORED_SOME_PIERCE: i32 = 2;

    pub const VERSION: &str = "0.21";
}
