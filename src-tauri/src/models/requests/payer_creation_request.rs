use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PayerCreationRequest {
    pub firstname: String,
    pub lastname: String,
    pub telephone: String,
    pub email: String
}