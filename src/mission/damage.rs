use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum DamageKind {
    Physical,
    Fire,
    Lightning,
    Ice,
    Poison,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Weapon {
    pub name: String,
    pub damage: i32,
    #[serde(default = "default_damage_kind")]
    pub kinds: Vec<DamageKind>,
}

fn default_damage_kind() -> Vec<DamageKind> {
    vec![DamageKind::Physical]
}
