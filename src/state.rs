use crate::prelude::*;

#[derive(Debug)]
pub struct State {
    map: Map,
    player: Player,
    frame: usize,
}

impl State {
    pub fn new() -> State {
        let (map, player_position) = MapBuilder::build(&mut RandomNumberGenerator::new());

        Self {
            map,
            player: Player::new(player_position),
            frame: 0,
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.frame += 1;
        // Only clear text console as sprite "fonts" should draw every square
        ctx.set_active_console(0);
        ctx.cls();

        self.player.update(ctx, &self.map);

        let camera = Camera::new(&self.player);
        self.player.render(ctx, &camera);
        self.map.render(ctx, &camera);
    }
}
