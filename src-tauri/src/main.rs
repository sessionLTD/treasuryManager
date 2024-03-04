// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use start_up::setup::SetupManager;

mod treasury_management;
mod models;
mod start_up;
mod commands;
mod database_management;
mod constants;
mod i18n;
#[cfg(test)]
mod test;

use commands::payer_commands::{create_new_payer, get_payer, get_all_payers_of_group};
use commands::translation_commands::{get_language, get_dictionary, set_language};
use commands::group_commands::{get_all_groups, save_group, save_new_group, add_payer_to_group, remove_payer_from_group};

fn main() {

    if SetupManager::requires_setup() {
        match SetupManager::setup() {
            Ok(_) => (),
            Err(error) => println!("Error while setting up programm: {}", error),
        };
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            create_new_payer,
            get_payer,
            get_all_payers_of_group,
            get_language,
            get_dictionary,
            set_language,
            get_all_groups,
            save_group,
            save_new_group,
            add_payer_to_group,
            remove_payer_from_group
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
