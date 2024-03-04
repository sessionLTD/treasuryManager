use std::{fs::{self, File}, io::Write, path::Path, str};

use crate::{constants::{GROUP_DATA_DIRECTORY, PAYER_DATA_DIRECTORY}, models::{group::GroupID, id_trait::ID, payer::{Payer, PayerID}}};

use super::data_base_error::DataBaseError;

pub struct PayerDataManager; 

impl PayerDataManager {
    pub fn save_new_payer(payer: &Payer, group_id: &GroupID) -> Result<(), DataBaseError> {
        let path = format!("{}\\{}{}", group_folder(group_id), PAYER_DATA_DIRECTORY, payer.get_id());
        let mut file = File::create(&path).map_err(|error| DataBaseError::FileCreation(format!("File: {} -> {}", path, error)))?;
        let payer_stringified = ron::to_string(payer).unwrap();
        file.write(payer_stringified.as_bytes()).map_err(|e| DataBaseError::Writing(e.to_string()))?;
        Ok(())
    }

    pub fn get_payer(id: &PayerID, group_id: &GroupID) -> Result<Payer, DataBaseError> {
        let path = format!("{}\\{}{}", group_folder(group_id), PAYER_DATA_DIRECTORY, id);
        if Path::new(&path).exists()  {
            match fs::read(path) {
                Ok(payer_data) => {
                    let string_data = str::from_utf8(payer_data.as_slice()).unwrap();
                    let payer = ron::from_str(string_data);
                    if let Ok(p) = payer {
                        Ok(p)
                    } else {
                        Err(DataBaseError::Reading("Cannot decode payer data".to_string()))
                    }
                },
                Err(e) => Err(DataBaseError::Reading(e.to_string())),
            }
        } else {
            Err(DataBaseError::NotFound)
        }
    }

    pub fn get_all_payers_of_group(group_id: &GroupID) -> Vec<Payer> {
        let mut payers: Vec<Payer> = vec![];
        if let Ok(entries) = fs::read_dir(format!("{}\\{}", group_folder(group_id), PAYER_DATA_DIRECTORY)) {
            for p_file in entries.flatten() {
                if let Ok(content) = fs::read(p_file.path()) {
                    let payer_string  = str::from_utf8(content.as_slice()).unwrap();
                    let payer = ron::from_str::<Payer>(payer_string);
                    if let Ok(p) = payer {
                        payers.push(p);
                    }
                }
            }
        }
        payers
    }
}

fn group_folder(group_id: &GroupID) -> String {
    format!("{}{}", GROUP_DATA_DIRECTORY, group_id.value())
}