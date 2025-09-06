use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SkillTargeting {
    Caster,
    Ranged {
        max_range: u32,
        sprite: AnimationSpriteKind,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
    pub cost: i32,
    pub effect: Effect,
    pub targeting: SkillTargeting,
}
