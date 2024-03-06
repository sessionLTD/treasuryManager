use serde::{Deserialize, Serialize};

use crate::models::{cashvalue::CashValue, group::GroupID, payer::PayerID};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct TransactionCreationRequest {
    pub amount: CashValue,
    pub payer_id: PayerID,
    pub group_id: GroupID,
    pub timestamp: u64,
    pub reason: String
}

impl TransactionCreationRequest {
    #[cfg(test)]
    pub fn new(amount: CashValue, payer_id: PayerID, group_id: GroupID, timestamp: u64, reason: String) -> Self {
        Self {
            amount,
            payer_id,
            group_id,
            timestamp,
            reason
        }
    }
}