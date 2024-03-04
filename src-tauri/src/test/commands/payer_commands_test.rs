use crate::{commands::payer_commands::{create_new_payer, get_all_payers_of_group, get_payer}, database_management::group_data_manager::GroupDataManager, models::requests::{group_creation_request::GroupCreationRequest, payer_creation_request::PayerCreationRequest}, test::utils::cleanup_group};

#[test]
pub fn save_new_payer_test() {
    let group = GroupCreationRequest{name: "Test Group".to_string(), members: None};
    if let Ok(group) = GroupDataManager::save_new_group(group) {
        let firstname = "Test".to_string();
        let request = PayerCreationRequest{
            firstname: firstname.clone(),
            lastname: "Payer".to_string(),
            telephone: "01234567889".to_string(),
            email: "test@testboy.com".to_string(),
            group_id: group.get_id().clone()
        };
        let result = create_new_payer(request);
        if result.is_ok() {
            let binding = result.unwrap();
            assert!(*binding.get_firstname() == firstname);
            cleanup_group(group.get_id());   
        } else {
            panic!("Creating a new payer was not successfull: {}", result.err().unwrap());
        }
    } else {
        panic!("Cannot create group!");
    }

}

#[test]
pub fn get_payer_data_test() {
    let group_creation_request = GroupCreationRequest{name: "Test Group".to_string(), members: None};

    if let Ok(group) = GroupDataManager::save_new_group(group_creation_request) {
        let firstname = "Test".to_string();
        let request = PayerCreationRequest{
            firstname: firstname.clone(),
            lastname: "Payer".to_string(),
            telephone: "01234567889".to_string(),
            email: "test@testboy.com".to_string(),
            group_id: group.get_id().clone()
        };
    
        let payer = create_new_payer(request).unwrap();
        let id = payer.get_id().clone();
        let result = get_payer(id, group.get_id().clone());
        assert!(result.is_ok(), "{}", result.err().unwrap());
        assert_eq!(result.unwrap().get_id(), payer.get_id());
        cleanup_group(group.get_id());
    } else {
        panic!("Cannot create group!");
    }
}

#[test]
pub fn get_all_payers_test() {
    let group = GroupCreationRequest{name:"Test Group".to_string(), members: None};
    if let Ok(group) = GroupDataManager::save_new_group(group) {

        let first_payer = PayerCreationRequest{
            firstname: "First Boy".to_string(),
            lastname: "Payer".to_string(),
            telephone: "01234567889".to_string(),
            email: "test@testboy.com".to_string(),
            group_id: group.get_id().clone()
        };
    
        let second_payer = PayerCreationRequest{
            firstname: "First Boy".to_string(),
            lastname: "Payer".to_string(),
            telephone: "01234567889".to_string(),
            email: "test@testboy.com".to_string(),
            group_id: group.get_id().clone()
        };
    
        let first_expected = create_new_payer(first_payer).unwrap();
        let second_expected = create_new_payer(second_payer).unwrap();
        
        let actual = get_all_payers_of_group(group.get_id().clone());
        cleanup_group(group.get_id());
        assert!(actual.contains(&first_expected), "Doesn't contain first payer");
        assert!(actual.contains(&second_expected), "Doesn't contain second payer");
    } else {
        panic!("Cannot create group!");
    }

}