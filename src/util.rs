use crate::prelude::*;

use macroquad::{
    color::{Color, GREEN},
    math::Vec2,
};
use rand::{Rng, rngs::StdRng};
use serde::{Deserialize, Serialize};

pub trait RandExt {
    fn flip(&mut self) -> bool;
}

impl RandExt for StdRng {
    fn flip(&mut self) -> bool {
        self.random_bool(0.5)
    }
}

// Point and Rect in macroquad use f32
// but that doesn't make any sense for roguelike grids

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl adam_fov_rs::GridPoint for Point {
    fn xy(&self) -> adam_fov_rs::IVec2 {
        adam_fov_rs::IVec2 {
            x: self.x,
            y: self.y,
        }
    }
}

impl Into<Vec2> for Point {
    fn into(self) -> Vec2 {
        Vec2 {
            x: self.x as f32,
            y: self.y as f32,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rect {
    pub x1: i32,
    pub x2: i32,
    pub y1: i32,
    pub y2: i32,
}

impl Rect {
    pub fn new(x1: i32, x2: i32, y1: i32, y2: i32) -> Self {
        Self { x1, x2, y1, y2 }
    }

    pub fn with_size(x: i32, y: i32, w: i32, h: i32) -> Rect {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.x1 && point.x < self.x2 && point.y >= self.y1 && point.y < self.y2
    }

    pub fn center(&self) -> Point {
        Point::new((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }

    pub fn intersect(&self, other: &Rect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    pub fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(Point),
    {
        for y in self.y1..self.y2 {
            for x in self.x1..self.x2 {
                f(Point::new(x, y));
            }
        }
    }
}

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

    let mut player = data.get_character("Player");
    player.position = Point::new(1, 1);

    let mut bat = data.get_character("Bat");
    bat.position = Point::new(1, 5);
    let id = bat.id;

    let mut map = Map::new_filled();
    for y in 1..6 {
        map.set(
            Point::new(1, y),
            MapTile {
                kind: TileKind::Floor,
                known: true,
            },
        );
    }
    let level = LevelState::new(map, vec![player, bat]);
    (id, level)
}
