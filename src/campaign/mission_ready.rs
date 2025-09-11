use std::collections::HashSet;

use crate::{
    mission::{Character, MissionState},
    prelude::*,
    screens::victory::VictoryState,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignState {
    pub character: Character,
    pub mission_count: u32,
    pub chosen_upgrades: HashSet<String>,
}

impl CampaignState {
    pub fn new(character: Character) -> CampaignState {
        Self {
            character,
            mission_count: 0,
            chosen_upgrades: HashSet::new(),
        }
    }

    pub fn process_ready_for_mission(&mut self, screen: &mut Screen) -> Option<GameFlow> {
        screen.play_random_music();

        if self.mission_count == MISSIONS_TO_VICTORY {
            Some(GameFlow::Victory(VictoryState::new()))
        } else {
            Some(GameFlow::Gameplay(MissionState::new(self.clone())))
        }
    }
}
