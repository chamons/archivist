use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationInfo {
    pub path: Vec<Point>,
    pub sprite_tile: Point,
    pub action: RequestedAction,
    pub ticks: usize,
}

impl AnimationInfo {
    pub fn new(
        source: Point,
        target: Point,
        level: &LevelState,
        sprite_tile: Point,
        action: RequestedAction,
    ) -> Self {
        let mut path = path_between_points(source, target, level, PathCharacterOptions::All)
            .expect("Created animation but no path?");
        // Pop off the starting position
        path.remove(0);
        // And the end as well
        path.pop();

        Self {
            path,
            sprite_tile,
            action,
            ticks: ANIMATION_TICKS_PER_TILE,
        }
    }

    pub fn handle_input(&mut self) -> HandleInputResponse {
        self.ticks -= 1;
        if self.ticks == 0 {
            self.path.remove(0);
            self.ticks = ANIMATION_TICKS_PER_TILE;
        }

        if is_key_pressed(KeyCode::Escape) {
            self.path.clear();
        }

        if self.path.is_empty() {
            HandleInputResponse::Action(Some(self.action.clone()))
        } else {
            HandleInputResponse::Action(None)
        }
    }

    pub fn render(&self, screen: &Screen) {
        if let Some(position) = self.path.first() {
            screen.draw_sprite(TileSet::FX, *position, self.sprite_tile);
        }
    }
}
