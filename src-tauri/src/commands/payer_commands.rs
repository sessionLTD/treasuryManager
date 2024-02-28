use crate::{database_management::payer_data_manager::PayerDataManager, models::{payer::{Payer, PayerID}, requests::payer_creation_request::PayerCreationRequest}};

#[tauri::command]
pub fn create_new_payer(payer: PayerCreationRequest) -> Result<Payer, String> {
    let payer = Payer::new(payer.firstname, payer.lastname, payer.telephone, payer.email);
    PayerDataManager::save_new_payer(&payer).map_err(|error| error.to_string())?;
    Ok(payer)
}

#[tauri::command]
pub fn get_payer(id: PayerID) -> Result<Payer, String> {
    PayerDataManager::get_payer(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_all_payers() -> Vec<Payer> {
    PayerDataManager::get_all_payers()
}