use serde::{Deserialize, Serialize};

use crate::models::group::GroupID;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PayerCreationRequest {
    pub firstname: String,
    pub lastname: String,
    pub telephone: String,
    pub email: String,
    pub group_id: GroupID
}

impl PayerCreationRequest {
    #[cfg(test)]
    pub fn new(firstname: String, lastname: String, telephone: String, email: String, group_id: GroupID) -> Self {
        Self {
            firstname,
            lastname,
            telephone,
            email,
            group_id
        }
    }
}