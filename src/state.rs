use log::debug;
use macroquad::{
    input::get_keys_pressed,
    shapes::draw_rectangle,
    window::{clear_background, screen_height, screen_width},
};

use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    level: LevelState,
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
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum RequestedAction {
    Move(CharacterId, Point),
    WeaponAttack {
        source: CharacterId,
        target: CharacterId,
        weapon: Weapon,
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
            RequestedAction::Move(id, point) => {
                let actor = self.level.find_character_mut(id);
                actor.position = point;
                if actor.is_player() {
                    self.level.update_visibility();
                }

                spend_ticks(&mut self.level, &mut self.current_actor, id, TICKS_MOVEMENT);
            }
            RequestedAction::WeaponAttack {
                source,
                target,
                weapon,
            } => {
                let target_character = self.level.find_character_mut(target);
                target_character.health.current -= weapon.damage;

                // We do not remove the player character, death checks will happen after action resolution
                if target_character.health.is_dead() && !target_character.is_player() {
                    self.level.remove_character(target);
                }
                spend_ticks(
                    &mut self.level,
                    &mut self.current_actor,
                    source,
                    TICKS_TO_BUMP,
                );
            }
            RequestedAction::Wait(id) => {
                spend_ticks(&mut self.level, &mut self.current_actor, id, TICKS_TO_ACT);
            }
            #[cfg(debug_assertions)]
            RequestedAction::DebugMenu(command) => {
                screen.push_floating_text(&format!("Running debug command: {command:?}"));
                match command {
                    DebugRequest::Save => {
                        std::fs::write("dev.save", self.save()).expect("Unable to save");
                    }
                    DebugRequest::Load => {
                        if let Ok(text) = std::fs::read("dev.save") {
                            *self = serde_json::from_slice(&text).expect("Unable to load dev save");
                        }
                    }
                }
            }
        }
    }

    pub fn save(&self) -> String {
        serde_json::to_string(self).expect("Unable to save game")
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
    let mut state = State::new();
    let mut screen = Screen::new().await;

    screen.push_floating_text("Explore the Dungeon. Cursor keys to move.");

    loop {
        state.frame += 1;
        clear_background(BLACK);

        loop {
            if let Some(action) = state.current_actor.act(&state.level, &screen) {
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
