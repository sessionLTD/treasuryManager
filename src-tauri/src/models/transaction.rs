use serde::{Deserialize, Serialize};

use super::{cashvalue::CashValue, payer::PayerID};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TransactionID(String);

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Transaction {
    id: TransactionID,
    amount: CashValue,
    payer_id: Option<PayerID>,
    reason: String,
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
}

