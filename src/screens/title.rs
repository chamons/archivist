use crate::campaign::CampaignScreenState;
use crate::mission::{MissionState, Screen};
use crate::prelude::*;

pub struct TitleState {
    has_save_game: bool,
    selection: usize,
}

impl TitleState {
    pub fn new() -> Self {
        TitleState {
            has_save_game: MissionState::savefile_exists(),
            selection: 0,
        }
    }

    pub fn process_frame(&mut self) -> Option<GameFlow> {
        Screen::draw_centered_text("The Archivist", 48, 75.0, None);

        let mut offset = 600.0;
        let mut next_option = 0;
        if self.has_save_game {
            let (color, background) = self.title_color_line(next_option);
            Screen::draw_centered_text_with_color("Load Game", 48, offset, color, background);
            offset += 50.0;
            next_option += 1;
        }

        {
            let (color, background) = self.title_color_line(next_option);
            Screen::draw_centered_text_with_color("New Game", 48, offset, color, background);
            offset += 50.0;
            next_option += 1;
        }
        {
            let (color, background) = self.title_color_line(next_option);
            Screen::draw_centered_text_with_color("Quit", 48, offset, color, background);
        }

        if is_key_pressed(KeyCode::Down) {
            if self.selection < 2 {
                self.selection += 1;
            }
        } else if is_key_pressed(KeyCode::Up) {
            if self.selection > 0 {
                self.selection -= 1;
            }
        } else if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::KpEnter) {
            if self.has_save_game {
                match self.selection {
                    0 => return Some(CampaignScreenState::load_save()),
                    1 => {
                        // If you start a new game delete the save anyway
                        MissionState::delete_any_save();
                        return Some(GameFlow::Campaign(CampaignScreenState::new()));
                    }
                    2 | _ => return Some(GameFlow::Quitting),
                }
            } else {
                match self.selection {
                    0 => {
                        return Some(GameFlow::Campaign(CampaignScreenState::new()));
                    }
                    1 | _ => return Some(GameFlow::Quitting),
                }
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
