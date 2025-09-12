use macroquad::text::draw_text;

use crate::{prelude::*, screens::title::TitleState};

const HELP_TEXT: &str = include_str!("../../data/help.txt");

pub struct HelpState {
    line: usize,
}

impl HelpState {
    pub fn new() -> Self {
        HelpState { line: 0 }
    }

    pub fn process_frame(&mut self) -> Option<GameFlow> {
        for (i, line) in HELP_TEXT.lines().skip(self.line).take(36).enumerate() {
            draw_text(line, 20.0, 20.0 + 20.0 * i as f32, 22.0, WHITE);
        }

        draw_text(
            "Up/Down to Scroll - Press Enter to exit",
            20.0,
            780.0,
            22.0,
            WHITE,
        );
        if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::Kp2) {
            self.line += 1;
            self.line = self.line.min(HELP_TEXT.lines().count() - 1);
        } else if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::Kp8) {
            if self.line > 0 {
                self.line -= 1;
            }
        }
        if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::KpEnter) {
            return Some(GameFlow::Title(TitleState::new()));
        }
        None
    }
}
