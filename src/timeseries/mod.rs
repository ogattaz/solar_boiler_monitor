//! Module de gestion des séries temporelles.

pub mod processor;
pub mod storage;

pub use processor::process_data;
pub use storage::store_locally;