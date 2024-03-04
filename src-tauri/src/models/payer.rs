use std::fmt::Display;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::id_trait::ID;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct PayerID(String);

impl ID for PayerID {
    fn new() -> Self {
        let id = Uuid::new_v4().to_string().replace('-', "");
        Self(id)
    }
    
    fn value(&self) -> &String {
        &self.0
    }
}

impl Display for PayerID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Payer {
    firstname: String,
    lastname: String,
    telephone: String,
    email: String,
    needs_to_pay: bool,
    id: PayerID
}

impl Payer {
    pub fn new(firstname: String, lastname: String, telephone: String, email: String) -> Self {
        Self { firstname, lastname, telephone, email, needs_to_pay: false, id: PayerID::new() }
    }
    #[cfg(test)]
    pub fn get_firstname(&self) -> &String {
        &self.firstname
    }

    pub fn get_id(&self) -> &PayerID {
        &self.id
    }
}


