use std::fmt::Display;

use uuid::Uuid;

use super::{cashvalue::CashValue, id_trait::ID, time_range::TimeRange, transaction::Transaction};

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

pub struct TransactionBundle {
    id: TransactionBundleID,
    transactions: Vec<Transaction>,
    time_range: TimeRange,
    balance: CashValue
}

impl TransactionBundle {
    pub fn new(transactions: Vec<Transaction>, time_range: TimeRange) -> Self {
        Self {id: TransactionBundleID::new(), transactions, time_range, balance: CashValue::new()}
    }

    pub fn get_id(&self) -> &TransactionBundleID {
        &self.id
    }

    pub fn get_transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }

    pub fn get_timerange(&self) -> &TimeRange {
        &self.time_range
    }
}