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
    current_actor: CurrentActor,
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
#[derive(Debug, PartialEq, Eq)]
pub enum DebugRequest {
    Save,
    Load,
}

#[derive(Debug, PartialEq, Eq)]
pub enum RequestedAction {
    Move(CharacterId, Point),
    DamageCharacter {
        source: CharacterId,
        target: CharacterId,
        weapon: Weapon,
    },
    Wait(CharacterId),
    PlayerTargeting,
    CancelledTargeting,
    #[cfg(debug_assertions)]
    DebugMenu(DebugRequest),
}

pub enum ResolvedAction {
    MoveActor(CharacterId, Point),
    DamageCharacter {
        source: CharacterId,
        target: CharacterId,
        weapon: Weapon,
    },
    Wait(CharacterId),
    PlayerTargeting,
    CancelledTargeting,
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

    // If we get rid of bump to attack we could drop this
    fn resolve_action(&self, action: RequestedAction) -> Option<ResolvedAction> {
        match action {
            RequestedAction::Move(id, target) => {
                let actor = self.level.find_character(id);

                let character_at_target = self.level.find_character_at_position(target);
                if let Some(character_at_target) = character_at_target {
                    Some(ResolvedAction::DamageCharacter {
                        target: character_at_target.id,
                        source: id,
                        weapon: actor.weapon.clone(),
                    })
                } else if self.level.character_can_enter(target) {
                    Some(ResolvedAction::MoveActor(id, target))
                } else {
                    None
                }
            }
            RequestedAction::DamageCharacter {
                source,
                target,
                weapon,
            } => Some(ResolvedAction::DamageCharacter {
                source,
                target,
                weapon,
            }),
            RequestedAction::Wait(id) => Some(ResolvedAction::Wait(id)),
            RequestedAction::PlayerTargeting => Some(ResolvedAction::PlayerTargeting),
            RequestedAction::CancelledTargeting => Some(ResolvedAction::CancelledTargeting),
            #[cfg(debug_assertions)]
            RequestedAction::DebugMenu(command) => Some(ResolvedAction::DebugMenu(command)),
        }
    }

    // Screen used in debug
    #[allow(unused_variables)]
    fn process_action(&mut self, action: RequestedAction, screen: &mut Screen) {
        if let Some(resolved_action) = self.resolve_action(action) {
            match resolved_action {
                ResolvedAction::MoveActor(id, point) => {
                    let actor = self.level.find_character_mut(id);
                    actor.position = point;
                    if actor.is_player() {
                        self.level.update_visibility();
                    }

                    self.spend_ticks(id, TICKS_MOVEMENT);
                }
                ResolvedAction::DamageCharacter {
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
                    self.spend_ticks(source, TICKS_TO_BUMP);
                }
                ResolvedAction::Wait(id) => {
                    self.spend_ticks(id, TICKS_TO_ACT);
                }
                ResolvedAction::PlayerTargeting => {
                    self.current_actor = CurrentActor::PlayerTargeting(TargetingInfo::new(
                        self.get_player().position,
                    ));
                }
                ResolvedAction::CancelledTargeting => {
                    self.current_actor = CurrentActor::PlayerStandardAction;
                }
                #[cfg(debug_assertions)]
                ResolvedAction::DebugMenu(command) => {
                    screen.push_floating_text(&format!("Running debug command: {command:?}"));
                    match command {
                        DebugRequest::Save => {
                            std::fs::write("dev.save", self.save()).expect("Unable to save");
                        }
                        DebugRequest::Load => {
                            if let Ok(text) = std::fs::read("dev.save") {
                                *self =
                                    serde_json::from_slice(&text).expect("Unable to load dev save");
                            }
                        }
                    }
                }
            }
        }
    }

    fn find_next_actor(&mut self) -> Option<CharacterId> {
        // Sort by ticks with id as tiebreaker
        self.level.characters.sort_by_key(|c| (c.ticks, c.id));
        if let Some(actor) = self.level.characters.last() {
            let id = actor.id;
            if actor.ticks < TICKS_TO_ACT {
                let missing = TICKS_TO_ACT - actor.ticks;
                self.add_ticks(missing);
            }
            Some(id)
        } else {
            None
        }
    }

    fn add_ticks(&mut self, amount: i32) {
        for character in &mut self.level.characters {
            character.ticks += amount;
        }
    }

    fn spend_ticks(&mut self, id: CharacterId, amount: i32) {
        self.level.find_character_mut(id).ticks -= amount;

        if let Some(next) = self.find_next_actor() {
            if self.level.find_character(next).is_player() {
                self.current_actor = CurrentActor::PlayerStandardAction;
            } else {
                self.current_actor = CurrentActor::EnemyAction(next);
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
