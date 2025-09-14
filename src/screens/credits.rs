use macroquad::{
    color::WHITE,
    input::{KeyCode, is_key_pressed},
    text::draw_text,
};

const CREDITS_TEXT: &str = include_str!("../../data/credits.txt");

use crate::{flow::GameFlow, screens::title::TitleState};

pub fn process_credits_frame() -> Option<GameFlow> {
    for (i, line) in CREDITS_TEXT.lines().enumerate() {
        draw_text(line, 20.0, 20.0 + 20.0 * i as f32, 22.0, WHITE);
    }

    draw_text("Press Enter to exit", 20.0, 780.0, 22.0, WHITE);

    if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::KpEnter) {
        return Some(GameFlow::Title(TitleState::new()));
    }
    None
}
