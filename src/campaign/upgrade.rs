use macroquad::{
    rand::ChooseRandom, shapes::draw_rectangle_lines, text::draw_text, window::screen_width,
};

use crate::{
    campaign::{CampaignState, CampaignStep, mission_ready::RuneKinds},
    mission::{Data, Health, StatusEffect, Will},
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
    added_defense: u32,
    #[serde(default)]
    provides_skills: Vec<String>,
    tags: Vec<RuneKinds>,
    #[serde(default)]
    pub eternal_status_effects: Vec<StatusEffect>,
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
            .collect::<Vec<_>>()
            .choose_multiple(3)
            .cloned()
            .collect();
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
        if selection.added_defense > 0 {
            self.campaign.character.defense =
                self.campaign.character.defense + selection.added_defense as i32;
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
        for effect in &selection.eternal_status_effects {
            self.campaign.character.status_effects.push(StatusEffect {
                name: effect.name.clone(),
                kind: effect.kind,
                duration: None,
                on_complete: None,
            });
        }
        self.campaign.chosen_upgrades.insert(selection.name.clone());
    }

    fn draw_upgrade_option(&mut self, index: usize) {
        let option = &self.options[index];
        let is_selected = self.selection == index;
        let top = 200.0 + 144.0 * index as f32;
        let left = (screen_width() - 600.0) / 2.0;

        let border_color = if is_selected { WHITE } else { BROWN };
        draw_rectangle_lines(left, top, 600.0, 120.0, 3.0, border_color);
        draw_text(&option.name, left + 50.0, top + 35.0, 22.0, WHITE);

        for (i, line) in option.description.iter().enumerate() {
            draw_text(
                line,
                left + 50.0,
                top + 60.0 + (i as f32 * 15.0),
                18.0,
                WHITE,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        campaign::upgrade::{STARTS_JSON, UpgradeOption},
        mission::Data,
    };

    #[test]
    fn can_parse_characters() {
        let upgrades: Vec<UpgradeOption> = serde_json::from_str(STARTS_JSON).unwrap();

        let data = Data::load().unwrap();
        let _skills: Vec<_> = upgrades
            .iter()
            .flat_map(|u| u.provides_skills.iter().map(|s| data.get_skill(s)))
            .collect();
    }
}
