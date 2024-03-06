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
    /// The data cannot be created because it already exists.
    AlreadyExists,
    /// The data base could not get the path to the directory wher the necessary data is located.
    PathFinding(String),
    /// There was an error in finding a cache because it cannot be created during another process.
    CacheProblem(String),
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
            DataBaseError::PathFinding(reason) => format!("Unable to find the path to the directory where that data should be located: {}", reason),
            DataBaseError::CacheProblem(reason) => format!("Unable to create a cache which is necessary: {}", reason),
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
            DataBaseError::PathFinding(reason) => DataBaseError::PathFinding(reason.to_owned()),
            DataBaseError::CacheProblem(reason) => DataBaseError::CacheProblem(reason.to_owned()),
        }
    }
}