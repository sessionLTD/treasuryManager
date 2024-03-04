use std::fmt::Display;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{id_trait::ID, payer::PayerID};
#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct Group {
    id: GroupID,
    name: String,
    members: Vec<PayerID>
}

impl Group {
    pub fn new(name: String, members: Option<Vec<PayerID>>) -> Self {
        let id = GroupID::new();
        if let Some(member_list) = members {
            Self{id, name, members: member_list}
        } else {
            Self{id, name, members: vec![]}
        }
    }

    pub fn get_id(&self) -> &GroupID {
        &self.id
    }

    pub fn get_members_mut(&mut self) -> &mut Vec<PayerID> {
        &mut self.members
    }

    pub fn get_members(&self) -> &Vec<PayerID> {
        &self.members
    }
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct GroupID(String);
impl ID for GroupID {
    fn new() -> Self {
        let id = Uuid::new_v4().to_string().replace('-', "");
        Self(id)
    }
    
    fn value(&self) -> &String {
        &self.0
    }
}

impl Display for GroupID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}