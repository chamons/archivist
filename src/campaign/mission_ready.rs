use crate::{
    mission::{Character, MissionState},
    prelude::*,
    screens::victory::VictoryState,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignState {
    pub character: Character,
    pub mission_count: u32,
}

impl CampaignState {
    pub fn new(character: Character) -> CampaignState {
        Self {
            character,
            mission_count: 0,
        }
    }

    pub fn process_ready_for_mission(&mut self) -> Option<GameFlow> {
        if self.mission_count == MISSIONS_TO_VICTORY {
            Some(GameFlow::Victory(VictoryState::new()))
        } else {
            Some(GameFlow::Gameplay(MissionState::new(self.clone())))
        }
    }
}
