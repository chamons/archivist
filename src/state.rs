use crate::prelude::*;

#[derive(Debug)]
pub struct LevelState {
    pub map: Map,
    pub characters: Vec<Character>,
}

impl LevelState {
    pub fn get_player(&self) -> &Character {
        self.characters
            .iter()
            .find(|c| c.is_player())
            .expect("Player must still exist")
    }

    pub fn find_character(&self, id: CharacterId) -> &Character {
        self.characters
            .iter()
            .find(|c| c.id == id)
            .expect("Action actor exists")
    }

    pub fn find_character_mut(&mut self, id: CharacterId) -> &mut Character {
        self.characters
            .iter_mut()
            .find(|c| c.id == id)
            .expect("Action actor exists")
    }

    pub fn find_character_at_position(&self, position: Point) -> Option<&Character> {
        self.characters.iter().find(|c| c.position == position)
    }

    pub fn character_can_enter(&self, point: Point) -> bool {
        self.map.in_bounds(point) && self.map.get(point) == TileKind::Floor
    }

    pub fn remove_character(&mut self, id: CharacterId) {
        self.characters.retain(|c| c.id != id);
    }

    pub fn render(&self, screen: &mut Screen, camera: &Camera) {
        for character in &self.characters {
            if camera.is_in_view(character.position) {
                character.render(screen);
            }
        }

        self.map.render(screen);
    }
}

#[derive(Debug)]
pub struct State {
    level: LevelState,
    frame: usize,
    camera: Camera,
    current_actor: CurrentActor,
}

impl State {
    pub fn new() -> State {
        let level = MapBuilder::build(&mut RandomNumberGenerator::new());

        Self {
            level,
            frame: 0,
            camera: Camera::new(),
            current_actor: CurrentActor::PlayerAction,
        }
    }
}

pub enum RequestedAction {
    Move(CharacterId, Point),
    Wait(CharacterId),
}

pub enum ResolvedAction {
    MoveActor(CharacterId, Point),
    RemoveCharacter {
        source: CharacterId,
        target: CharacterId,
    },
    Wait(CharacterId),
}

impl State {
    pub fn get_player(&self) -> &Character {
        self.level
            .characters
            .iter()
            .find(|c| c.is_player())
            .expect("Player must still exist")
    }

    fn resolve_action(&self, action: RequestedAction) -> Option<ResolvedAction> {
        match action {
            RequestedAction::Move(id, target) => {
                let actor = self.level.find_character(id);

                let character_at_target = self.level.find_character_at_position(target);
                if let Some(character_at_target) = character_at_target
                    && actor.is_player()
                {
                    Some(ResolvedAction::RemoveCharacter {
                        target: character_at_target.id,
                        source: id,
                    })
                } else if self.level.character_can_enter(target) {
                    Some(ResolvedAction::MoveActor(id, target))
                } else {
                    None
                }
            }
            RequestedAction::Wait(id) => Some(ResolvedAction::Wait(id)),
        }
    }

    fn process_action(&mut self, action: RequestedAction) -> bool {
        if let Some(resolved_action) = self.resolve_action(action) {
            match resolved_action {
                ResolvedAction::MoveActor(id, point) => {
                    println!("Moving {id:?} to {point:?}");
                    self.level.find_character_mut(id).position = point;
                    self.spend_ticks(id, TICKS_MOVEMENT);
                    true
                }
                ResolvedAction::RemoveCharacter { source, target } => {
                    self.level.remove_character(target);
                    self.spend_ticks(source, TICKS_TO_BUMP);
                    true
                }
                ResolvedAction::Wait(id) => {
                    println!("Waiting on {id:?}");
                    self.spend_ticks(id, TICKS_TO_ACT);
                    true
                }
            }
        } else {
            false
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
        println!("Spending {amount} ticks on {id:?}");
        self.level.find_character_mut(id).ticks -= amount;

        if let Some(next) = self.find_next_actor() {
            if self.level.find_character(next).is_player() {
                self.current_actor = CurrentActor::PlayerAction;
            } else {
                self.current_actor = CurrentActor::EnemyAction(next);
            }
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.frame += 1;

        let mut needs_paint = false;
        loop {
            // Only paint if we are on the first frame
            needs_paint |= self.frame == 1;

            // The current actor did something
            if let Some(action) = self.current_actor.act(&self.level, ctx) {
                needs_paint |= self.process_action(action);
            }

            // Or our animation bounce changed
            needs_paint |= self.camera.update(self.get_player().position, self.frame);

            // We continue looping until the current actor needs to wait
            if self.current_actor.needs_to_wait() {
                break;
            }
        }

        if needs_paint {
            let mut screen = Screen::new(ctx, &self.camera);

            // Only clear text console as sprite "fonts" should draw every square
            screen.clear();

            self.level.render(&mut screen, &self.camera);
        }
    }
}
