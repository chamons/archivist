use crate::{mission::MissionState, prelude::*, screens::victory::VictoryState};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignState {
    mission_count: u32,
}

impl CampaignState {
    pub fn new() -> Self {
        Self { mission_count: 0 }
    }

    pub fn load_save() -> GameFlow {
        match MissionState::load_from_disk() {
            Some(state) => GameFlow::Gameplay(state),
            None => GameFlow::Campaign(CampaignState::new()),
        }
    }

    pub fn process_frame(&mut self) -> Option<GameFlow> {
        if self.mission_count == MISSIONS_TO_VICTORY {
            Some(GameFlow::Victory(VictoryState::new()))
        } else {
            self.mission_count += 1;
            Some(GameFlow::Gameplay(MissionState::new(self.clone())))
        }
    }
}
