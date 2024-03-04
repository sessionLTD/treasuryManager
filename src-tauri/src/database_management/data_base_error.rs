use std::{error::Error, fmt::Display};

#[derive(Debug)]
/// Error type that is used for problems that occur within a data base.
pub enum DataBaseError {
    /// The data base is unable to read the contents of its files. The reason is inside the enum.
    Reading(String),
    /// The data base is unable to write to the files. The reason is inside the enum.
    Writing(String),
    /// The data base is unable to create new files to store its data. The reason is inside the enum.
    FileCreation(String),
    /// The data base could not get the data that was looked for.
    NotFound,
    AlreadyExists
}

impl Error for DataBaseError {}
impl Display for DataBaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            DataBaseError::Reading(reason) => format!("Unable to read files: {}", reason),
            DataBaseError::Writing(reason) => format!("Unable to write to files: {}", reason),
            DataBaseError::FileCreation(reason) => format!("Unable to create new files: {}", reason),
            DataBaseError::NotFound => String::from("Could not find the searched data."),
            DataBaseError::AlreadyExists => String::from("Inserted value already exists."),
        };
        write!(f, "{}", value)
    }
}

impl From<&DataBaseError> for DataBaseError {
    fn from(value: &DataBaseError) -> Self {
        match value {
            DataBaseError::Reading(reason) => DataBaseError::Reading(reason.to_owned()),
            DataBaseError::Writing(reason) => DataBaseError::Writing(reason.to_owned()),
            DataBaseError::FileCreation(reason) => DataBaseError::FileCreation(reason.to_owned()),
            DataBaseError::NotFound => DataBaseError::NotFound,
            DataBaseError::AlreadyExists => DataBaseError::AlreadyExists,
        }
    }
}