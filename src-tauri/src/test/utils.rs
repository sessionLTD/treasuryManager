use std::{fs, path::Path};

use crate::{constants::GROUP_DATA_DIRECTORY, models::{group::GroupID, id_trait::ID}};

pub fn cleanup_group(group_id: &GroupID) {
    let directory = format!("{}{}", GROUP_DATA_DIRECTORY, group_id.value());
    if Path::new(&directory).exists() {
        match fs::remove_dir_all(&directory) {
            Ok(_) => (),
            Err(e) => println!("WARNING: Could not remove directory for group {}: {}", group_id, e),
        }
    }
}