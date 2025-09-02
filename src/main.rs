use archivist::prelude::*;
use bracket_lib::prelude::{BError, main_loop};
use env_logger::Logger;

fn main() -> BError {
    let _logger = Logger::from_default_env();

    let context = Screen::console_window_config().build()?;

    main_loop(context, State::new())
}
