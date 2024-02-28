use crate::{commands::translation_commands::{get_dictionary, get_language}, start_up::setup::SetupManager};

#[test]
pub fn get_language_test() {
    if SetupManager::requires_setup() {
        SetupManager::setup().expect("Setup didn't work. Aborting");
    }
    let language = get_language();
    assert!(language.is_some())
}

#[test]
pub fn get_current_dictionary_test() {
    if SetupManager::requires_setup() {
        SetupManager::setup().expect("Setup didn't work. Aborting");
    }
    let language = get_language();
    assert!(language.is_some());
    let dictionary = get_dictionary(language.unwrap().identifier().to_string());
    assert!(dictionary.is_some());
}