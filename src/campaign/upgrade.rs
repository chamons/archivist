use macroquad::{shapes::draw_rectangle_lines, text::draw_text};
use rand::seq::IteratorRandom;

use crate::{
    campaign::{CampaignState, CampaignStep},
    mission::{Data, Health, Will},
    prelude::*,
};

const STARTS_JSON: &str = include_str!("../../data/upgrades.json");

// This is a stub for a future crafting system
#[derive(Debug, Clone, Serialize, Deserialize)]
struct UpgradeOption {
    name: String,
    description: Vec<String>,
    #[serde(default)]
    added_health: u32,
    #[serde(default)]
    added_will: u32,
    #[serde(default)]
    added_damage: u32,
    #[serde(default)]
    provides_skills: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpgradeState {
    options: Vec<UpgradeOption>,

    campaign: CampaignState,
    selection: usize,
}

impl UpgradeState {
    pub fn new(campaign: CampaignState) -> Self {
        let upgrades: Vec<UpgradeOption> =
            serde_json::from_str(STARTS_JSON).expect("Unable to load upgrade choice data");

        let options = upgrades
            .into_iter()
            .filter(|u| !campaign.chosen_upgrades.contains(&u.name))
            .choose_multiple(&mut rand::rng(), 3);

        Self {
            options,
            campaign,
            selection: 0,
        }
    }

    pub fn process_frame(&mut self) -> Option<CampaignStep> {
        self.draw_upgrade_option(0);
        self.draw_upgrade_option(1);
        self.draw_upgrade_option(2);

        if is_key_pressed(KeyCode::Down) {
            if self.selection < 2 {
                self.selection += 1;
            }
        } else if is_key_pressed(KeyCode::Up) {
            if self.selection > 0 {
                self.selection -= 1;
            }
        } else if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::KpEnter) {
            self.upgrade_character();

            return Some(CampaignStep::MissionReady(self.campaign.clone()));
        }
        None
    }

    fn upgrade_character(&mut self) {
        let selection = &self.options[self.selection];

        if selection.added_damage > 0 {
            self.campaign.character.weapon.damage += selection.added_damage as i32;
        }
        if selection.added_health > 0 {
            self.campaign.character.health =
                Health::new(self.campaign.character.health.max + selection.added_health as i32);
        }
        if selection.added_will > 0 {
            self.campaign.character.will =
                Will::new(self.campaign.character.will.max + selection.added_will as i32);
        }
        if !selection.provides_skills.is_empty() {
            let data = Data::load().expect("Load data for upgrade");
            let mut new_skills = selection
                .provides_skills
                .iter()
                .map(|s| data.get_skill(s))
                .collect();
            self.campaign.character.skills.append(&mut new_skills);
        }
        self.campaign.chosen_upgrades.insert(selection.name.clone());
    }

    fn draw_upgrade_option(&mut self, index: usize) {
        let option = &self.options[index];
        let is_selected = self.selection == index;
        let top = 200.0 + 144.0 * index as f32;

        let border_color = if is_selected { WHITE } else { BROWN };
        draw_rectangle_lines(200.0, top, 600.0, 120.0, 3.0, border_color);
        draw_text(&option.name, 250.0, top + 35.0, 22.0, WHITE);

        for (i, line) in option.description.iter().enumerate() {
            draw_text(line, 250.0, top + 60.0 + (i as f32 * 15.0), 18.0, WHITE);
        }
    }
}
