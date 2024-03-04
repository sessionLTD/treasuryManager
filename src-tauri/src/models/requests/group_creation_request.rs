use serde::{Deserialize, Serialize};

use crate::models::payer::PayerID;

#[derive(Deserialize, Serialize, Debug)]
pub struct GroupCreationRequest {
    pub name: String,
    pub members: Option<Vec<PayerID>>,
}