use archivist::prelude::*;
use bracket_lib::prelude::{BError, BTermBuilder, main_loop};
use env_logger::Logger;

fn main() -> BError {
    let _logger = Logger::from_default_env();

    let context = BTermBuilder::new()
        .with_title("The Archivist")
        .with_fps_cap(30.0)
        .with_dimensions(CAMERA_VIEWPORT_WIDTH, CAMERA_DISPLAY_HEIGHT)
        .with_tile_dimensions(24, 24)
        .with_resource_path("resources/")
        .with_font("terminal8x8.png", 8, 8)
        .with_font("oryx_16bit_fantasy_creatures.png", 24, 24)
        .with_font("oryx_16bit_fantasy_world.png", 24, 24)
        .with_simple_console_no_bg(SCREEN_WIDTH, SCREEN_HEIGHT, "terminal8x8.png")
        .with_simple_console_no_bg(
            CAMERA_VIEWPORT_WIDTH,
            CAMERA_DISPLAY_HEIGHT,
            "oryx_16bit_fantasy_world.png",
        )
        .with_simple_console_no_bg(
            CAMERA_VIEWPORT_WIDTH,
            CAMERA_DISPLAY_HEIGHT,
            "oryx_16bit_fantasy_creatures.png",
        )
        .build()?;

    main_loop(context, State::new())
}
