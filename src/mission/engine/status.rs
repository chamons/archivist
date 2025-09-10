use crate::{mission::Effect, prelude::*};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusEffectCompleteEffect {
    pub reapply_count: usize,
    pub complete_effect: Option<Box<Effect>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatusEffectKind {
    Might,
    Protection,
    Weakness,
    Quick,
    Lifesteal,
    Blessed,
    Cursed,
    RepeatingPositive,
    RepeatingNegative,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusEffect {
    pub name: String,
    pub kind: StatusEffectKind,
    #[serde(default)]
    pub duration: Option<i32>,
    #[serde(default)]
    pub on_complete: Option<StatusEffectCompleteEffect>,
}

impl StatusEffect {
    pub fn is_positive(&self) -> bool {
        match self.kind {
            StatusEffectKind::Might => true,
            StatusEffectKind::Protection => true,
            StatusEffectKind::Quick => true,
            StatusEffectKind::Lifesteal => true,
            StatusEffectKind::Blessed => true,
            StatusEffectKind::Cursed => false,
            StatusEffectKind::Weakness => false,
            StatusEffectKind::RepeatingPositive => true,
            StatusEffectKind::RepeatingNegative => false,
        }
    }

    pub fn tick(&mut self, amount: i32) {
        if let Some(duration) = &mut self.duration {
            *duration -= amount;
        }
    }
}
