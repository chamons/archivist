use crate::prelude::*;

#[derive(Debug)]
pub struct State {
    map: Map,
    player: Player,
    frame: usize,
    camera: Camera,
}

impl State {
    pub fn new() -> State {
        let (map, player_position) = MapBuilder::build(&mut RandomNumberGenerator::new());

        Self {
            map,
            player: Player::new(player_position),
            frame: 0,
            camera: Camera::new(),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.frame += 1;

        // Only paint if we are on the first frame, the player did something, or our animation bounce changed
        let mut needs_paint = self.frame == 1;
        needs_paint |= self.player.update(ctx, &self.map);
        needs_paint |= self.camera.update(&self.player, self.frame);

        if needs_paint {
            let mut screen = Screen::new(ctx, &self.camera);

            // Only clear text console as sprite "fonts" should draw every square
            screen.clear();

            self.player.render(&mut screen);
            self.map.render(&mut screen);
        }
    }
}
