use std::{fs::{self, File, OpenOptions}, io::Write, path::Path};
use crate::{constants::{TRANSACTION_BUNDLE_FILE_SUFFIX, TRANSACTION_BUNDLE_MAP_FILE, TRANSACTION_BUNDLE_MAP_FILE_SUFFIX, TRANSACTION_DIRECTORY}, models::{group::GroupID, requests::transaction_creation_request::TransactionCreationRequest, timestamp_to_transaction_bundle::TimestampToTransactionBundle, transaction::Transaction, transaction_bundle::{TransactionBundle, TransactionBundleID}}, utils::file_fetcher::FileService};
use super::{data_base_error::DataBaseError, group_data_manager::GroupDataManager, payer_data_manager::PayerDataManager};

pub struct TransactionDataManager {
    bundle_map: Option<TimestampToTransactionBundle>,
    current_group: Option<GroupID>
}

impl TransactionDataManager {
    pub const fn new() -> Self {
        Self {bundle_map: None, current_group: None}
    }

    pub fn new_transaction(&mut self, transaction_creation_request: &TransactionCreationRequest) -> Result<Transaction, DataBaseError> {
        if self.is_not_initialised(&transaction_creation_request.group_id) {
            self.init(&transaction_creation_request.group_id)?;
        }
        let mut payer = PayerDataManager::get_payer(&transaction_creation_request.payer_id, &transaction_creation_request.group_id)?;
        let transaction: Transaction = Transaction::new_from_request(transaction_creation_request);
        
        let bundle_map = &self.bundle_map;
        if let Some(map) = bundle_map {
            let bundle_id = {
                let map_bundle_id = map.map.get(&transaction_creation_request.timestamp);
                if let Some(m) = map_bundle_id {
                    m.to_owned()
                } else {
                    self.create_new_bundle(&transaction_creation_request.group_id, &transaction_creation_request.timestamp)?
                }
            };

            self.save_transaction_in_bundle(&transaction, &bundle_id)?;
            *payer.get_balance_mut() += *transaction.get_amount();
            Ok(transaction)
        } else {
            Err(DataBaseError::CacheProblem("There is no bundle map in the cache.".to_string()))
        }


    }  


    pub fn create_new_bundle(&mut self, group_id: &GroupID, timestamp: &u64) -> Result<TransactionBundleID, DataBaseError> {
        let bundle = TransactionBundle::new(vec![], *timestamp);
        self.save_bundle(&bundle, group_id)?;
        Ok(bundle.get_id().clone())
    }

    pub fn init(&mut self, group_id: &GroupID) -> Result<(), DataBaseError> {
        self.current_group.replace(group_id.clone());
        self.bundle_map.replace(bundle_map_for_group(group_id).map_err(|e| DataBaseError::CacheProblem(format!("Cannot create bundle map cache: {}", e)))?);
        Ok(())
    }

    fn is_not_initialised(&self, group_id: &GroupID) -> bool {
        let current_group = &self.current_group;
        let bundle_map = &self.bundle_map;
        current_group.is_none() || current_group.clone().is_some_and(|group| group != *group_id) || bundle_map.is_none()
    }

    fn save_bundle(&mut self, bundle: &TransactionBundle, group_id: &GroupID) -> Result<(), DataBaseError> {
        let path = transaction_directory(group_id)?;
        let file_path = format!("{}{}.bundle", path, bundle.get_id());
        let mut file = File::create(file_path)
            .map_err(|e| DataBaseError::FileCreation(format!("Tried to create Bundlefile: {}", e)))?;
        let bundle_string = ron::to_string(&bundle)
            .map_err(|e| DataBaseError::Writing(format!("Cannot convert bundle to string: {}", e)))?;
        file.write(bundle_string.as_bytes())
            .map_err(|e| DataBaseError::Writing(format!("Cannot write to bundle file: {}", e)))?;
        if self.bundle_map.is_some() {
            let mut map = self.bundle_map.clone().unwrap();
            map.map.insert(*bundle.get_timestamp(), bundle.get_id().clone());
            self.bundle_map.replace(map);
        }
        self.save_bundle_map()?;
        Ok(())
    }

    fn save_bundle_map(&self) -> Result<(), DataBaseError> {
        let bundle_map = &self.bundle_map;
        let current_group = &self.current_group;
        if let Some(map) = bundle_map {
            if let Some(group) = current_group {
                let path = bundle_map_file(group)?;
                let get_file: Result<File, DataBaseError> = if Path::new(&path).exists() {
                    let file = OpenOptions::new().write(true).open(path)
                        .map_err(|e| DataBaseError::Reading(format!("Cannot open File: {}", e)))?;
                    Ok(file)
                } else { 
                    let file = File::create(path)
                    .map_err(|e| DataBaseError::FileCreation(format!("Cannot create File: {}", e)))?;
                    Ok(file)
                };
                let mut file = get_file?;
                let map_string = ron::to_string(&map)
                    .map_err(|e| DataBaseError::Writing(format!("Cannot convert map to string: {}", e)))?;
                file.write(map_string.as_bytes()).map_err(|e| DataBaseError::Writing(format!("Cannot write to Map file: {}", e)))?;
                Ok(())
            } else {
                Err(DataBaseError::CacheProblem("Concurreny Error".to_string()))
            } 
        }   else {
            Err(DataBaseError::CacheProblem("Concurreny Error".to_string()))
        }
    }

    fn save_transaction_in_bundle(&self, transaction: &Transaction, bundle_id: &TransactionBundleID) -> Result<(), DataBaseError> {
        let group = &self.current_group;
        if let Some(group) = group {
            let directory = transaction_directory(group)?;
            let path = format!("{}{}{}", directory, bundle_id, TRANSACTION_BUNDLE_FILE_SUFFIX);
            if Path::new(&path).exists() {
                let mut file = OpenOptions::new().write(true).read(true).open(&path).map_err(|e| DataBaseError::Reading(format!("Cannot open bundle file: {}", e)))?;
                let content = fs::read(&path).map_err(|e| DataBaseError::Reading(format!("Cannot read existing bundle data: {}", e)))?;
                let content_string = std::str::from_utf8(&content).map_err(|e| DataBaseError::Reading(format!("Cannot convert bundle content to string: {}", e)))?;
                let mut bundle = ron::from_str::<TransactionBundle>(content_string).map_err(|e| DataBaseError::Reading(format!("Cannot convert bundle string to object: {}", e)))?;
                bundle.get_transactions_mut().push(transaction.clone());
                bundle.recalculate();
                let ron = ron::to_string(&bundle).map_err(|e| DataBaseError::Writing(format!("Cannot convert to ron: {}", e)))?;
                file.write(ron.as_bytes()).map_err(|e| DataBaseError::Writing(format!("Cannot write to bundle file: {}", e)))?;
                Ok(())
            } else {
                Err(DataBaseError::NotFound)
            }
        } else {
            Err(DataBaseError::CacheProblem("There is no group in chache.".to_string()))
        }
    }
}

fn bundle_map_for_group(group_id: &GroupID) -> Result<TimestampToTransactionBundle, DataBaseError> {
    let directory = transaction_directory(group_id)?;
    let map_file_path = format!("{}{}{}", directory, TRANSACTION_BUNDLE_MAP_FILE, TRANSACTION_BUNDLE_MAP_FILE_SUFFIX);
    if Path::new(&map_file_path).exists() {
        let content = FileService::get_string_content(&map_file_path)
            .map_err(|e| DataBaseError::Reading(format!("Cannot get map content: {}", e)))?;
        let map = ron::from_str::<TimestampToTransactionBundle>(&content)
            .map_err(|e| DataBaseError::Reading(format!("Tried to convert Timestamp to TransactionBundle map: {}", e)))?;
        Ok(map)
    } else {
        Err(DataBaseError::PathFinding("Cannot find Timestamp to TransactionBundle map file".to_string()))
    }
}

fn transaction_directory(group_id: &GroupID) -> Result<String, DataBaseError> {
    let group_directory = GroupDataManager::get_group_directory(group_id)
        .map_err(|e| DataBaseError::PathFinding(format!("Cannot find transaction folder for group {}: {}", group_id, e)))?;
    Ok(format!("{}\\{}", group_directory, TRANSACTION_DIRECTORY))
}

fn bundle_map_file(group_id: &GroupID) -> Result<String, DataBaseError> {
    let t_directory = transaction_directory(group_id)?;
    Ok(format!("{}{}{}", t_directory, TRANSACTION_BUNDLE_MAP_FILE, TRANSACTION_BUNDLE_FILE_SUFFIX))
}