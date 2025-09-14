use crate::mission::*;
use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnimationSpriteKind {
    SingleFrame(Point),
    Directional(Point),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnimationInfo {
    pub path: Vec<Point>,
    pub kind: AnimationSpriteKind,
    pub action: RequestedAction,
    pub ticks: usize,
    pub direction_offset: i32,
    pub is_player: bool,
}

impl AnimationInfo {
    pub fn new(
        source: Point,
        target: Point,
        level: &LevelState,
        kind: AnimationSpriteKind,
        action: RequestedAction,
        is_player: bool,
    ) -> Self {
        let mut path = path_between_points(source, target, level, PathCharacterOptions::All)
            .expect("Created animation but no path?");

        // Special case if you use a ranged to target yourself
        if path.len() == 1 {
            return Self {
                path: vec![],
                kind,
                action,
                ticks: ANIMATION_TICKS_PER_TILE,
                direction_offset: 0,
                is_player,
            };
        }

        let direction_offset = Self::calculate_animation_direction_offset(path[0], path[1]);
        // Pop off the starting position
        path.remove(0);
        // And the end as well
        path.pop();

        Self {
            path,
            kind,
            action,
            ticks: ANIMATION_TICKS_PER_TILE,
            direction_offset,
            is_player,
        }
    }

    fn update_animation_direction_offset(&mut self) {
        // If we have two points, calculate based on them else 0
        self.direction_offset = match (self.path.get(0), self.path.get(1)) {
            (Some(first), Some(second)) => {
                Self::calculate_animation_direction_offset(*first, *second)
            }
            _ => self.direction_offset,
        }
    }

    fn calculate_animation_direction_offset(first: Point, second: Point) -> i32 {
        let delta = second - first;
        match (delta.x, delta.y) {
            (x, y) if x > 0 && y > 0 => 4,
            (x, y) if x > 0 && y == 0 => 0,
            (x, y) if x > 0 && y < 0 => 7,
            (x, y) if x < 0 && y > 0 => 5,
            (x, y) if x < 0 && y == 0 => 2,
            (x, y) if x < 0 && y < 0 => 6,
            (x, y) if x == 0 && y < 0 => 1,
            (x, y) if x == 0 && y > 0 => 3,
            _ => 0,
        }
    }

    pub fn handle_input(&mut self) -> HandleInputResponse {
        self.ticks -= 1;
        if self.ticks == 0 {
            self.path.remove(0);
            self.update_animation_direction_offset();
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
            screen.draw_sprite(TileSet::FX, *position, self.get_sprite());
        }
    }

    fn get_sprite(&self) -> Point {
        match self.kind {
            AnimationSpriteKind::SingleFrame(point) => point,
            AnimationSpriteKind::Directional(point) => {
                Point::new(point.x + self.direction_offset, point.y)
            }
        }
    }
}
