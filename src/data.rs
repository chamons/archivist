use crate::prelude::*;

const CHARACTERS_JSON: &str = include_str!("../data/characters.json");

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct CharacterInfo {
    pub name: String,
    pub max_health: u32,
    pub difficulty: Option<u32>,
    pub base_sprite_tile: Point,
}

pub struct Data {
    characters: Vec<CharacterInfo>,
}

impl Data {
    pub fn load() -> Result<Self, serde_json::Error> {
        let enemies = serde_json::from_str(CHARACTERS_JSON)?;
        Ok(Self {
            characters: enemies,
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
            base_sprite_tile: character_info.base_sprite_tile,
        }
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
