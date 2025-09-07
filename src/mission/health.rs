use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

impl Health {
    pub fn new(max: i32) -> Self {
        Self { current: max, max }
    }

    pub fn is_dead(&self) -> bool {
        self.current <= 0
    }

    pub fn increase(&mut self, amount: i32) {
        self.current = (self.current + amount).min(self.max);
    }

    pub fn percentage(&self) -> f32 {
        self.current as f32 / self.max as f32
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Will {
    pub current: i32,
    pub max: i32,
}

impl Will {
    pub fn new(max: i32) -> Self {
        Self { current: max, max }
    }

    pub fn has_enough(&self, cost: i32) -> bool {
        self.current >= cost
    }

    pub fn increase(&mut self, amount: i32) {
        self.current = (self.current + amount).min(self.max);
    }

    pub fn percentage(&self) -> f32 {
        self.current as f32 / self.max as f32
    }
}
