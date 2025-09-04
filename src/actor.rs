use adam_fov_rs::GridPoint;
use macroquad::input::{
    MouseButton, is_mouse_button_released, mouse_delta_position, mouse_position,
};

use crate::prelude::*;

pub fn handle_movement_key() -> Option<Point> {
    if is_key_pressed(KeyCode::Left) | is_key_pressed(KeyCode::Kp4) {
        Some(Point::new(-1, 0))
    } else if is_key_pressed(KeyCode::Right) | is_key_pressed(KeyCode::Kp6) {
        Some(Point::new(1, 0))
    } else if is_key_pressed(KeyCode::Up) | is_key_pressed(KeyCode::Kp8) {
        Some(Point::new(0, -1))
    } else if is_key_pressed(KeyCode::Down) | is_key_pressed(KeyCode::Kp2) {
        Some(Point::new(0, 1))
    } else if is_key_pressed(KeyCode::Kp1) {
        Some(Point::new(-1, 1))
    } else if is_key_pressed(KeyCode::Kp3) {
        Some(Point::new(1, 1))
    } else if is_key_pressed(KeyCode::Kp7) {
        Some(Point::new(-1, -1))
    } else if is_key_pressed(KeyCode::Kp9) {
        Some(Point::new(1, -1))
    } else {
        None
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BlinkInfo {
    Solid(usize),
    Blinking(usize),
}

impl BlinkInfo {
    pub fn tick(&mut self) -> bool {
        match self {
            BlinkInfo::Solid(ticks) => {
                *ticks -= 1;
                if *ticks == 0 {
                    *self = BlinkInfo::Blinking(TARGET_FRAME_PAUSE_WINDOW);
                }
                true
            }
            BlinkInfo::Blinking(ticks) => {
                *ticks -= 1;
                if *ticks == 0 {
                    *self = BlinkInfo::Solid(TARGET_FRAME_BLINK);
                }
                false
            }
        }
    }

    fn reset(&mut self) {
        *self = BlinkInfo::Solid(TARGET_FRAME_BLINK);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetingInfo {
    pub position: Point,
    pub blink: BlinkInfo,
}

impl TargetingInfo {
    pub fn new(position: Point) -> Self {
        Self {
            position,
            blink: BlinkInfo::Solid(TARGET_FRAME_BLINK),
        }
    }

    fn handle_input(
        &mut self,
        level: &LevelState,
        screen: &Screen,
        is_current_target_valid: bool,
    ) -> Option<RequestedAction> {
        if is_key_pressed(KeyCode::Escape) {
            Some(RequestedAction::CancelledTargeting)
        } else if is_key_pressed(KeyCode::Enter)
            || is_key_pressed(KeyCode::KpEnter)
            || is_mouse_button_released(MouseButton::Left)
        {
            if is_current_target_valid {
                Some(RequestedAction::Wait(level.get_player().id))
            } else {
                None
            }
        } else if let Some(movement_delta) = handle_movement_key() {
            self.set_position(self.position + movement_delta);
            None
        } else if mouse_delta_position().length() > 0.0 {
            let mouse = mouse_position();
            let x = (mouse.0 / 24.0).floor() as i32 + screen.camera.left_x;
            let y = (mouse.1 / 24.0).floor() as i32 + screen.camera.top_y;
            self.set_position(Point::new(x, y));
            None
        } else if is_key_pressed(KeyCode::Tab) {
            let player_position = level.get_player().position;
            let visibility = level.map.compute_visibility(player_position);
            let mut visible_enemies = level
                .characters
                .iter()
                .filter(|c| !c.is_player() && visibility.get(c.position))
                .collect::<Vec<_>>();
            visible_enemies.sort_by_key(|e| player_position.king_dist(e.position));

            let current_enemy = visible_enemies.iter().find(|e| e.position == self.position);
            // If we have an enemy targeted
            if current_enemy.is_some() {
                // And there is more than one
                if visible_enemies.len() > 1 {
                    if let Some(current_index) = visible_enemies
                        .iter()
                        .position(|e| e.position == self.position)
                    {
                        // Move to the next one closest to player, wrapping around if needed
                        if let Some(next_enemy) = visible_enemies.get(current_index + 1) {
                            self.set_position(next_enemy.position);
                        } else if let Some(first_enemy) = visible_enemies.first() {
                            self.set_position(first_enemy.position);
                        }
                    }
                }
            } else {
                // If not, pick closest one
                if let Some(enemy) = visible_enemies.first() {
                    self.set_position(enemy.position);
                }
            }

            None
        } else {
            None
        }
    }

    fn set_position(&mut self, point: Point) {
        self.position = point;
        self.blink.reset();
    }
}

// In a turn based game, only sometimes does the player get to move
// This contains what the current "thing takes it's turn" is
// which could be an animation for example
#[derive(Debug, Serialize, Deserialize)]
pub enum CurrentActor {
    PlayerStandardAction,
    PlayerTargeting(TargetingInfo),
    EnemyAction(CharacterId),
}

impl CurrentActor {
    pub fn act(&mut self, level: &LevelState, screen: &Screen) -> Option<RequestedAction> {
        match self {
            CurrentActor::PlayerStandardAction => {
                let player = level.get_player();
                get_player_action(player)
            }
            CurrentActor::PlayerTargeting(targeting_info) => {
                let is_current_target_valid = Self::is_current_target_valid(targeting_info, level);
                targeting_info.handle_input(level, screen, is_current_target_valid)
            }
            CurrentActor::EnemyAction(id) => Some(default_action(level, *id)),
        }
    }

    pub fn render(&mut self, screen: &Screen, level: &LevelState) {
        if let CurrentActor::PlayerTargeting(targeting_info) = self {
            let should_draw = targeting_info.blink.tick();

            if should_draw {
                let color = if Self::is_current_target_valid(&targeting_info, level) {
                    WHITE
                } else {
                    RED
                };
                screen.draw_targeting(targeting_info.position, color);
            }
        }
    }

    fn is_current_target_valid(targeting_info: &TargetingInfo, level: &LevelState) -> bool {
        let character_target_target = level.find_character_at_position(targeting_info.position);
        character_target_target.is_some() && !character_target_target.unwrap().is_player()
    }

    pub fn needs_to_wait(&self) -> bool {
        match self {
            CurrentActor::PlayerStandardAction => true,
            CurrentActor::PlayerTargeting(_) => true,
            CurrentActor::EnemyAction(_) => false,
        }
    }
}
