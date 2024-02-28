use serde::{Deserialize, Serialize};

use crate::models::{cashvalue::CashValue, payer::PayerID};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct TransactionCreationRequest {
    amount: CashValue,
    payer_id: PayerID,
    reason: String
}