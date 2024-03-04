use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{cashvalue::CashValue, id_trait::ID, payer::PayerID};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TransactionID(String);

impl ID for TransactionID {
    fn new() -> Self {
        let id = Uuid::new_v4().to_string().replace('-', "");
        Self(id)
    }
    
    fn value(&self) -> &String {
        &self.0
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Transaction {
    id: TransactionID,
    amount: CashValue,
    payer_id: Option<PayerID>,
    reason: String,
    timestamp: u64,
}

impl Transaction {
    pub fn get_id(&self) -> &TransactionID {
        &self.id
    }

    pub fn get_amount(&self) -> &CashValue {
        &self.amount
    }

    pub fn get_payer_id(&self) -> Option<&PayerID> {
        self.payer_id.as_ref()
    }

    pub fn get_reason(&self) -> &String {
        &self.reason
    }

    pub fn set_reason(&mut self, reason: String) {
        self.reason = reason;
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }
}

