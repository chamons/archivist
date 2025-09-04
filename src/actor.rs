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

    fn handle_input(&mut self, level: &LevelState) -> Option<RequestedAction> {
        if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::KpEnter) {
            Some(RequestedAction::Wait(level.get_player().id))
        } else if let Some(movement_delta) = handle_movement_key() {
            self.position = self.position + movement_delta;
            self.blink.reset();
            None
        } else {
            None
        }
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
    pub fn act(&mut self, level: &LevelState) -> Option<RequestedAction> {
        match self {
            CurrentActor::PlayerStandardAction => {
                let player = level.get_player();
                get_player_action(player)
            }
            CurrentActor::PlayerTargeting(target) => target.handle_input(level),
            CurrentActor::EnemyAction(id) => Some(default_action(level, *id)),
        }
    }

    pub fn render(&mut self, screen: &Screen) {
        if let CurrentActor::PlayerTargeting(targeting_info) = self {
            let should_draw = targeting_info.blink.tick();

            if should_draw {
                screen.draw_targeting(targeting_info.position);
            }
        }
    }

    pub fn needs_to_wait(&self) -> bool {
        match self {
            CurrentActor::PlayerStandardAction => true,
            CurrentActor::PlayerTargeting(_) => true,
            CurrentActor::EnemyAction(_) => false,
        }
    }
}
