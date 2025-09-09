use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatusEffectKind {
    Might,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusEffect {
    pub name: String,
    pub kind: StatusEffectKind,
    pub duration: i32,
}

impl StatusEffect {
    pub fn is_positive(&self) -> bool {
        match self.kind {
            StatusEffectKind::Might => true,
        }
    }
}
