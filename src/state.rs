use directories::ProjectDirs;
use log::debug;
use macroquad::{
    input::{get_keys_pressed, is_key_down, is_quit_requested, prevent_quit},
    shapes::draw_rectangle,
    window::{clear_background, screen_height, screen_width},
};

use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub level: LevelState,
    frame: usize,
    pub current_actor: CurrentActor,
}

impl State {
    pub fn new() -> State {
        let seed = rand::rng().next_u64();
        let mut rng = StdRng::seed_from_u64(seed);
        debug!("Generating map with seed {seed}");

        let level = MapBuilder::build(&mut rng);

        Self {
            level,
            frame: 0,
            current_actor: CurrentActor::PlayerStandardAction,
        }
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
    #[cfg(debug_assertions)]
    DebugMenu(DebugRequest),
}

impl State {
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

    pub fn save_to_disk(&self) {
        if let Some(dirs) = ProjectDirs::from("com", "", "Archivist") {
            match std::fs::create_dir_all(dirs.data_dir()) {
                Ok(_) => {
                    let mut path = dirs.data_dir().to_path_buf();
                    path.push("game.sav");
                    if let Err(e) = std::fs::write(path, self.save_to_string()) {
                        eprintln!("Unable to save game: {e:?}");
                    }
                }
                Err(e) => {
                    eprintln!("Unable to create game game location: {e:?}");
                }
            }
        }
    }

    pub fn load_from_disk() -> Option<Self> {
        if let Some(dirs) = ProjectDirs::from("com", "", "Archivist") {
            let mut path = dirs.data_dir().to_path_buf();
            path.push("game.sav");

            if let Ok(text) = std::fs::read(&path) {
                match serde_json::from_slice(&text) {
                    Ok(state) => {
                        if std::fs::remove_file(path).is_err() {
                            eprintln!("Unable to delete game after load.");
                        }
                        return Some(state);
                    }
                    Err(e) => {
                        eprintln!("Unable to load game: {e:?}");
                    }
                }
            }
        }
        None
    }
}

pub async fn handle_death(state: &State, screen: &mut Screen) -> State {
    let mut death_frame = 0;
    loop {
        death_frame += 1;
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
            "You have died. Press any key to start again.",
            22,
            screen_height() / 2.0,
            Some(GRAY),
        );

        // Make sure keys pressed on frame of death don't trigger new game
        if death_frame > 10 && get_keys_pressed().iter().len() > 0 {
            return State::new();
        }

        macroquad::window::next_frame().await
    }
}

pub async fn main() {
    prevent_quit();
    let mut screen = Screen::new().await;

    let mut state = if let Some(state) = State::load_from_disk() {
        screen.push_floating_text("Welcome back.");

        state
    } else {
        screen.push_floating_text("Explore the Dungeon. Cursor keys to move.");

        State::new()
    };

    loop {
        state.frame += 1;
        clear_background(BLACK);

        loop {
            if is_quit_requested()
                || (is_key_pressed(KeyCode::Q)
                    && (is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift)))
            {
                state.save_to_disk();
                return;
            }

            if let Some(action) = state.current_actor.act(&state.level, &mut screen) {
                state.process_action(action, &mut screen);
            }

            if state.is_player_dead() {
                state = handle_death(&state, &mut screen).await;
            }

            screen
                .camera
                .update(state.get_player().position, state.frame);

            // We continue looping until the current actor needs to wait
            if state.current_actor.needs_to_wait() {
                break;
            }
        }

        state.level.render(&mut screen);
        state.current_actor.render(&screen, &state.level);

        macroquad::window::next_frame().await
    }
}
