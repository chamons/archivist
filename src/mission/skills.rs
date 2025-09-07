use crate::mission::*;
use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SkillTargeting {
    Caster,
    Ranged {
        max_range: u32,
        sprite: AnimationSpriteKind,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SkillCost {
    Will(i32),
    Charges { remaining: i32, total: i32 },
}

impl SkillCost {
    pub fn can_pay(&self, character: &Character) -> bool {
        match self {
            SkillCost::Will(cost) => character.will.has_enough(*cost),
            SkillCost::Charges { remaining, .. } => *remaining > 0,
        }
    }

    pub fn term(&self) -> &'static str {
        match self {
            SkillCost::Will(_) => "will",
            SkillCost::Charges { .. } => "charges",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
    pub cost: SkillCost,
    pub effect: Effect,
    pub targeting: SkillTargeting,
}
