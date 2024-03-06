use std::{fs, path::Path};

use crate::{constants::GROUP_DATA_DIRECTORY, database_management::group_data_manager::GroupDataManager, models::{group::{Group, GroupID}, id_trait::ID, payer::PayerID, requests::group_creation_request::GroupCreationRequest}};

pub fn cleanup_group(group_id: &GroupID) {
    let directory = format!("{}{}", GROUP_DATA_DIRECTORY, group_id.value());
    if Path::new(&directory).exists() {
        match fs::remove_dir_all(&directory) {
            Ok(_) => (),
            Err(e) => println!("WARNING: Could not remove directory for group {}: {}", group_id, e),
        }
    }
}

pub fn create_empty_group(name: Option<String>) -> Group {
    let group_name = name.unwrap_or("Test Group".to_string());
    let group = GroupCreationRequest{name: group_name, members: None};
    let g = GroupDataManager::save_new_group(group);
    if let Ok(g) = g {
        g
    } else {
        let error = g.err().unwrap();
        panic!("Cannot create Group: {}", error);
    }
}

pub fn create_filled_group(name: Option<String>) -> Group {
    let group_name = name.unwrap_or("Test Group".to_string());
    let group = GroupCreationRequest{name: group_name, members: Some(vec![PayerID::new()])};
    let g = GroupDataManager::save_new_group(group);
    if let Ok(g) = g {
        g
    } else {
        let error = g.err().unwrap();
        panic!("Cannot create Group: {}", error);
    }
}