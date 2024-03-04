use std::{fs::{self, File}, io::{BufRead, Write}, str};

use serde::{Deserialize, Serialize};

use crate::constants::PROGRAM_PROPERTIES;

use super::languages::{ENGLISH, GERMAN, NORWEGIAN};

const LANGUAGE_IDENTIFIER: &str = "language";
const TRANSLATIONS_PATH: &str = "translations\\";

pub struct TranslationSerivce;

impl TranslationSerivce {
    pub fn get_language() -> Option<Language> {
        if let Ok(content) = fs::read(PROGRAM_PROPERTIES) {
            for line in content.lines().flatten() {
                if line.starts_with(LANGUAGE_IDENTIFIER) {
                    let split: Vec<&str> = line.split(':').collect();
                    let language_identifier = split[1];
                    return Some(Language::from_identifier(language_identifier));
                }
            }
            return None;
        }
        None
    }

    pub fn get_dictionary(language: Language) -> Option<String> {
        let path = format!("{}{}.json", TRANSLATIONS_PATH, language.identifier());
        if let Ok(content) = fs::read(path) {
            match str::from_utf8(content.as_slice()) {
                Ok(c) => Some(c.to_string()),
                Err(e) => {
                    println!("{}", e);
                    None
                }
            }
        } else {
            None
        }
    }

    pub fn set_language(language: Language) {
        let mut file = File::open(PROGRAM_PROPERTIES).expect("Cannot open program properties");
        let language_property = format!("{}:{}", LANGUAGE_IDENTIFIER, language.identifier());
        let _ = file.write(language_property.as_bytes());
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Language {
    German,
    English,
    Norwegian
}

impl Language {
    pub fn from_identifier(i: &str) -> Language {
        match i {
            GERMAN => Language::German,
            ENGLISH => Language::English,
            NORWEGIAN => Language::Norwegian,
            _ => Language::English
        }   
    }

    pub fn identifier(&self) -> &str {
        match self {
            Language::German => GERMAN,
            Language::English => ENGLISH,
            Language::Norwegian => NORWEGIAN,
        }
    }
}