use std::collections::HashSet;

use macroquad::rand::ChooseRandom;

use crate::{
    mission::{Character, MissionState},
    prelude::*,
    screens::victory::VictoryState,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RuneKinds {
    Fire,
    Life,
    Ice,
    Force,
    Protection,
    Mind,
}

impl std::fmt::Display for RuneKinds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuneKinds::Fire => f.write_str("Fire"),
            RuneKinds::Life => f.write_str("Life"),
            RuneKinds::Ice => f.write_str("Ice"),
            RuneKinds::Force => f.write_str("Force"),
            RuneKinds::Protection => f.write_str("Protection"),
            RuneKinds::Mind => f.write_str("Mind"),
        }
    }
}

impl RuneKinds {
    pub fn all() -> [RuneKinds; 6] {
        [
            RuneKinds::Fire,
            RuneKinds::Life,
            RuneKinds::Ice,
            RuneKinds::Force,
            RuneKinds::Protection,
            RuneKinds::Mind,
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignState {
    pub character: Character,
    pub chosen_upgrades: HashSet<String>,
    pub collected_runes: HashSet<RuneKinds>,
}

impl CampaignState {
    pub fn new(character: Character) -> CampaignState {
        Self {
            character,
            chosen_upgrades: HashSet::new(),
            collected_runes: HashSet::new(),
        }
    }

    pub fn process_ready_for_mission(&mut self, screen: &mut Screen) -> Option<GameFlow> {
        screen.play_random_music();

        let runes_to_find = RuneKinds::all()
            .into_iter()
            .filter(|r| !self.collected_runes.contains(r))
            .collect::<Vec<_>>();

        let rune_to_find = runes_to_find.choose();

        if let Some(rune_to_find) = rune_to_find {
            Some(GameFlow::Gameplay(MissionState::new(
                self.clone(),
                *rune_to_find,
            )))
        } else {
            Some(GameFlow::Victory(VictoryState::new()))
        }
    }

    pub fn game_complete(&self) -> bool {
        RuneKinds::all()
            .into_iter()
            .filter(|r| !self.collected_runes.contains(r))
            .next()
            .is_none()
    }

    pub fn mission_count(&self) -> u32 {
        self.collected_runes.len() as u32
    }
}
