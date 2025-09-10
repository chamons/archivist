use crate::{
    campaign::{select_equipment::SelectEquipmentState, upgrade::UpgradeState},
    mission::MissionState,
    prelude::*,
};

mod mission_ready;
pub use mission_ready::CampaignState;

mod select_equipment;

mod upgrade;

#[derive(Debug, Clone, Serialize, Deserialize)]
enum CampaignStep {
    SelectEquipment(SelectEquipmentState),
    SelectUpgrade(UpgradeState),
    MissionReady(CampaignState),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignScreenState {
    step: CampaignStep,
    frame: usize,
}

impl CampaignScreenState {
    pub fn new() -> Self {
        Self {
            step: CampaignStep::SelectEquipment(SelectEquipmentState::new()),
            frame: 0,
        }
    }

    pub fn mission_complete(mut campaign: CampaignState) -> Self {
        campaign.mission_count += 1;

        if campaign.mission_count == MISSIONS_TO_VICTORY {
            // Quickly move us to MissionReady so we can win
            Self {
                step: CampaignStep::MissionReady(campaign),
                frame: 0,
            }
        } else {
            Self {
                step: CampaignStep::SelectUpgrade(UpgradeState::new(campaign)),
                frame: 0,
            }
        }
    }

    pub fn load_save() -> GameFlow {
        match MissionState::load_from_disk() {
            Some(state) => GameFlow::Gameplay(state),
            None => GameFlow::Campaign(CampaignScreenState::new()),
        }
    }

    pub fn process_frame(&mut self, screen: &Screen) -> Option<GameFlow> {
        self.frame += 1;

        match &mut self.step {
            CampaignStep::SelectEquipment(state) => {
                if let Some(next_step) = state.process_frame(screen, self.frame) {
                    self.step = next_step;
                }
                None
            }
            CampaignStep::SelectUpgrade(state) => {
                if let Some(next_step) = state.process_frame() {
                    self.step = next_step;
                }
                None
            }
            CampaignStep::MissionReady(state) => state.process_ready_for_mission(),
        }
    }
}
