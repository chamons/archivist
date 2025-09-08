use std::collections::{HashMap, HashSet};

use archivist::mission::TileSet;
use archivist::{
    mission::{Data, Screen},
    prelude::Point,
};
use macroquad::color::WHITE;
use macroquad::input::{KeyCode, is_key_pressed};
use macroquad::text::draw_text;
use macroquad::{
    color::BLACK,
    input::mouse_position,
    window::{Conf, clear_background},
};

fn window_conf() -> Conf {
    Conf {
        window_title: "Creature Sampler".to_string(),
        window_width: 1024,
        window_height: 800,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let data = Data::load().unwrap();

    let screen = Screen::new().await;

    let enemies = data.get_all_enemies();
    let tags: HashSet<String> = enemies
        .iter()
        .flat_map(|e| {
            let info = data.get_character_info(e);
            info.tags.iter().cloned().collect::<Vec<_>>()
        })
        .collect();
    let tags: Vec<String> = tags.into_iter().collect();

    let mut level_filter: Option<i32> = None;
    let mut tag_filter: Option<String> = None;

    loop {
        clear_background(BLACK);

        if is_key_pressed(KeyCode::Key1) {
            level_filter = Some(1);
        } else if is_key_pressed(KeyCode::Key2) {
            level_filter = Some(2);
        } else if is_key_pressed(KeyCode::Key3) {
            level_filter = Some(3);
        } else if is_key_pressed(KeyCode::Key0) {
            level_filter = None
        } else if is_key_pressed(KeyCode::Left) {
            if tag_filter.as_ref().is_none() {
                tag_filter = tags.last().cloned();
            } else {
                let index = tags.iter().position(|t| t == tag_filter.as_ref().unwrap());
                tag_filter = match index {
                    Some(index) => {
                        let index = index as i32 - 1;
                        if index < 0 {
                            None
                        } else {
                            tags.get(index as usize).cloned()
                        }
                    }
                    None => None,
                };
            }
        } else if is_key_pressed(KeyCode::Right) {
            if tag_filter.as_ref().is_none() {
                tag_filter = tags.first().cloned();
            } else {
                let index = tags.iter().position(|t| t == tag_filter.as_ref().unwrap());
                tag_filter = match index {
                    Some(index) => tags.get(index + 1).cloned(),
                    None => tags.first().cloned(),
                };
            }
        }

        let mut characters = HashMap::new();

        let mut count = 0;
        for enemy_name in &enemies {
            let enemy = data.get_character(enemy_name);
            let mut filter_pass = if let Some(level_filter) = level_filter {
                data.get_character_info(enemy_name).difficulty == Some(level_filter as u32)
            } else {
                true
            };
            if let Some(tag_filter) = &tag_filter {
                filter_pass &= data
                    .get_character_info(enemy_name)
                    .tags
                    .iter()
                    .any(|t| t == tag_filter)
            }

            if filter_pass {
                characters.insert(Point::new(count % 20 as i32, count / 20 as i32), enemy);
                count += 1;
            }
        }

        for i in 0..characters.len() {
            let position = Point::new(i as i32 % 20, i as i32 / 20);
            let enemy = characters.get(&position).unwrap();
            screen.draw_sprite(TileSet::Creatures, position, enemy.base_sprite_tile);
        }

        let mouse = mouse_position();
        let mouse_grid = Point::new(mouse.0 as i32 / 24, mouse.1 as i32 / 24);
        if let Some(moused_over) = characters.get(&mouse_grid) {
            draw_text(
                &format!("Name: {}", moused_over.name),
                50.0,
                400.0,
                20.0,
                WHITE,
            );
            draw_text(
                &format!("Health: {}", moused_over.health.max),
                50.0,
                420.0,
                20.0,
                WHITE,
            );
            draw_text(
                &format!("Will: {}", moused_over.will.max),
                50.0,
                440.0,
                20.0,
                WHITE,
            );
            draw_text(
                &format!("Damage: {}", moused_over.weapon.damage),
                50.0,
                460.0,
                20.0,
                WHITE,
            );
            draw_text(
                &format!("Sprite: {:?}", moused_over.base_sprite_tile),
                50.0,
                480.0,
                20.0,
                WHITE,
            );
            draw_text(
                &format!("Intelligent: {:?}", moused_over.enemy_memory.is_some()),
                50.0,
                500.0,
                20.0,
                WHITE,
            );
            if !moused_over.skills.is_empty() {
                draw_text("Skills", 50.0, 520.0, 20.0, WHITE);
            }
            for (i, skill) in moused_over.skills.iter().enumerate() {
                let offset = i as f32 * 20.0;
                draw_text(
                    &format!("{:?}", skill.name),
                    75.0,
                    540.0 + offset,
                    20.0,
                    WHITE,
                );
                draw_text(
                    &format!("{:?}", skill.effect),
                    75.0,
                    560.0 + offset,
                    20.0,
                    WHITE,
                );
                draw_text(
                    &format!("{:?}", skill.targeting),
                    75.0,
                    580.0 + offset,
                    20.0,
                    WHITE,
                );
            }
        }

        draw_text(&format!("Tags: {:?}", tags), 10.0, 730.0, 20.0, WHITE);
        if let Some(level_filter) = level_filter {
            draw_text(
                &format!("Level Filter: {}", level_filter),
                10.0,
                750.0,
                20.0,
                WHITE,
            );
        }
        if let Some(tag_filter) = &tag_filter {
            draw_text(
                &format!("Tag Filter: {}", tag_filter),
                10.0,
                770.0,
                20.0,
                WHITE,
            );
        }

        macroquad::window::next_frame().await
    }
}
