use std::fmt::Display;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct PayerID(pub String);

impl Display for PayerID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Payer {
    firstname: String,
    lastname: String,
    telephone: String,
    email: String,
    id: PayerID
}

impl Payer {
    pub fn new(firstname: String, lastname: String, telephone: String, email: String) -> Self {
        let id = PayerID(Uuid::new_v4().to_string().replace('-', ""));
        Self { firstname, lastname, telephone, email, id }
    }

    pub fn get_firstname(&self) -> &String {
        &self.firstname
    }

    pub fn get_lastname(&self) -> &String {
        &self.lastname
    }

    pub fn get_email(&self) -> &String {
        &self.email
    }

    pub fn get_telephone(&self) -> &String {
        &self.telephone
    }

    pub fn get_id(&self) -> &PayerID {
        &self.id
    }
}


