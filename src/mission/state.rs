use std::path::PathBuf;

use directories::ProjectDirs;
use macroquad::input::{is_key_down, is_quit_requested};

use crate::campaign::CampaignScreenState;
use crate::campaign::CampaignState;
use crate::mission::*;
use crate::prelude::*;
use crate::screens::death::DeathState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionState {
    pub level: LevelState,
    frame: usize,
    pub current_actor: CurrentActor,

    pub mission_complete: bool,
    pub campaign: CampaignState,
}

impl MissionState {
    pub fn new(campaign: CampaignState) -> MissionState {
        let level = generate_random_map(campaign.character.clone());

        Self {
            level,
            frame: 0,
            current_actor: CurrentActor::PlayerStandardAction,
            mission_complete: false,
            campaign,
        }
    }

    pub fn process_frame(&mut self, screen: &mut Screen) -> Option<GameFlow> {
        self.frame += 1;

        loop {
            if is_quit_requested()
                || (is_key_pressed(KeyCode::Q)
                    && (is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift)))
            {
                self.save_to_disk();
                return Some(GameFlow::Quitting);
            }

            if let Some(action) = self.current_actor.act(&self.level, screen) {
                self.process_action(action, screen);
            }

            if self.is_player_dead() {
                return Some(GameFlow::Dead(DeathState::new(self.clone())));
            } else if self.mission_complete {
                return Some(GameFlow::Campaign(CampaignScreenState::mission_complete(
                    self.campaign.clone(),
                )));
            }

            screen.camera.update(self.get_player().position, self.frame);

            // We continue looping until the current actor needs to wait
            if self.current_actor.needs_to_wait() {
                break;
            }
        }

        self.level.render(screen);
        self.current_actor.render(&screen, &self.level);
        None
    }
}

#[cfg(debug_assertions)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DebugRequest {
    Save,
    Load,
    DumpState,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum RequestedAction {
    Move(CharacterId, Point),
    WeaponAttack {
        source: CharacterId,
        target: CharacterId,
        weapon: Weapon,
    },
    UseSkill {
        source: CharacterId,
        target: CharacterId,
        skill_name: String,
    },
    Wait(CharacterId),
    Stairs,
    #[cfg(debug_assertions)]
    DebugMenu(DebugRequest),
}

impl MissionState {
    pub fn get_player(&self) -> &Character {
        self.level
            .characters
            .iter()
            .find(|c| c.is_player())
            .expect("Player must still exist")
    }

    pub fn is_player_dead(&self) -> bool {
        self.get_player().health.is_dead()
    }

    // Screen used in debug
    #[allow(unused_variables)]
    fn process_action(&mut self, action: RequestedAction, screen: &mut Screen) {
        match action {
            RequestedAction::Move(id, dest) => {
                move_character(self, id, dest, screen);
            }
            RequestedAction::WeaponAttack {
                source,
                target,
                weapon,
            } => {
                weapon_attack(self, source, target, weapon);
            }
            RequestedAction::Wait(id) => {
                character_wait(self, id, screen);
            }
            RequestedAction::UseSkill {
                source,
                target,
                skill_name,
            } => {
                apply_skill(self, source, target, &skill_name);
            }
            RequestedAction::Stairs => ascend_stars(self, screen),
            #[cfg(debug_assertions)]
            RequestedAction::DebugMenu(command) => {
                screen.push_floating_text(&format!("Running debug command: {command:?}"));
                match command {
                    DebugRequest::Save => {
                        std::fs::write("dev.save", self.save_to_string()).expect("Unable to save");
                    }
                    DebugRequest::Load => {
                        if let Ok(text) = std::fs::read("dev.save") {
                            *self = serde_json::from_slice(&text).expect("Unable to load dev save");
                        }
                    }
                    DebugRequest::DumpState => {
                        let log = serde_json::to_string_pretty(&self.level)
                            .expect("Unable to dump state");
                        std::fs::write("dev.log", log).expect("Unable to write dump");
                    }
                }
            }
        }
    }

    pub fn save_to_string(&self) -> String {
        serde_json::to_string(self).expect("Unable to save game")
    }

    pub fn savefile_name() -> PathBuf {
        let dirs =
            ProjectDirs::from("com", "", "Archivist").expect("Unable to find project directory?");
        let mut path = dirs.data_dir().to_path_buf();
        path.push("game.sav");
        path
    }

    pub fn savefile_exists() -> bool {
        let filename = Self::savefile_name();

        match std::fs::exists(&filename) {
            Ok(exists) => exists,
            Err(_) => false,
        }
    }

    pub fn save_to_disk(&self) {
        let filename = Self::savefile_name();

        match std::fs::create_dir_all(filename.parent().expect("Project dir should be longer")) {
            Ok(()) => {
                if let Err(e) = std::fs::write(filename, self.save_to_string()) {
                    eprintln!("Unable to save game: {e:?}");
                }
            }
            Err(e) => {
                eprintln!("Unable to create game game location: {e:?}");
            }
        }
    }

    pub fn load_from_disk() -> Option<Self> {
        let filename = Self::savefile_name();
        if let Ok(text) = std::fs::read(&filename) {
            match serde_json::from_slice(&text) {
                Ok(state) => {
                    if std::fs::remove_file(filename).is_err() {
                        eprintln!("Unable to delete game after load.");
                    }
                    return Some(state);
                }
                Err(e) => {
                    eprintln!("Unable to load game: {e:?}");
                }
            }
        }

        None
    }
}
