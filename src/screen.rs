use macroquad::{
    shapes::{draw_rectangle, draw_rectangle_lines},
    text::{draw_text, measure_text},
    texture::{DrawTextureParams, Texture2D, build_textures_atlas, draw_texture_ex},
    window::screen_width,
};

use crate::prelude::*;

pub enum TileSet {
    Creatures,
    FX,
    Items,
    Tiles,
    World,
}

pub struct FloatingText {
    pub text: String,
    pub timer: u32,
}

pub struct Screen {
    pub creatures: Texture2D,
    pub fx: Texture2D,
    pub items: Texture2D,
    pub tiles: Texture2D,
    pub world: Texture2D,
    pub text: Texture2D,
    pub camera: Camera,

    pub floating_text: Option<FloatingText>,
}

impl Screen {
    pub async fn new() -> Self {
        let creatures =
            macroquad::texture::load_texture("resources/oryx_16bit_fantasy_creatures_trans.png")
                .await
                .expect("Unable to load art");
        let fx = macroquad::texture::load_texture("resources/oryx_16bit_fantasy_fx_trans.png")
            .await
            .expect("Unable to load art");
        let items =
            macroquad::texture::load_texture("resources/oryx_16bit_fantasy_items_trans.png")
                .await
                .expect("Unable to load art");
        let tiles = macroquad::texture::load_texture("resources/oryx_16bit_fantasy_tiles.png")
            .await
            .expect("Unable to load art");
        let world =
            macroquad::texture::load_texture("resources/oryx_16bit_fantasy_world_trans.png")
                .await
                .expect("Unable to load art");
        let text = macroquad::texture::load_texture("resources/terminal8x8.png")
            .await
            .expect("Unable to load art");

        build_textures_atlas();

        let camera = Camera::new();
        Self {
            creatures,
            fx,
            items,
            tiles,
            world,
            text,
            camera,
            floating_text: None,
        }
    }

    pub fn push_floating_text(&mut self, text: &str) {
        self.floating_text = Some(FloatingText {
            text: text.to_string(),
            timer: TICKS_FLOATING_TEXT,
        });
    }

    pub fn render_floating_text(&mut self) {
        if let Some(floating_text) = &mut self.floating_text {
            floating_text.timer -= 1;
            if floating_text.timer == 0 {
                self.floating_text = None;
            } else {
                let foreground_fade = (20 + floating_text.timer as i32).min(60) as f32 / 60.0;
                let text_color = Color::new(
                    1.00 * foreground_fade,
                    1.00 * foreground_fade,
                    1.00 * foreground_fade,
                    1.00,
                );

                let background_fade = (floating_text.timer as i32).min(15) as f32 / 15.0;
                let background = Color::new(0.0, 0.0, 0.0, 1.00 * background_fade);
                Self::draw_centered_text_with_color(
                    &floating_text.text,
                    21,
                    35.0,
                    text_color,
                    Some(background),
                );
            }
        }
    }

    pub fn draw_sprite(&self, set: TileSet, position: Point, tile: Point) {
        let texture = self.get_texture(set);
        let screen_x: f32 = (position.x - self.camera.left_x) as f32;
        let screen_y: f32 = (position.y - self.camera.top_y) as f32;
        draw_texture_ex(
            texture,
            24. * screen_x,
            24. * screen_y,
            WHITE,
            DrawTextureParams {
                source: Some(MRect::new(
                    tile.x as f32 * 24.0,
                    tile.y as f32 * 24.0,
                    24.0,
                    24.0,
                )),
                ..Default::default()
            },
        );
    }

    pub fn draw_fog(&self, position: Point) {
        let screen_x: f32 = (position.x - self.camera.left_x) as f32;
        let screen_y: f32 = (position.y - self.camera.top_y) as f32;
        draw_rectangle(
            24.0 * screen_x,
            24.0 * screen_y,
            24.0,
            24.0,
            Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.60,
            },
        );
    }

    pub fn draw_targeting(&self, position: Point) {
        let screen_x: f32 = (position.x - self.camera.left_x) as f32;
        let screen_y: f32 = (position.y - self.camera.top_y) as f32;
        draw_rectangle_lines(24.0 * screen_x, 24.0 * screen_y, 24.0, 24.0, 2.0, WHITE);
    }

    fn get_texture(&self, set: TileSet) -> &Texture2D {
        match set {
            TileSet::Creatures => &self.creatures,
            TileSet::FX => &self.fx,
            TileSet::Items => &self.items,
            TileSet::Tiles => &self.tiles,
            TileSet::World => &self.world,
        }
    }

    pub fn draw_centered_text(text: &str, size: u16, y: f32, background: Option<Color>) {
        Self::draw_centered_text_with_color(text, size, y, WHITE, background);
    }

    pub fn draw_centered_text_with_color(
        text: &str,
        size: u16,
        y: f32,
        text_color: Color,
        background: Option<Color>,
    ) {
        let text_size = measure_text(text, None, size, 1.0);
        let text_x = screen_width() / 2.0 - text_size.width / 2.0;

        if let Some(background) = background {
            const BACKGROUND_PADDING: f32 = 2.0;

            draw_rectangle(
                text_x - BACKGROUND_PADDING,
                y - text_size.offset_y - BACKGROUND_PADDING,
                text_size.width + BACKGROUND_PADDING * 2.0,
                text_size.height + BACKGROUND_PADDING * 2.0,
                background,
            );
        }

        draw_text(text, text_x, y, size as f32, text_color);
    }
}
