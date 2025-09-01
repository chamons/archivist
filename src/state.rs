use crate::prelude::*;

pub struct State {
    map: Map,
}

impl State {
    pub fn new() -> State {
        Self { map: Map::new() }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.map.render(ctx);
    }
}
