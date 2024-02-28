use std::fs;

use crate::{constants::PAYER_DATA_DIRECTORY, models::payer::PayerID};

pub fn cleanup_payer(id: &PayerID) {
    let path = format!("{}{}", PAYER_DATA_DIRECTORY, id);
    match fs::remove_file(path) {
        Ok(_) => (),
        Err(e) => println!("WARNING: Could not remove file for user {}: {}", id, e),
    };
}