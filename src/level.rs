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

    pub fn render(&self, screen: &mut Screen, camera: &Camera) {
        for character in &self.characters {
            if camera.is_in_view(character.position) {
                character.render(screen);
            }
        }

        self.map.render(screen);

        self.render_hud(screen);
    }

    fn render_hud(&self, screen: &mut Screen) {
        let health = self.get_player().health.clone();

        let mut draw_batch = DrawBatch::new();
        draw_batch.target(ScreenLayer::Text.into());
        draw_batch.print_centered(1, "Explore the Dungeon. Cursor keys to move.");
        draw_batch.bar_horizontal(
            Point::zero(),
            SCREEN_WIDTH * 2,
            health.current,
            health.max,
            ColorPair::new(RED, BLACK),
        );
        draw_batch.print_color_centered(
            0,
            format!(" Health: {} / {} ", health.current, health.max),
            ColorPair::new(WHITE, RED),
        );
        draw_batch.submit(0).expect("Batch error");

        render_draw_buffer(screen.ctx).expect("Render error");
    }
}
