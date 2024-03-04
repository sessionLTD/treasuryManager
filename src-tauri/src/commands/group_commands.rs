use crate::{database_management::group_data_manager::GroupDataManager, models::{group::{Group, GroupID}, payer::{Payer, PayerID}, requests::group_creation_request::GroupCreationRequest}};

#[tauri::command]
pub fn get_all_groups() -> Vec<Group> {
  GroupDataManager::get_all_groups()
}

#[tauri::command]
pub fn save_new_group(group_creation_request: GroupCreationRequest) -> Result<Group, String> {
  GroupDataManager::save_new_group(group_creation_request).map_err(|err| err.to_string())
}

#[tauri::command]
pub fn save_group(group: Group) -> Result<(), String> {
    GroupDataManager::save_group(&group).map_err(|e| format!("{}", e))
}

#[tauri::command]
pub fn add_payer_to_group(payer: Payer, group_id: GroupID) -> Result<Group, String> {
    GroupDataManager::add_payer_to_group(&payer, &group_id).map_err(|e| format!("{}", e))
}

#[tauri::command]
pub fn remove_payer_from_group(payer_id: PayerID, group_id: GroupID) -> Result<Group, String> {
    GroupDataManager::remove_payer_from_group(&payer_id, &group_id).map_err(|e| format!("{}", e))
}