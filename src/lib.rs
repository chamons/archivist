mod map;
mod state;

pub mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: u32 = 80;
    pub const SCREEN_HEIGHT: u32 = 50;
    pub use crate::map::*;
    pub use crate::state::*;
}
