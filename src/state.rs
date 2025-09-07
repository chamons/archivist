use std::path::PathBuf;

use directories::ProjectDirs;
use log::debug;
use macroquad::{
    input::{get_keys_pressed, is_key_down, is_quit_requested, prevent_quit},
    shapes::draw_rectangle,
    window::{clear_background, screen_height, screen_width},
};

use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    pub level: LevelState,
    frame: usize,
    pub current_actor: CurrentActor,

    pub completed: bool,
}

impl State {
    pub fn new() -> State {
        let seed = rand::rng().next_u64();
        let mut rng = StdRng::seed_from_u64(seed);
        debug!("Generating map with seed {seed}");

        let level = CellsMapBuilder::build(&mut rng);

        level.map.dump_map_to_console();

        Self {
            level,
            frame: 0,
            current_actor: CurrentActor::PlayerStandardAction,
            completed: false,
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
    Stairs,
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

enum GameFlow {
    Title(bool, usize),
    Gameplay(State),
    Dead(usize, State),
    Quitting,
    Victory(usize, State),
}

impl GameFlow {
    pub fn frame(&mut self, screen: &mut Screen) {
        let maybe_next = match self {
            GameFlow::Title(has_save_game, selection) => {
                Self::render_title(*has_save_game, selection)
            }
            GameFlow::Gameplay(state) => Self::render_gameplay(state, screen),
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
        Self::Title(State::savefile_exists(), 0)
    }

    fn render_victory(
        state: &mut State,
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
                        let state = State::load_from_disk().unwrap_or_else(|| State::new());
                        return Some(GameFlow::Gameplay(state));
                    }
                    1 => {
                        return Some(GameFlow::Gameplay(State::new()));
                    }
                    2 | _ => return Some(GameFlow::Quitting),
                }
            } else {
                match selection {
                    0 => {
                        return Some(GameFlow::Gameplay(State::new()));
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

    fn render_gameplay(state: &mut State, screen: &mut Screen) -> Option<GameFlow> {
        state.frame += 1;

        loop {
            if is_quit_requested()
                || (is_key_pressed(KeyCode::Q)
                    && (is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift)))
            {
                state.save_to_disk();
                return Some(GameFlow::Quitting);
            }

            if let Some(action) = state.current_actor.act(&state.level, screen) {
                state.process_action(action, screen);
            }

            if state.is_player_dead() {
                return Some(GameFlow::Dead(0, state.clone()));
            } else if state.completed {
                return Some(GameFlow::Victory(0, state.clone()));
            }

            screen
                .camera
                .update(state.get_player().position, state.frame);

            // We continue looping until the current actor needs to wait
            if state.current_actor.needs_to_wait() {
                break;
            }
        }

        state.level.render(screen);
        state.current_actor.render(&screen, &state.level);
        None
    }

    fn render_dead(
        state: &mut State,
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

        flow.frame(&mut screen);

        if matches!(flow, GameFlow::Quitting) {
            break;
        }

        macroquad::window::next_frame().await
    }
}
