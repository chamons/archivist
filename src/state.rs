use crate::prelude::*;

#[derive(Debug)]
pub struct State {
    map: Map,
    player: Player,
}

impl State {
    pub fn new() -> State {
        let (map, player_position) = MapBuilder::build(&mut RandomNumberGenerator::new());

        Self {
            map,
            player: Player::new(player_position),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        self.player.update(ctx, &self.map);

        self.map.render(ctx);
        self.player.render(ctx);
    }
}
