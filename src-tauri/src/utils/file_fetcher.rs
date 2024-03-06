use std::{error::Error, fmt::{write, Display}, fs::{File, OpenOptions}, io::Read, path::Path};

pub struct FileService;

impl FileService {
    pub fn get_file(path: &str) -> Result<File, FileServiceError> {
        if Path::new(path).exists() {
            let file = OpenOptions::new().write(true).read(true).open(path)
                .map_err(|e| FileServiceError::UnableToOpen(e.to_string()))?;
            Ok(file)
        } else {
            Err(FileServiceError::NotFound)
        }
    }

    pub fn get_file_read(path: &str) -> Result<File, FileServiceError> {
        if Path::new(path).exists() {
            let file = OpenOptions::new().read(true).open(path)
                .map_err(|e| FileServiceError::UnableToOpen(e.to_string()))?;
            Ok(file)
        } else {
            Err(FileServiceError::NotFound)
        }
    }

    pub fn get_string_content(path: &str) -> Result<String, FileServiceError> {
        let mut file =  FileService::get_file_read(path)?;
        let mut content_string = String::new();
        file.read_to_string(&mut content_string)
            .map_err(|e| FileServiceError::UnableToRead(e.to_string()))?;
        Ok(content_string)
    }
}

#[derive(Debug)]
pub enum FileServiceError {
    NotFound,
    UnableToRead(String),
    UnableToWrite(String),
    UnableToOpen(String)
}

impl Error for FileServiceError {}
impl Display for FileServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            FileServiceError::NotFound => "Cannot find input path.".to_string(),
            FileServiceError::UnableToRead(r) => format!("Cannot read file contents: {}", r),
            FileServiceError::UnableToWrite(r) => format!("Cannot write into file.{}", r),
            FileServiceError::UnableToOpen(r) => format!("Cannot open file. {}", r),
        };
        write!(f, "{}", value)
    }
}

