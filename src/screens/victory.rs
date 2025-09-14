use macroquad::input::get_keys_pressed;
use macroquad::window::clear_background;

use crate::prelude::*;
use crate::screens::title::TitleState;

pub struct VictoryState {
    frame: usize,
}

impl VictoryState {
    pub fn new() -> Self {
        Self { frame: 0 }
    }

    pub fn process_frame(&mut self, screen: &Screen) -> Option<GameFlow> {
        self.frame += 1;
        clear_background(BLACK);

        let mut offset = 350.0;

        Screen::draw_centered_text(
            "Retrieving the final rune has completed the archive.",
            22,
            offset,
            Some(GRAY),
        );
        offset += 40.0;

        Screen::draw_centered_text(
            &format!(
                "You have won on {:?} difficulty. Well Done!",
                screen.options.difficulty
            ),
            22,
            offset,
            Some(GRAY),
        );
        offset += 40.0;

        Screen::draw_centered_text("Press any key", 22, offset, Some(GRAY));

        if self.frame > 10 && get_keys_pressed().iter().len() > 0 {
            Some(GameFlow::Title(TitleState::new()))
        } else {
            None
        }
    }
}
