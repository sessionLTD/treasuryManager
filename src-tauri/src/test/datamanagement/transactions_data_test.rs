use chrono;

use crate::{commands::payer_commands::create_new_payer, database_management::{payer_data_manager::PayerDataManager, transaction_data_manager::TransactionDataManager}, models::{cashvalue::CashValue, requests::{payer_creation_request::PayerCreationRequest, transaction_creation_request::TransactionCreationRequest}}, start_up::setup::SetupManager, test::utils::create_empty_group};

#[test]
pub fn new_transaction_creation_test() {
    if SetupManager::requires_setup() {
        SetupManager::setup().expect("Cannot Setup");
    }
    let mut manager = TransactionDataManager::new();

    let group = create_empty_group(None);

    let payer_request = PayerCreationRequest::new("Tester".to_string(), "MacTester".to_string(), "28319083".to_string(), "e mai".to_string(), group.get_id().clone());
    let payer = create_new_payer(payer_request).expect("Cannot create new payer");
    PayerDataManager::save_new_payer(&payer, group.get_id()).expect("Cannot save payer");
    let now = chrono::offset::Utc::now().timestamp() as u64;
    let request = TransactionCreationRequest::new(CashValue::new_with_value(34.0), payer.get_id().clone(), group.get_id().clone(), now, "Testing if this works".to_string());

    let result = manager.new_transaction(&request);
    assert!(result.is_ok(), "{}", result.err().unwrap());
}