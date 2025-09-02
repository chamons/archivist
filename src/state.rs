use crate::prelude::*;

#[derive(Debug)]
pub struct State {
    map: Map,
    characters: Vec<Character>,
    frame: usize,
    camera: Camera,
}

impl State {
    pub fn new() -> State {
        let (map, characters) = MapBuilder::build(&mut RandomNumberGenerator::new());

        Self {
            map,
            frame: 0,
            camera: Camera::new(),
            characters,
        }
    }
}

pub enum RequestedAction {
    Move(Point),
}

pub enum ResolvedAction {
    MoveActor(Point),
    RemoveCharacter(CharacterId),
}

impl State {
    fn get_player(&self) -> &Character {
        self.characters
            .iter()
            .find(|c| c.is_player())
            .expect("Player must still exist")
    }

    fn resolve_action(
        &self,
        action: RequestedAction,
        actor_id: CharacterId,
    ) -> Option<ResolvedAction> {
        let actor = self
            .characters
            .iter()
            .find(|c| c.id == actor_id)
            .expect("Action actor exists");

        match action {
            RequestedAction::Move(target) => {
                let character_at_target = self.characters.iter().find(|c| c.position == target);
                if let Some(character_at_target) = character_at_target
                    && actor.is_player()
                {
                    Some(ResolvedAction::RemoveCharacter(character_at_target.id))
                } else if self.map.can_enter(target) {
                    Some(ResolvedAction::MoveActor(target))
                } else {
                    None
                }
            }
        }
    }

    fn process_action(&mut self, action: RequestedAction, actor_id: CharacterId) -> bool {
        if let Some(resolved_action) = self.resolve_action(action, actor_id) {
            match resolved_action {
                ResolvedAction::MoveActor(point) => {
                    self.characters
                        .iter_mut()
                        .find(|c| c.id == actor_id)
                        .expect("Actor exists")
                        .position = point;
                    true
                }
                ResolvedAction::RemoveCharacter(character_id) => {
                    self.characters.retain(|c| c.id != character_id);
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

        // Only paint if we are on the first frame, the player did something, or our animation bounce changed
        let mut needs_paint = self.frame == 1;

        {
            let player = self.get_player();
            if let Some(action) = get_player_action(player, ctx) {
                needs_paint |= self.process_action(action, player.id);
            }
        }
        needs_paint |= self.camera.update(self.get_player().position, self.frame);

        if needs_paint {
            let mut screen = Screen::new(ctx, &self.camera);

            // Only clear text console as sprite "fonts" should draw every square
            screen.clear();

            for character in &self.characters {
                character.render(&mut screen);
            }

            self.map.render(&mut screen);
        }
    }
}
