use std::fmt::Display;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::{cashvalue::CashValue, id_trait::ID, transaction::Transaction};

#[derive(Deserialize, Serialize, Clone)]
pub struct TransactionBundleID(pub String);
impl ID for TransactionBundleID {
    fn new() -> Self {
        let id = Uuid::new_v4().to_string().replace('-', "");
        Self(id)
    }

    fn value(&self) -> &String {
        &self.0
    }
}

impl Display for TransactionBundleID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Deserialize, Serialize)]
pub struct TransactionBundle {
    id: TransactionBundleID,
    transactions: Vec<Transaction>,
    timestamp: u64,
    balance: CashValue
}

impl TransactionBundle {
    pub fn new(transactions: Vec<Transaction>, timestamp: u64) -> Self {
        Self {id: TransactionBundleID::new(), transactions, timestamp, balance: CashValue::new()}
    }

    pub fn get_id(&self) -> &TransactionBundleID {
        &self.id
    }

    pub fn get_transactions_mut(&mut self) -> &mut Vec<Transaction> {
        &mut self.transactions
    }

    pub fn get_timestamp(&self) -> &u64 {
        &self.timestamp
    }

    pub fn recalculate(&mut self) {
        let mut balance = CashValue::new();
        for transaction in &self.transactions {
            balance += *transaction.get_amount();
        }
        self.balance = balance;
    }
}