use crate::{prelude::*, screens::title::TitleState};

pub struct OptionsState {
    selection: usize,
}

impl OptionsState {
    pub fn new() -> Self {
        OptionsState { selection: 0 }
    }

    pub fn process_frame(&mut self, screen: &mut Screen) -> Option<GameFlow> {
        let mut offset = 350.0;
        let mut next_option = 0;

        {
            let (color, background) = self.title_color_line(next_option);
            Screen::draw_centered_text_with_color(
                &format!("Music: {:.2}", screen.options.music),
                48,
                offset,
                color,
                background,
            );
            offset += 50.0;
            next_option += 1;
        }
        {
            let (color, background) = self.title_color_line(next_option);
            Screen::draw_centered_text_with_color(
                &format!("Sound Effects: {:.2}", screen.options.sound),
                48,
                offset,
                color,
                background,
            );
            offset += 50.0;
            next_option += 1;
        }
        {
            let (color, background) = self.title_color_line(next_option);
            Screen::draw_centered_text_with_color("Exit", 48, offset, color, background);
        }

        if is_key_pressed(KeyCode::Down) {
            if self.selection < 2 {
                self.selection += 1;
            }
        } else if is_key_pressed(KeyCode::Up) {
            if self.selection > 0 {
                self.selection -= 1;
            }
        } else if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::Kp4) {
            if self.selection == 0 {
                screen.options.music -= 0.05;
                screen.options.music = screen.options.music.max(0.0);
                screen.set_music_volume(screen.options.music);
            } else if self.selection == 1 {
                screen.options.sound -= 0.05;
                screen.options.sound = screen.options.sound.max(0.0);
                screen.play_sound("drip");
            }
        } else if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::Kp6) {
            if self.selection == 0 {
                screen.options.music += 0.05;
                screen.options.music = screen.options.music.min(1.0);
                screen.set_music_volume(screen.options.music);
            } else if self.selection == 1 {
                screen.options.sound += 0.05;
                screen.options.sound = screen.options.sound.min(1.0);
                screen.play_sound("drip");
            }
        } else if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::KpEnter) {
            if self.selection == 2 {
                screen.options.save();
                return Some(GameFlow::Title(TitleState::new()));
            }
        }

        None
    }

    fn title_color_line(&self, current: usize) -> (Color, Option<Color>) {
        if current == self.selection {
            (BLUE, Some(WHITE))
        } else {
            (WHITE, None)
        }
    }
}
