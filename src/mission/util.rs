use crate::mission::*;
use crate::prelude::*;

use macroquad::color::{Color, GREEN};

pub fn color_for_will(percentage: f32) -> Color {
    match percentage {
        x if x > 0.8 => Color {
            r: 0.28,
            g: 0.64,
            b: 1.0,
            a: 1.0,
        },
        x if x > 0.6 => BLUE,
        x if x > 0.3 => Color {
            r: 0.0,
            g: 0.36,
            b: 0.72,
            a: 1.0,
        },
        _ => Color {
            r: 0.0,
            g: 0.28,
            b: 0.56,
            a: 1.0,
        },
    }
}

pub fn color_for_health(percentage: f32) -> Color {
    match percentage {
        x if x > 0.8 => GREEN,
        x if x > 0.6 => YELLOW,
        x if x > 0.3 => ORANGE,
        _ => RED,
    }
}

pub fn handle_movement_key() -> Option<Point> {
    if is_key_pressed(KeyCode::Left) | is_key_pressed(KeyCode::Kp4) | is_key_pressed(KeyCode::H) {
        Some(Point::new(-1, 0))
    } else if is_key_pressed(KeyCode::Right)
        | is_key_pressed(KeyCode::Kp6)
        | is_key_pressed(KeyCode::L)
    {
        Some(Point::new(1, 0))
    } else if is_key_pressed(KeyCode::Up)
        | is_key_pressed(KeyCode::Kp8)
        | is_key_pressed(KeyCode::K)
    {
        Some(Point::new(0, -1))
    } else if is_key_pressed(KeyCode::Down)
        | is_key_pressed(KeyCode::Kp2)
        | is_key_pressed(KeyCode::J)
    {
        Some(Point::new(0, 1))
    } else if is_key_pressed(KeyCode::Kp1) | is_key_pressed(KeyCode::B) {
        Some(Point::new(-1, 1))
    } else if is_key_pressed(KeyCode::Kp3) | is_key_pressed(KeyCode::N) {
        Some(Point::new(1, 1))
    } else if is_key_pressed(KeyCode::Kp7) | is_key_pressed(KeyCode::Y) {
        Some(Point::new(-1, -1))
    } else if is_key_pressed(KeyCode::Kp9) | is_key_pressed(KeyCode::U) {
        Some(Point::new(1, -1))
    } else {
        None
    }
}

pub fn handle_move_bump(actor: &Character, dest: Point, level: &LevelState) -> RequestedAction {
    if let Some(target) = level.find_character_at_position(dest) {
        RequestedAction::WeaponAttack {
            source: actor.id,
            target: target.id,
            weapon: actor.weapon.clone(),
        }
    } else {
        RequestedAction::Move(actor.id, dest)
    }
}

#[cfg(test)]
pub fn create_test_map() -> (CharacterId, LevelState) {
    let data = Data::load().unwrap();

    let mut player = data.get_character("Bat");
    player.name = "Player".to_string();
    player.position = Point::new(1, 1);

    let mut bat = data.get_character("Bat");
    bat.position = Point::new(1, 5);
    let id = bat.id;

    let mut map = Map::new_filled(MapTheme::Stone);
    for y in 1..6 {
        map.set(
            Point::new(1, y),
            MapTile {
                kind: TileKind::Floor,
                known: true,
                variation: 0,
            },
        );
    }
    let level = LevelState::new(map, vec![player, bat], vec![]);
    (id, level)
}
