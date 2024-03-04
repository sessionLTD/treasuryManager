use crate::{database_management::payer_data_manager::PayerDataManager, models::{group::GroupID, payer::{Payer, PayerID}, requests::payer_creation_request::PayerCreationRequest}};

#[tauri::command]
pub fn create_new_payer(payer_request: PayerCreationRequest) -> Result<Payer, String> {
    let payer = Payer::new(payer_request.firstname, payer_request.lastname, payer_request.telephone, payer_request.email);
    PayerDataManager::save_new_payer(&payer, &payer_request.group_id).map_err(|error| error.to_string())?;
    Ok(payer)
}

#[tauri::command]
pub fn get_payer(id: PayerID, group_id: GroupID) -> Result<Payer, String> {
    PayerDataManager::get_payer(&id, &group_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_all_payers_of_group(group_id: GroupID) -> Vec<Payer> {
    PayerDataManager::get_all_payers_of_group(&group_id)
}