use crate::prelude::*;

const CHARACTERS_JSON: &str = include_str!("../data/characters.json");
const SKILLS_JSON: &str = include_str!("../data/skills.json");
const ITEMS_JSON: &str = include_str!("../data/items.json");

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterInfo {
    pub name: String,
    pub max_health: u32,
    pub difficulty: Option<u32>,
    pub base_sprite_tile: Point,
    pub weapon: Weapon,
    #[serde(default)]
    pub max_will: u32,
    #[serde(default)]
    pub skills: Vec<String>,
}

pub struct Data {
    characters: Vec<CharacterInfo>,
    skills: Vec<Skill>,
    items: Vec<Item>,
}

impl Data {
    pub fn load() -> Result<Self, serde_json::Error> {
        let skills = serde_json::from_str(SKILLS_JSON)?;
        let characters = serde_json::from_str(CHARACTERS_JSON)?;
        let items = serde_json::from_str(ITEMS_JSON)?;
        Ok(Self {
            skills,
            characters,
            items,
        })
    }

    pub fn get_character(&self, name: &str) -> Character {
        let character_info = self
            .characters
            .iter()
            .find(|e| e.name == name)
            .expect(&format!("Unable to load character data for: {}", name));
        Character {
            name: character_info.name.clone(),
            position: Point::zero(),
            id: CharacterId::next(),
            ticks: 0,
            health: Health::new(character_info.max_health as i32),
            will: Will::new(character_info.max_will as i32),
            base_sprite_tile: character_info.base_sprite_tile,
            weapon: character_info.weapon.clone(),
            skills: character_info
                .skills
                .iter()
                .map(|s| self.get_skill(s))
                .collect(),
            carried_items: vec![],
        }
    }

    pub fn get_skill(&self, name: &str) -> Skill {
        self.skills
            .iter()
            .find(|s| s.name == name)
            .expect(&format!("Unable to find skill: {}", name))
            .clone()
    }

    pub fn get_item(&self, name: &str) -> Item {
        self.items
            .iter()
            .find(|i| i.name == name)
            .expect(&format!("Unable to find item: {}", name))
            .clone()
    }

    pub fn get_enemies_at_level(&self, difficulty: u32) -> Vec<String> {
        self.characters
            .iter()
            .filter(|c| c.difficulty == Some(difficulty))
            .map(|c| c.name.clone())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn can_parse_characters() {
        let data = Data::load().unwrap();
        let level_zero = data.get_enemies_at_level(0);
        for name in level_zero {
            let _ = data.get_character(&name);
        }
    }
}
