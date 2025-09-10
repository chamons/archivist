use macroquad::input::get_keys_pressed;
use macroquad::window::{clear_background, screen_height};

use crate::prelude::*;
use crate::screens::title::TitleState;

pub struct VictoryState {
    frame: usize,
}

impl VictoryState {
    pub fn new() -> Self {
        Self { frame: 0 }
    }

    pub fn process_frame(&mut self) -> Option<GameFlow> {
        self.frame += 1;
        clear_background(BLACK);

        Screen::draw_centered_text(
            "You have won. Press any key.",
            22,
            screen_height() / 2.0,
            Some(GRAY),
        );

        if self.frame > 10 && get_keys_pressed().iter().len() > 0 {
            Some(GameFlow::Title(TitleState::new()))
        } else {
            None
        }
    }
}
