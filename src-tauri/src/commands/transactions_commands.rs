use crate::{models::{requests::transaction_creation_request::TransactionCreationRequest, transaction::Transaction}, TRANSACTION_DATA_MANAGER};

#[tauri::command]
pub fn create_new_transaction(request: TransactionCreationRequest) -> Result<Transaction, String> {
   unsafe {
       if let Ok(mut manager) = TRANSACTION_DATA_MANAGER.lock() {
            let transaction = manager.new_transaction(&request).map_err(|e| e.to_string())?;
            Ok(transaction)
       } else {
            Err("Cannot access transaction data manager. It seems to be already in use".to_string())
       }
   }
}