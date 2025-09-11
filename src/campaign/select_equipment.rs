use macroquad::{
    shapes::draw_rectangle_lines,
    text::draw_text,
    texture::{DrawTextureParams, draw_texture_ex},
    window::screen_width,
};

use crate::{
    campaign::{CampaignState, CampaignStep},
    mission::{Character, CharacterId, Data, Health, Weapon, Will},
    prelude::*,
};

const STARTS_JSON: &str = include_str!("../../data/starts.json");

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EquipmentOption {
    name: String,
    description: Vec<String>,
    sprite: Point,
    weapon: Weapon,
    health: u32,
    will: u32,
    defense: u32,
    #[serde(default)]
    provides_skills: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectEquipmentState {
    options: Vec<EquipmentOption>,
    selection: usize,
    bounce: bool,
}

impl SelectEquipmentState {
    pub fn new() -> Self {
        let weapons =
            serde_json::from_str(STARTS_JSON).expect("Unable to load equipment choice data");
        Self {
            selection: 0,
            bounce: false,
            options: weapons,
        }
    }

    pub fn process_frame(&mut self, screen: &Screen, frame: usize) -> Option<CampaignStep> {
        if frame % BOUNCE_FRAME == 0 {
            self.bounce = !self.bounce;
        }

        self.draw_equipment_option(screen, 0);
        self.draw_equipment_option(screen, 1);
        self.draw_equipment_option(screen, 2);

        if is_key_pressed(KeyCode::Down) {
            if self.selection < 2 {
                self.selection += 1;
            }
        } else if is_key_pressed(KeyCode::Up) {
            if self.selection > 0 {
                self.selection -= 1;
            }
        } else if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::KpEnter) {
            return Some(CampaignStep::MissionReady(CampaignState::new(
                self.outfit_character(),
            )));
        }
        None
    }

    fn outfit_character(&self) -> Character {
        let selection = &self.options[self.selection];

        let data = Data::load().expect("Mission data should load");
        let mut skills: Vec<_> = selection
            .provides_skills
            .iter()
            .map(|s| data.get_skill(s))
            .collect();
        skills.push(data.get_skill("Health Potion"));

        Character {
            name: "Player".to_string(),
            position: Point::zero(),
            id: CharacterId::next(),
            ticks: TICKS_TO_ACT, // Start off ready to go, since first move is always player
            health: Health::new(selection.health as i32),
            will: Will::new(selection.will as i32),
            base_sprite_tile: selection.sprite,
            weapon: selection.weapon.clone(),
            skills,
            carried_items: vec![],
            enemy_memory: None,
            status_effects: vec![],
            defense: selection.defense as i32,
        }
    }

    fn draw_equipment_option(&mut self, screen: &Screen, index: usize) {
        let option = &self.options[index];
        let is_selected = self.selection == index;
        let top = 200.0 + 144.0 * index as f32;
        let left = (screen_width() - 600.0) / 2.0;

        let border_color = if is_selected { WHITE } else { BROWN };
        draw_rectangle_lines(left, top, 600.0, 120.0, 3.0, border_color);
        draw_text(&option.name, left + 100.0, top + 25.0, 22.0, WHITE);

        for (i, line) in option.description.iter().enumerate() {
            draw_text(
                line,
                left + 100.0,
                top + 50.0 + (i as f32 * 15.0),
                18.0,
                WHITE,
            );
        }

        let mut sprite = option.sprite.clone();
        if is_selected && self.bounce {
            sprite.y += 1;
        }
        let texture = screen.get_texture(TileSet::Creatures);

        draw_texture_ex(
            texture,
            left + 40.0,
            24.0 * (10.0 + 6.0 * index as f32),
            WHITE,
            DrawTextureParams {
                source: Some(MRect::new(
                    sprite.x as f32 * 24.0,
                    sprite.y as f32 * 24.0,
                    24.0,
                    24.0,
                )),
                ..Default::default()
            },
        );
    }
}
