use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CashValue {
    amount: f32
}

impl CashValue {
    pub fn get(&self) -> f32 {
        (self.amount * 100.0).round() / 100.0
    }
}