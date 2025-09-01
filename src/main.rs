use archivist::prelude::*;
use bracket_lib::prelude::{BError, BTermBuilder, main_loop};

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("The Archivist")
        .with_fps_cap(30.0)
        .build()?;

    main_loop(context, State::new())
}
