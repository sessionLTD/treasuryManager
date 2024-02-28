use crate::{database_management::payer_data_manager::PayerDataManager, models::payer::Payer, test::utils::cleanup_payer};

#[test]
pub fn save_new_payer_test() {
    let payer = Payer::new(
        "Test".to_string(),
        "Payer".to_string(), 
        "0123456676".to_string(), 
        "test@tests.com".to_string()
    );

    let result = PayerDataManager::save_new_payer(&payer);
    assert!(result.is_ok());
    cleanup_payer(payer.get_id());
}

#[test]
pub fn get_payer_data_test() {
    let payer = Payer::new(
        "Test".to_string(),
        "Payer".to_string(), 
        "0123456676".to_string(), 
        "test@tests.com".to_string()
    );

    let result = PayerDataManager::save_new_payer(&payer);
    assert!(result.is_ok());
    let gathered_payer = PayerDataManager::get_payer(payer.get_id());
    assert!(gathered_payer.is_ok());
    assert_eq!(gathered_payer.unwrap().get_id(), payer.get_id());
    cleanup_payer(payer.get_id())
}