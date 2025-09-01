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
            // Only clear text console as sprite "fonts" should draw every square
            ctx.set_active_console(0);
            ctx.cls();

            ctx.set_active_console(1);
            ctx.fill_region(
                Rect::with_size(
                    0,
                    0,
                    CAMERA_VIEWPORT_WIDTH * SPRITE_SIZE as i32,
                    CAMERA_DISPLAY_HEIGHT * SPRITE_SIZE as i32,
                ),
                0,
                BLACK,
                BLACK,
            );

            ctx.set_active_console(2);
            ctx.fill_region(
                Rect::with_size(
                    0,
                    0,
                    CAMERA_VIEWPORT_WIDTH * SPRITE_SIZE as i32,
                    CAMERA_DISPLAY_HEIGHT * SPRITE_SIZE as i32,
                ),
                0,
                BLACK,
                BLACK,
            );

            self.player.render(ctx, &self.camera);
            self.map.render(ctx, &self.camera);
        }
    }
}
