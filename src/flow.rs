use macroquad::{input::prevent_quit, window::clear_background};

use crate::{
    campaign::CampaignScreenState,
    mission::MissionState,
    prelude::*,
    screens::{death::DeathState, title::TitleState, victory::VictoryState},
};

pub enum GameFlow {
    Campaign(CampaignScreenState),
    Title(TitleState),
    Gameplay(MissionState),
    Dead(DeathState),
    Quitting,
    Victory(VictoryState),
}

impl GameFlow {
    pub fn process_frame(&mut self, screen: &mut Screen) {
        let maybe_next = match self {
            GameFlow::Title(state) => state.process_frame(),
            GameFlow::Campaign(state) => state.process_frame(screen),
            GameFlow::Gameplay(state) => state.process_frame(screen),
            GameFlow::Dead(state) => state.process_frame(screen),
            GameFlow::Victory(state) => state.process_frame(),
            GameFlow::Quitting => return,
        };
        if let Some(next) = maybe_next {
            if matches!(next, GameFlow::Title(_)) {
                screen.music.play_music_track(0);
            }
            *self = next;
        }
    }
}

pub async fn main() {
    prevent_quit();
    let mut screen = Screen::new().await;

    clear_background(BLACK);
    Screen::draw_centered_text("Loading...", 32, 200.0, None);
    macroquad::window::next_frame().await;

    screen.music.load().await;

    screen.music.play_music_track(0);
    let mut flow = GameFlow::Title(TitleState::new());

    loop {
        clear_background(BLACK);

        flow.process_frame(&mut screen);

        if matches!(flow, GameFlow::Quitting) {
            break;
        }

        macroquad::window::next_frame().await
    }
}
