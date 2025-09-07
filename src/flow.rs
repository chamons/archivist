use macroquad::{
    input::{get_keys_pressed, prevent_quit},
    shapes::draw_rectangle,
    window::{clear_background, screen_height, screen_width},
};

use crate::{
    mission::{MissionState, Screen},
    prelude::*,
};

pub enum GameFlow {
    Title(bool, usize),
    Gameplay(MissionState),
    Dead(usize, MissionState),
    Quitting,
    Victory(usize, MissionState),
}

impl GameFlow {
    pub fn process_frame(&mut self, screen: &mut Screen) {
        let maybe_next = match self {
            GameFlow::Title(has_save_game, selection) => {
                Self::render_title(*has_save_game, selection)
            }
            GameFlow::Gameplay(state) => state.process_frame(screen),
            GameFlow::Dead(death_frame, state) => Self::render_dead(state, screen, death_frame),
            GameFlow::Victory(victory_frame, state) => {
                Self::render_victory(state, screen, victory_frame)
            }
            GameFlow::Quitting => return,
        };
        if let Some(next) = maybe_next {
            *self = next;
        }
    }

    pub fn title() -> Self {
        Self::Title(MissionState::savefile_exists(), 0)
    }

    fn render_victory(
        state: &mut MissionState,
        screen: &mut Screen,
        victory_frame: &mut usize,
    ) -> Option<GameFlow> {
        *victory_frame += 1;
        clear_background(BLACK);
        state.level.render(screen);
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

        if *victory_frame > 10 && get_keys_pressed().iter().len() > 0 {
            Some(GameFlow::title())
        } else {
            None
        }
    }

    fn render_title(has_save_game: bool, selection: &mut usize) -> Option<GameFlow> {
        Screen::draw_centered_text("The Archivist", 48, 75.0, None);

        let mut offset = 600.0;
        let mut next_option = 0;
        if has_save_game {
            let (color, background) = Self::title_color_line(next_option, *selection);
            Screen::draw_centered_text_with_color("Load Game", 48, offset, color, background);
            offset += 50.0;
            next_option += 1;
        }

        {
            let (color, background) = Self::title_color_line(next_option, *selection);
            Screen::draw_centered_text_with_color("New Game", 48, offset, color, background);
            offset += 50.0;
            next_option += 1;
        }
        {
            let (color, background) = Self::title_color_line(next_option, *selection);
            Screen::draw_centered_text_with_color("Quit", 48, offset, color, background);
        }

        if is_key_pressed(KeyCode::Down) {
            if *selection < 2 {
                *selection += 1;
            }
        } else if is_key_pressed(KeyCode::Up) {
            if *selection > 0 {
                *selection -= 1;
            }
        } else if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::KpEnter) {
            if has_save_game {
                match selection {
                    0 => {
                        let state =
                            MissionState::load_from_disk().unwrap_or_else(|| MissionState::new());
                        return Some(GameFlow::Gameplay(state));
                    }
                    1 => {
                        return Some(GameFlow::Gameplay(MissionState::new()));
                    }
                    2 | _ => return Some(GameFlow::Quitting),
                }
            } else {
                match selection {
                    0 => {
                        return Some(GameFlow::Gameplay(MissionState::new()));
                    }
                    1 | _ => return Some(GameFlow::Quitting),
                }
            }
        }

        None
    }

    fn title_color_line(current: usize, selection: usize) -> (Color, Option<Color>) {
        if current == selection {
            (BLUE, Some(WHITE))
        } else {
            (WHITE, None)
        }
    }

    fn render_dead(
        state: &mut MissionState,
        screen: &mut Screen,
        death_frame: &mut usize,
    ) -> Option<GameFlow> {
        *death_frame += 1;
        clear_background(BLACK);
        state.level.render(screen);
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
            "You have died. Press any key.",
            22,
            screen_height() / 2.0,
            Some(GRAY),
        );

        if *death_frame > 10 && get_keys_pressed().iter().len() > 0 {
            Some(GameFlow::title())
        } else {
            None
        }
    }
}

pub async fn main() {
    prevent_quit();
    let mut screen = Screen::new().await;

    let mut flow = GameFlow::title();

    loop {
        clear_background(BLACK);

        flow.process_frame(&mut screen);

        if matches!(flow, GameFlow::Quitting) {
            break;
        }

        macroquad::window::next_frame().await
    }
}
