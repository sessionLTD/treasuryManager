use std::fmt::Display;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{cashvalue::CashValue, group::GroupID, id_trait::ID, payer::PayerID, requests::transaction_creation_request::TransactionCreationRequest};

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

impl Display for TransactionID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Transaction {
    id: TransactionID,
    amount: CashValue,
    payer_id: PayerID,
    group_id: GroupID,
    reason: String,
    timestamp: u64,
}

impl Transaction {

    pub fn new_from_request(request: &TransactionCreationRequest) -> Self {
        Self {
            id: TransactionID::new(),
            amount: request.amount,
            payer_id: request.payer_id.clone(),
            group_id: request.group_id.clone(),
            reason: request.reason.clone(),
            timestamp: request.timestamp
        }
    }

    pub fn get_amount(&self) -> &CashValue {
        &self.amount
    }
}

