use std::ops::{Add, AddAssign};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
pub struct CashValue {
    amount: f32
}

impl CashValue {
    pub fn get(&self) -> f32 {
        (self.amount * 100.0).round() / 100.0
    }

    pub fn new() -> Self {
        Self {amount: 0.0}
    }

    #[cfg(test)]
    pub fn new_with_value(value: f32) -> Self {
        let rounded_value = (value * 100.0).round() / 100.0;
        Self {amount: rounded_value}
    }
}

impl Add for CashValue {
    type Output = CashValue;

    fn add(self, rhs: Self) -> Self::Output {
        let new_amount = self.get() + rhs.get();
        Self{amount: new_amount}
    }
}

impl AddAssign for CashValue {
    fn add_assign(&mut self, rhs: Self) {
        let new_value = self.get() + rhs.get();
        self.amount = new_value;
    }
}