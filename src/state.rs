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

    pub fn render(&self, screen: &mut Screen) {
        for character in &self.characters {
            character.render(screen);
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
}

pub enum ResolvedAction {
    MoveActor(CharacterId, Point),
    RemoveCharacter(CharacterId),
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
                    Some(ResolvedAction::RemoveCharacter(character_at_target.id))
                } else if self.level.character_can_enter(target) {
                    Some(ResolvedAction::MoveActor(id, target))
                } else {
                    None
                }
            }
        }
    }

    fn process_action(&mut self, action: RequestedAction) -> bool {
        if let Some(resolved_action) = self.resolve_action(action) {
            match resolved_action {
                ResolvedAction::MoveActor(id, point) => {
                    self.level.find_character_mut(id).position = point;
                    true
                }
                ResolvedAction::RemoveCharacter(id) => {
                    self.level.remove_character(id);
                    true
                }
            }
        } else {
            false
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.frame += 1;

        // Only paint if we are on the first frame, the current actor did something, or our animation bounce changed
        let mut needs_paint = self.frame == 1;

        if let Some(action) = self.current_actor.act(&self.level, ctx) {
            needs_paint |= self.process_action(action);
        }

        needs_paint |= self.camera.update(self.get_player().position, self.frame);

        if needs_paint {
            let mut screen = Screen::new(ctx, &self.camera);

            // Only clear text console as sprite "fonts" should draw every square
            screen.clear();

            self.level.render(&mut screen);
        }
    }
}
