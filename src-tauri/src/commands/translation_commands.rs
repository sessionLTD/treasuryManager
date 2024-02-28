use crate::i18n::translation::{Language, TranslationSerivce};

#[tauri::command]
pub fn get_language() -> Option<Language> {
    TranslationSerivce::get_language()
}

#[tauri::command]
pub fn get_dictionary(language: String) -> Option<String> {
    let lang = Language::from_identifier(&language);
    TranslationSerivce::get_dictionary(lang)
}

#[tauri::command]
pub fn set_language(language: String) {
    let lang = Language::from_identifier(&language);
    TranslationSerivce::set_language(lang);
}