use std::{fs::{self, File, OpenOptions}, io::{Error, Write}, path::Path, str};

use crate::{constants::{GROUP_DATA_DIRECTORY, GROUP_DATA_FILE, PAYER_DATA_DIRECTORY, TRANSACTION_DIRECTORY}, models::{group::{Group, GroupID}, id_trait::ID, payer::{Payer, PayerID}, requests::group_creation_request::GroupCreationRequest}};

use super::data_base_error::DataBaseError;

pub struct GroupDataManager;

impl GroupDataManager {
    /// Creates a new directory to save the data of a group. Creates a group data file and the necessary sub folders to store the information about Payers and Transactions.
    /// Returns an error when something went wrong.
    pub fn save_new_group(group_request: GroupCreationRequest) -> Result<Group, DataBaseError> {
        let group = Group::new(group_request.name, group_request.members);
        let group_folder = group_directory(group.get_id());
        if !Path::new(&group_folder).exists() {
            fs::create_dir(&group_folder)
                .map_err(|error| DataBaseError::FileCreation(error.to_string()))?;
            let group_data = ron::to_string(&group);
            if let Ok(data) = group_data {
                let mut file = File::create(group_data_file(group.get_id()))
                    .map_err(|error| DataBaseError::FileCreation(error.to_string()))?;
                file.write(data.as_bytes())
                    .map_err(|error| DataBaseError::Writing(error.to_string()))?;
                fs::create_dir(transactions_directory_of_group(group.get_id()))
                    .map_err(|error| DataBaseError::FileCreation(error.to_string()))?;
                fs::create_dir(payer_directory_of_group(group.get_id()))
                    .map_err(|error| DataBaseError::FileCreation(error.to_string()))?;
                Ok(group)
            } else {
                Err(DataBaseError::Writing(format!("Cannot create ron-string from group: {}", group.get_id().value())))
            }
        } else {
            Err(DataBaseError::AlreadyExists)
        }
    }

    /// Returns all groups that are saved on the machine. The groups must be inside the groups directory. If this function cannot find anything, it returns an empty Vec.
    pub fn get_all_groups() -> Vec<Group> {
        let mut groups: Vec<Group> = vec![];
        if let Ok(entries) = fs::read_dir(GROUP_DATA_DIRECTORY) {
            for group in entries.flatten() {
                if let Ok(content) = fs::read(format!("{}\\{}", group.path().display(), GROUP_DATA_FILE)) {
                    let payer_string  = str::from_utf8(content.as_slice()).unwrap();
                    let payer = ron::from_str::<Group>(payer_string);
                    if let Ok(p) = payer {
                        groups.push(p);
                    }
                }
            }
        }
        groups
    }

    /// Adds a new payer to a group. Returns an error when the group cannot be read or the new data cannot be saved. Returns the new group if successful.
    pub fn add_payer_to_group(payer: &Payer, group_id: &GroupID) -> Result<Group, DataBaseError> {
        let gathered_group = get_group(group_id);
        if let Ok(mut group) = gathered_group {
            group.get_members_mut().push(payer.get_id().clone());
            GroupDataManager::save_group(&group)?;
            Ok(group)
        } else {
            Err(DataBaseError::Reading(gathered_group.err().unwrap().to_string()))
        }
    }

    pub fn remove_payer_from_group(payer_id: &PayerID, group_id: &GroupID) -> Result<Group, DataBaseError> {
        let gathered_group = get_group(group_id);
        if let Ok(mut group) = gathered_group {
            let mut payer_index: Option<usize> = None;
            let mut i = 0;
            let members = group.get_members();
            while i < members.len() && payer_index.is_none() {
                let member = members.get(i);
                if let Some(member) = member {
                    if *member == *payer_id {
                        payer_index = Some(i);
                    }
                }
                i += 1;
            }
            if let Some(index) = payer_index {
                group.get_members_mut().remove(index);
                GroupDataManager::save_group(&group)?;
            }
            Ok(group)
        } else {
            Err(DataBaseError::Reading(gathered_group.err().unwrap().to_string()))
        }
    }

    /// Saves the given state of a group. The group needs to already exist in the database.
    pub fn save_group(group: &Group) -> Result<(), DataBaseError> {
        let group_directory = group_directory(group.get_id());
        if Path::new(&group_directory).exists() {
            let group_file = group_data_file(group.get_id());
            let group_string = ron::to_string(group)
                    .map_err(|e| DataBaseError::Writing(e.to_string()))?;
            if !Path::new(&group_file).exists() {
                let mut file = File::create(&group_file)
                    .map_err(|e| DataBaseError::Writing(e.to_string()))?;
                file.write(group_string.as_bytes())
                    .map_err(|e| DataBaseError::Writing(e.to_string()))?;
                Ok(())
            } else {
                let mut file = OpenOptions::new().write(true).open(&group_file)
                    .map_err(|error| DataBaseError::Writing(format!("Problem while trying to open group file to write in it: {}", error)))?;
                file.write(group_string.as_bytes())
                    .map_err(|e| DataBaseError::Writing(e.to_string()))?;
                Ok(())
            }
        } else {
            Err(DataBaseError::NotFound)
        }
    }
}

fn group_directory(group_id: &GroupID) -> String {
    format!("{}{}", GROUP_DATA_DIRECTORY, group_id.value())
}

fn group_data_file(group_id: &GroupID) -> String {
    format!("{}\\{}", group_directory(group_id), GROUP_DATA_FILE)
}

fn transactions_directory_of_group(group_id: &GroupID) -> String {
    format!("{}\\{}", group_directory(group_id), TRANSACTION_DIRECTORY)
}

fn payer_directory_of_group(group_id: &GroupID) -> String {
    format!("{}\\{}", group_directory(group_id), PAYER_DATA_DIRECTORY)
}

fn get_group(group_id: &GroupID) -> Result<Group, Error> {
    let content = fs::read(group_data_file(group_id))?;
    let content_string = std::str::from_utf8(content.as_slice());
    if let Ok(content_string) =  content_string{
        ron::from_str::<Group>(content_string).map_err(|error| Error::new(std::io::ErrorKind::InvalidInput, error))
    } else {
        let error = content_string.err().unwrap();
        Err(Error::new(std::io::ErrorKind::InvalidInput, error))
    }
}