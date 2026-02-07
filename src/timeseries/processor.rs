//! Traitement des données en séries temporelles.

use serde::{Serialize, Deserialize};
use crate::queue::Value;

/// Structure pour une donnée brute.
#[derive(Debug, Serialize, Deserialize)]
pub struct RawData {
    pub id: u16,
    pub timestamp: u64,
    pub value: String,
}

// Implémentation du trait `From` pour convertir `Value` en `RawData`
impl From<Value> for RawData {
    fn from(value: Value) -> Self {
        RawData {
            id: value.id,
            timestamp: value.timestamp,
            value: value.value,
        }
    }
}

pub fn process_data(raw_data: RawData)  {

}


