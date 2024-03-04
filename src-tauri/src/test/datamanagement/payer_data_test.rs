use crate::{database_management::{group_data_manager::GroupDataManager, payer_data_manager::PayerDataManager}, models::{payer::Payer, requests::group_creation_request::GroupCreationRequest}, start_up::setup::SetupManager, test::utils::cleanup_group};

#[test]
pub fn save_new_payer_test() {
    if SetupManager::requires_setup() {
        let _ = SetupManager::setup();
    }
    let payer = Payer::new(
        "Test".to_string(),
        "Payer".to_string(), 
        "0123456676".to_string(), 
        "test@tests.com".to_string()
    );

    let group_request = GroupCreationRequest{name: "Test group".to_string(), members: Some(vec![payer.get_id().clone()])};
    if let Ok(group) = GroupDataManager::save_new_group(group_request) {
        let group_id = group.get_id().clone();
        let result = PayerDataManager::save_new_payer(&payer, &group_id);
        assert!(result.is_ok());
        cleanup_group(&group_id);
    } else {
        panic!("Cannot create group!");
    }
}

#[test]
pub fn get_payer_data_test() {
    if SetupManager::requires_setup() {
        let _ = SetupManager::setup();
    }
    let payer = Payer::new(
        "Test".to_string(),
        "Payer".to_string(), 
        "0123456676".to_string(), 
        "test@tests.com".to_string()
    );

    let group_request = GroupCreationRequest{name: "Test group".to_string(), members: Some(vec![payer.get_id().clone()])};
    if let Ok(group) = GroupDataManager::save_new_group(group_request) {
        let group_id = group.get_id().clone();
        let result = PayerDataManager::save_new_payer(&payer, &group_id);
        assert!(result.is_ok());
        let gathered_payer = PayerDataManager::get_payer(payer.get_id(), &group_id);
        assert!(gathered_payer.is_ok());
        assert_eq!(gathered_payer.unwrap().get_id(), payer.get_id());
        cleanup_group(&group_id);
    } else {
        panic!("Cannot create Group!");
    }
}