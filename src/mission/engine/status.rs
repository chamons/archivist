use crate::{mission::Effect, prelude::*};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusEffectCompleteEffect {
    pub reapply_count: usize,
    pub complete_effect: Option<Box<Effect>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatusEffectKind {
    Might,
    Protection,
    RepeatingPositive,
    RepeatingNegative,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusEffect {
    pub name: String,
    pub kind: StatusEffectKind,
    pub duration: i32,
    pub on_complete: Option<StatusEffectCompleteEffect>,
}

impl StatusEffect {
    pub fn is_positive(&self) -> bool {
        match self.kind {
            StatusEffectKind::Might => true,
            StatusEffectKind::Protection => true,
            StatusEffectKind::RepeatingPositive => true,
            StatusEffectKind::RepeatingNegative => false,
        }
    }

    pub fn tick(&mut self, amount: i32) {
        self.duration -= amount;
    }
}
