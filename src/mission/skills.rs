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
    None,
    Will(i32),
    Charges { remaining: i32, total: i32 },
    Cooldown { ticks: i32, cost: i32 },
}

impl SkillCost {
    pub fn can_pay(&self, character: &Character) -> bool {
        match self {
            SkillCost::None => true,
            SkillCost::Will(cost) => character.will.has_enough(*cost),
            SkillCost::Charges { remaining, .. } => *remaining > 0,
            SkillCost::Cooldown { ticks, .. } => *ticks == 0,
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
