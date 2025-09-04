use macroquad::{shapes::draw_rectangle, window::screen_width};

use crate::prelude::*;

#[derive(Debug)]
pub struct LevelState {
    pub map: Map,
    pub characters: Vec<Character>,
}

impl LevelState {
    pub fn get_player(&self) -> &Character {
        self.characters
            .iter()
            .find(|c| c.is_player())
            .expect("Player must still exist")
    }

    pub fn find_character(&self, id: CharacterId) -> &Character {
        self.characters
            .iter()
            .find(|c| c.id == id)
            .expect("Action actor exists")
    }

    pub fn find_character_mut(&mut self, id: CharacterId) -> &mut Character {
        self.characters
            .iter_mut()
            .find(|c| c.id == id)
            .expect("Action actor exists")
    }

    pub fn find_character_at_position(&self, position: Point) -> Option<&Character> {
        self.characters.iter().find(|c| c.position == position)
    }

    pub fn character_can_enter(&self, point: Point) -> bool {
        self.map.in_bounds(point) && self.map.get(point) == TileKind::Floor
    }

    pub fn remove_character(&mut self, id: CharacterId) {
        self.characters.retain(|c| c.id != id);
    }

    pub fn render(&self, screen: &mut Screen) {
        self.map.render(screen);

        for character in &self.characters {
            if screen.camera.is_in_view(character.position) {
                character.render(screen);
            }
        }

        self.render_hud();
        screen.render_floating_text();
    }

    fn render_hud(&self) {
        let health = self.get_player().health.clone();
        let health_percentage = health.current as f32 / health.max as f32;

        const LIFE_PADDING_X: f32 = 4.0;
        const LIFE_PADDING_Y: f32 = 2.0;
        draw_rectangle(0.0, 0.0, screen_width(), 18.0, BLACK);
        draw_rectangle(
            LIFE_PADDING_X,
            LIFE_PADDING_Y,
            (screen_width() - LIFE_PADDING_X * 2.0) * health_percentage,
            16.0,
            color_for_health(health_percentage),
        );

        Screen::draw_centered_text(
            &format!("{}/{}", health.current, health.max),
            17,
            15.0,
            None,
        );
    }
}
