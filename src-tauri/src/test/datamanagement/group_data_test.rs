use std::{fs, path::Path};

use crate::{constants::{GROUP_DATA_DIRECTORY, GROUP_DATA_FILE, PAYER_DATA_DIRECTORY, TRANSACTION_DIRECTORY}, database_management::group_data_manager::GroupDataManager, models::{group::Group, id_trait::ID, payer::{Payer, PayerID}, requests::group_creation_request::GroupCreationRequest}, start_up::setup::SetupManager, test::utils::{cleanup_group, create_empty_group, create_filled_group}};

#[test]
pub fn create_group_test() {
    if SetupManager::requires_setup() {
        let _ = SetupManager::setup();
    }
    let group_request = GroupCreationRequest{name: "Test group".to_string(), members: Some(vec![PayerID::new()])};
    let result = GroupDataManager::save_new_group(group_request);
    assert!(result.is_ok(), "{}", result.err().unwrap().to_string());
    let group = result.ok().unwrap();
    let directory = format!("{}{}", GROUP_DATA_DIRECTORY, group.get_id().value());
    assert!(Path::new(&directory).exists(), "{}", format!("Directory doesn't exist: {}", directory));
    let payer_directory = format!("{}\\{}", directory, PAYER_DATA_DIRECTORY);
    assert!(Path::new(&payer_directory).exists(), "{}", format!("Directory doesn't exist: {}", payer_directory));
    let transaction_directory = format!("{}\\{}", directory, TRANSACTION_DIRECTORY);
    assert!(Path::new(&transaction_directory).exists(), "{}", format!("Directory doesn't exist: {}", transaction_directory));
    let group_data_file = format!("{}\\{}", directory, GROUP_DATA_FILE);
    assert!(Path::new(&group_data_file).exists(), "{}", format!("File doesn't exist: {}", group_data_file));
    let content = fs::read(group_data_file);
    assert!(content.is_ok());
    let group_from_file = ron::from_str::<Group>(unsafe { std::str::from_utf8_unchecked(content.unwrap().as_slice()) });
    assert!(group_from_file.is_ok());
    assert_eq!(group_from_file.unwrap().get_id(), group.get_id());
    cleanup_group(group.get_id());
}

#[test]
pub fn get_all_groups_test() {
    let group1 = create_empty_group(Some("First".to_string()));
    let group2 = create_empty_group(Some("Second".to_string()));

    let groups = GroupDataManager::get_all_groups();
    if !groups.contains(&group1) {
        panic!("Groups don't contain group 1!");
    }
    cleanup_group(group1.get_id());
    if !groups.contains(&group2) {
        panic!("Groups don't contain group 2!");
    }
    cleanup_group(group2.get_id());
}

#[test]
pub fn add_payer_to_group_test() {
    let group = create_empty_group(None);
    let payer = Payer::new("Tester".to_string(), "McTesting".to_string(), "012345677899".to_string(), "tests@theTestingFactory.com".to_string());

    let result = GroupDataManager::add_payer_to_group(&payer, group.get_id());
    cleanup_group(group.get_id());
    assert!(result.is_ok(), "{}", result.err().unwrap().to_string());
    let got_result =  result.ok().unwrap();
    assert!(got_result.get_members().contains(payer.get_id()));
}

#[test]
pub fn remove_payer_from_group_test() {
    let group = create_filled_group(None);
    let payer = Payer::new("Tester".to_string(), "McTesting".to_string(), "012345677899".to_string(), "tests@theTestingFactory.com".to_string());

    GroupDataManager::add_payer_to_group(&payer, group.get_id()).expect("Cannot add payer to group");
    let result = GroupDataManager::remove_payer_from_group(payer.get_id(), group.get_id());
    cleanup_group(group.get_id());
    assert!(result.is_ok(), "{}", format!("Cannot remove a payer from the group: {}", result.err().unwrap()));
    let binding = result.unwrap();
    let members = binding.get_members();
    assert!(!members.contains(payer.get_id()), "Payer is still inside the group!");
    assert!(members.len() == 1, "Too many payers removed!");
}