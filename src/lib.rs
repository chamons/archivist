mod camera;
mod map;
mod player;
mod state;
mod util;

pub mod prelude {
    pub use crate::camera::*;
    pub use crate::map::*;
    pub use crate::player::*;
    pub use crate::state::*;
    pub use bracket_lib::prelude::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;

    pub const CAMERA_VIEWPORT_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const CAMERA_DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
}
