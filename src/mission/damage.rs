use serde::{Deserialize, Serialize};

use crate::mission::Effect;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy, Default)]
pub enum DamagePierce {
    #[default]
    None,
    Some,
    Full,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Weapon {
    pub name: String,
    pub damage: i32,
    #[serde(default)]
    pub on_hit: Option<Effect>,
    #[serde(default)]
    pub pierce: DamagePierce,
}
