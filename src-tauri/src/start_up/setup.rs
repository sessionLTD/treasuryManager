use std::{error::Error, fmt::Display, fs::{self, File}, io::Write, path::Path};


use crate::constants::{DATA_DIRECTORY, GROUP_DATA_DIRECTORY, PROGRAM_PROPERTIES
};

pub struct SetupManager;

impl SetupManager {
    /// Checks if all necessesary directories and files are present to assure the correct functionality of the programm.
    pub fn requires_setup() -> bool {
       !(Path::new(DATA_DIRECTORY).exists() && Path::new(PROGRAM_PROPERTIES).exists() && Path::new(GROUP_DATA_DIRECTORY).exists())
    }

    /// Creates the necessesary directories to store the data of the programm. Returns an error if something hinders the process.
    pub fn setup() -> Result<(), SetupError> {
        if !Path::new(DATA_DIRECTORY).exists() {
            fs::create_dir(DATA_DIRECTORY)
                        .map_err(|error| SetupError::UnableToCreateDirectory(error.to_string())).as_ref()?;
        }
        if !Path::new(GROUP_DATA_DIRECTORY).exists() {
            fs::create_dir(GROUP_DATA_DIRECTORY)
                        .map_err(|error| SetupError::UnableToCreateDirectory(error.to_string())).as_ref()?;
        }
        if !Path::new(PROGRAM_PROPERTIES).exists() {
            let mut properties_file = File::create(PROGRAM_PROPERTIES)
                .map_err(|error| SetupError::UnableToCreateFile(error.to_string()))?;
            properties_file.write(b"language:en")
                .map_err(|error| SetupError::CannotWriteToFile(error.to_string()))?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum SetupError {
    UnableToCreateDirectory(String),
    UnableToCreateFile(String),
    CannotWriteToFile(String)
}

impl Error for SetupError {}
impl Display for SetupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            SetupError::UnableToCreateDirectory(v) => format!("Unable to create the necessesary directories: {}", v),
            SetupError::UnableToCreateFile(v) => format!("Unable to create the necessary file: {}", v),
            SetupError::CannotWriteToFile(v) => format!("Cannot write to file: {}", v),
        };
        write!(f, "{}", value)
    }
}
impl From<&SetupError> for SetupError {
    fn from(value: &SetupError) -> Self {
        match value {
            SetupError::UnableToCreateDirectory(v) => SetupError::UnableToCreateDirectory(v.to_string()),
            SetupError::UnableToCreateFile(v) => SetupError::UnableToCreateFile(v.to_string()),
            SetupError::CannotWriteToFile(b) => SetupError::CannotWriteToFile(b.to_string()),
        }
    }
}