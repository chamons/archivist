use std::collections::HashMap;

use macroquad::{
    audio::{PlaySoundParams, Sound, load_sound, play_sound, play_sound_once, stop_sound},
    shapes::{draw_rectangle, draw_rectangle_lines},
    text::{draw_text, measure_text},
    texture::{DrawTextureParams, Texture2D, build_textures_atlas, draw_texture_ex},
    window::screen_width,
};
use rand::Rng;

use crate::mission::*;
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

pub struct Music {
    tracks: Vec<Sound>,
    sounds: HashMap<String, Sound>,
    current_track: Option<usize>,
}

impl Music {
    pub fn new() -> Self {
        Self {
            tracks: vec![],
            sounds: HashMap::new(),
            current_track: None,
        }
    }

    pub async fn load(&mut self) {
        self.tracks = vec![
            load_sound("resources/music/01 Tower Ascent.ogg")
                .await
                .expect("Unable to load music"),
            load_sound("resources/music/02 Cave of the Dead.ogg")
                .await
                .expect("Unable to load music"),
            load_sound("resources/music/03 Imminent Danger.ogg")
                .await
                .expect("Unable to load music"),
            load_sound("resources/music/04 The Underground.ogg")
                .await
                .expect("Unable to load music"),
            load_sound("resources/music/05 Sorceress' Layer.ogg")
                .await
                .expect("Unable to load music"),
            load_sound("resources/music/06 Crystal Mine.ogg")
                .await
                .expect("Unable to load music"),
            load_sound("resources/music/07 Distorted Planet.ogg")
                .await
                .expect("Unable to load music"),
            load_sound("resources/music/08 Stalagmite.ogg")
                .await
                .expect("Unable to load music"),
            load_sound("resources/music/09 From The Darkness.ogg")
                .await
                .expect("Unable to load music"),
            load_sound("resources/music/10 Reprocussions.ogg")
                .await
                .expect("Unable to load music"),
            load_sound("resources/music/11 Tense Situation.ogg")
                .await
                .expect("Unable to load music"),
            load_sound("resources/music/12 Dimensions.ogg")
                .await
                .expect("Unable to load music"),
            load_sound("resources/music/13 Mineshaft.ogg")
                .await
                .expect("Unable to load music"),
            load_sound("resources/music/14 The Only Way.ogg")
                .await
                .expect("Unable to load music"),
        ];

        self.sounds.insert(
            "attack_b".to_string(),
            load_sound("resources/sound/attack_b.wav")
                .await
                .expect("Unable to load sound"),
        );
        self.sounds.insert(
            "burn".to_string(),
            load_sound("resources/sound/burn.wav")
                .await
                .expect("Unable to load sound"),
        );
        self.sounds.insert(
            "curse".to_string(),
            load_sound("resources/sound/curse.wav")
                .await
                .expect("Unable to load sound"),
        );
        self.sounds.insert(
            "impact_a".to_string(),
            load_sound("resources/sound/impact_a.wav")
                .await
                .expect("Unable to load sound"),
        );
        self.sounds.insert(
            "impact_b".to_string(),
            load_sound("resources/sound/impact_b.wav")
                .await
                .expect("Unable to load sound"),
        );
        self.sounds.insert(
            "lightning_a".to_string(),
            load_sound("resources/sound/lightning_a.wav")
                .await
                .expect("Unable to load sound"),
        );
    }

    pub fn play_music_track(&mut self, index: usize) {
        self.play(index);
    }

    pub fn play_random_music(&mut self) {
        let track = rand::rng().random_range(1..self.tracks.len());
        self.play(track);
    }

    pub fn play_sound(&self, name: &str) {
        let sound = self.sounds.get(name).expect("Unable to get sound");
        play_sound_once(sound);
    }

    fn play(&mut self, index: usize) {
        if let Some(current_track) = &self.current_track {
            stop_sound(&self.tracks[*current_track]);
        }

        let track = &self.tracks[index];

        play_sound(
            &track,
            PlaySoundParams {
                looped: true,
                volume: 0.35,
            },
        );
        self.current_track = Some(index);
    }
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
    pub music: Music,
}

impl Screen {
    pub async fn new() -> Self {
        let music = Music::new();

        let creatures = macroquad::texture::load_texture(
            "resources/art/oryx_16bit_fantasy_creatures_trans.png",
        )
        .await
        .expect("Unable to load art");
        let fx = macroquad::texture::load_texture("resources/art/oryx_16bit_fantasy_fx_trans.png")
            .await
            .expect("Unable to load art");
        let items =
            macroquad::texture::load_texture("resources/art/oryx_16bit_fantasy_items_trans.png")
                .await
                .expect("Unable to load art");
        let tiles = macroquad::texture::load_texture("resources/art/oryx_16bit_fantasy_tiles.png")
            .await
            .expect("Unable to load art");
        let world =
            macroquad::texture::load_texture("resources/art/oryx_16bit_fantasy_world_trans.png")
                .await
                .expect("Unable to load art");
        let text = macroquad::texture::load_texture("resources/art/terminal8x8.png")
            .await
            .expect("Unable to load art");

        build_textures_atlas();

        let camera = Camera::new();
        Self {
            music,
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
                    53.0,
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

    pub fn draw_tiny_sprite(&self, set: TileSet, position: Point, tile: Point) {
        let texture = self.get_texture(set);
        let screen_x: f32 = (position.x - self.camera.left_x) as f32;
        let screen_y: f32 = (position.y - self.camera.top_y) as f32;
        draw_texture_ex(
            texture,
            4.0 + 24.0 * screen_x,
            4.0 + 24.0 * screen_y,
            WHITE,
            DrawTextureParams {
                source: Some(MRect::new(
                    tile.x as f32 * 16.0,
                    tile.y as f32 * 16.0,
                    16.0,
                    16.0,
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

    pub fn draw_targeting(&self, position: Point, color: Color) {
        let screen_x: f32 = (position.x - self.camera.left_x) as f32;
        let screen_y: f32 = (position.y - self.camera.top_y) as f32;
        draw_rectangle_lines(24.0 * screen_x, 24.0 * screen_y, 24.0, 24.0, 2.0, color);
    }

    pub fn get_texture(&self, set: TileSet) -> &Texture2D {
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
