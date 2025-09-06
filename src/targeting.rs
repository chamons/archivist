use adam_fov_rs::GridPoint;
use macroquad::input::{
    MouseButton, is_mouse_button_released, mouse_delta_position, mouse_position,
};

use crate::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlinkInfo {
    Solid(usize),
    Blinking(usize),
}

impl BlinkInfo {
    pub fn tick(&mut self) {
        match self {
            BlinkInfo::Solid(ticks) => {
                *ticks -= 1;
                if *ticks == 0 {
                    *self = BlinkInfo::Blinking(TARGET_FRAME_PAUSE_WINDOW);
                }
            }
            BlinkInfo::Blinking(ticks) => {
                *ticks -= 1;
                if *ticks == 0 {
                    *self = BlinkInfo::Solid(TARGET_FRAME_BLINK);
                }
            }
        }
    }

    pub fn should_draw(&self) -> bool {
        match self {
            BlinkInfo::Solid(_) => true,
            BlinkInfo::Blinking(_) => false,
        }
    }

    fn reset(&mut self) {
        *self = BlinkInfo::Solid(TARGET_FRAME_BLINK);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetEffect {
    pub effect: Effect,
    pub cost: i32,
    pub spite: AnimationSpriteKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetingInfo {
    pub position: Point,
    pub blink: BlinkInfo,
    pub effect: TargetEffect,
}

impl TargetingInfo {
    pub fn new(position: Point, effect: TargetEffect) -> Self {
        Self {
            position,
            effect,
            blink: BlinkInfo::Solid(TARGET_FRAME_BLINK),
        }
    }

    pub fn handle_input(
        &mut self,
        level: &LevelState,
        screen: &Screen,
        is_current_target_valid: bool,
    ) -> HandleInputResponse {
        self.tick();

        if is_key_pressed(KeyCode::Escape) {
            HandleInputResponse::ChangeActor(CurrentActor::PlayerStandardAction)
        } else if is_key_pressed(KeyCode::Enter)
            || is_key_pressed(KeyCode::KpEnter)
            || is_mouse_button_released(MouseButton::Left)
        {
            if is_current_target_valid {
                if let Some(target) = level.find_character_at_position(self.position) {
                    let player = level.get_player();
                    HandleInputResponse::ChangeActor(CurrentActor::Animation(AnimationInfo::new(
                        player.position,
                        target.position,
                        level,
                        self.effect.spite.clone(),
                        RequestedAction::UseEffect {
                            source: level.get_player().id,
                            target: target.id,
                            effect: self.effect.effect.clone(),
                            cost: self.effect.cost,
                        },
                    )))
                } else {
                    HandleInputResponse::Action(None)
                }
            } else {
                HandleInputResponse::Action(None)
            }
        } else if let Some(movement_delta) = handle_movement_key() {
            self.set_position(self.position + movement_delta);
            HandleInputResponse::Action(None)
        } else if mouse_delta_position().length() > 0.0 {
            let mouse = mouse_position();
            let x = (mouse.0 / 24.0).floor() as i32 + screen.camera.left_x;
            let y = (mouse.1 / 24.0).floor() as i32 + screen.camera.top_y;
            self.set_position(Point::new(x, y));
            HandleInputResponse::Action(None)
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

            HandleInputResponse::Action(None)
        } else {
            HandleInputResponse::Action(None)
        }
    }

    pub fn tick(&mut self) {
        self.blink.tick();
    }

    pub fn render(&self, screen: &Screen, level: &LevelState) {
        if self.blink.should_draw() {
            let color = if CurrentActor::is_current_target_valid(&self, level) {
                WHITE
            } else {
                RED
            };
            screen.draw_targeting(self.position, color);
        }
    }

    fn set_position(&mut self, point: Point) {
        self.position = point;
        self.blink.reset();
    }
}
