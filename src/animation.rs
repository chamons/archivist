use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationInfo {
    pub target: Point,
    pub sprite_tile: Point,
    pub action: RequestedAction,
}
