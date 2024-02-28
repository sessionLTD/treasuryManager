use crate::{commands::payer_commands::{create_new_payer, get_all_payers, get_payer}, models::requests::payer_creation_request::PayerCreationRequest, test::utils::cleanup_payer};

#[test]
pub fn save_new_payer_test() {
    let firstname = "Test".to_string();
    let request = PayerCreationRequest{
        firstname: firstname.clone(),
        lastname: "Payer".to_string(),
        telephone: "01234567889".to_string(),
        email: "test@testboy.com".to_string()
    };
    let result = create_new_payer(request);
    if result.is_ok() {
        let binding = result.unwrap();
        assert!(*binding.get_firstname() == firstname);
        cleanup_payer(binding.get_id());   
    } else {
        panic!("Creating a new payer was not successfull: {}", result.err().unwrap());
    }
}

#[test]
pub fn get_payer_data_test() {
    let firstname = "Test".to_string();
    let request = PayerCreationRequest{
        firstname: firstname.clone(),
        lastname: "Payer".to_string(),
        telephone: "01234567889".to_string(),
        email: "test@testboy.com".to_string()
    };

    let payer = create_new_payer(request).unwrap();
    let id = payer.get_id().clone();
    let result = get_payer(id);
    assert!(result.is_ok(), "{}", result.err().unwrap());
    assert_eq!(result.unwrap().get_id(), payer.get_id());
    cleanup_payer(payer.get_id())
}

#[test]
pub fn get_all_payers_test() {
    let first_payer = PayerCreationRequest{
        firstname: "First Boy".to_string(),
        lastname: "Payer".to_string(),
        telephone: "01234567889".to_string(),
        email: "test@testboy.com".to_string()
    };

    let second_payer = PayerCreationRequest{
        firstname: "First Boy".to_string(),
        lastname: "Payer".to_string(),
        telephone: "01234567889".to_string(),
        email: "test@testboy.com".to_string()
    };

    let first_expected = create_new_payer(first_payer).unwrap();
    let second_expected = create_new_payer(second_payer).unwrap();
    
    let actual = get_all_payers();
    cleanup_payer(first_expected.get_id());
    cleanup_payer(second_expected.get_id());
    assert!(actual.contains(&first_expected), "Doesn't contain first payer");
    assert!(actual.contains(&second_expected), "Doesn't contain second payer");
}