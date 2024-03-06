use std::collections::HashMap;
use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize};

use super::transaction_bundle::TransactionBundleID;

#[derive(Deserialize, Clone)]
pub struct TimestampToTransactionBundle{
    pub map: HashMap<u64, TransactionBundleID>
}

impl Serialize for TimestampToTransactionBundle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            let mut map = serializer.serialize_map(Some(self.map.len()))?;
            for (k, v) in &self.map {
                map.serialize_entry(&k.to_string(), &v)?;
            }
            map.end()
    }
}