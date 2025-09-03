use crate::prelude::*;

pub enum ScreenLayer {
    Text,
    World,
    Creatures,
}

impl Into<usize> for ScreenLayer {
    fn into(self) -> usize {
        match self {
            ScreenLayer::Text => 2,
            ScreenLayer::World => 0,
            ScreenLayer::Creatures => 1,
        }
    }
}

/// With multiple virtual consoles, a single place to keep it all straight
pub struct Screen<'a> {
    pub ctx: &'a mut BTerm,
    pub camera: &'a Camera,
}

impl<'a> Screen<'a> {
    pub fn new(ctx: &'a mut BTerm, camera: &'a Camera) -> Self {
        Self { ctx, camera }
    }

    pub fn console_window_config() -> BTermBuilder {
        BTermBuilder::new()
            .with_title("The Archivist")
            .with_fps_cap(30.0)
            .with_dimensions(CAMERA_VIEWPORT_WIDTH, CAMERA_DISPLAY_HEIGHT)
            .with_tile_dimensions(SPRITE_SIZE, SPRITE_SIZE)
            .with_resource_path("resources/")
            .with_font("terminal8x8.png", 8, 8)
            .with_font("oryx_16bit_fantasy_creatures.png", SPRITE_SIZE, SPRITE_SIZE)
            .with_font("oryx_16bit_fantasy_world.png", SPRITE_SIZE, SPRITE_SIZE)
            // We use sparse console here because "sprite fonts" don't play well with
            // set everything to ' ' character cls
            // Terminal 0 - World
            .with_sparse_console(
                CAMERA_VIEWPORT_WIDTH,
                CAMERA_DISPLAY_HEIGHT,
                "oryx_16bit_fantasy_world.png",
            )
            // Terminal 1 - Creatures
            .with_sparse_console_no_bg(
                CAMERA_VIEWPORT_WIDTH,
                CAMERA_DISPLAY_HEIGHT,
                "oryx_16bit_fantasy_creatures.png",
            )
            // Terminal 2 - Text
            .with_simple_console_no_bg(
                CAMERA_VIEWPORT_WIDTH * 2,
                CAMERA_DISPLAY_HEIGHT * 2,
                "terminal8x8.png",
            )
    }

    pub fn clear(&mut self) {
        self.ctx.set_active_console(ScreenLayer::Text.into());
        self.ctx.cls();

        self.ctx.set_active_console(ScreenLayer::World.into());
        self.ctx.cls();

        self.ctx.set_active_console(ScreenLayer::Creatures.into());
        self.ctx.cls();
    }

    pub fn set_active(&mut self, layer: ScreenLayer) {
        self.ctx.set_active_console(layer.into());
    }

    pub fn bounce(&self) -> bool {
        self.camera.bounce
    }

    pub fn set_sprite<G: Into<FontCharType>>(&mut self, position: Point, glyph: G) {
        self.ctx.set(
            position.x - self.camera.left_x,
            position.y - self.camera.top_y,
            WHITE,
            BLACK,
            glyph.into(),
        );
    }
}
