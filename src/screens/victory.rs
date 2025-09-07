use macroquad::input::get_keys_pressed;
use macroquad::shapes::draw_rectangle;
use macroquad::window::{clear_background, screen_height, screen_width};

use crate::mission::{MissionState, Screen};
use crate::prelude::*;
use crate::screens::title::TitleState;

pub struct VictoryState {
    frame: usize,
    mission_state: MissionState,
}

impl VictoryState {
    pub fn new(mission_state: MissionState) -> Self {
        Self {
            frame: 0,
            mission_state,
        }
    }

    pub fn process_frame(&mut self, screen: &mut Screen) -> Option<GameFlow> {
        self.frame += 1;
        clear_background(BLACK);
        self.mission_state.level.render(screen);
        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.60,
            },
        );

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
