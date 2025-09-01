use archivist::prelude::*;
use bracket_lib::prelude::{BError, BTermBuilder, main_loop};
use env_logger::Logger;

fn main() -> BError {
    let _logger = Logger::from_default_env();

    let context = BTermBuilder::simple80x50()
        .with_title("The Archivist")
        .with_fps_cap(30.0)
        .build()?;

    main_loop(context, State::new())
}
