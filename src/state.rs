use crate::prelude::*;

#[derive(Debug)]
pub struct State {
    map: Map,
    player: Player,
}

impl State {
    pub fn new() -> State {
        Self {
            map: Map::new(),
            player: Player::new(Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2)),
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
