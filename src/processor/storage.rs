//! Stockage local des séries temporelles.

use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Stocke les données localement dans un fichier.
pub fn store_locally(data: &[(String, f64, u64)], file_path: &str) -> Result<(), String> {
    let path = Path::new(file_path);
    let mut file = File::create(path).map_err(|e| e.to_string())?;

    for (metric, value, timestamp) in data {
        writeln!(file, "{},{},{}", metric, value, timestamp).map_err(|e| e.to_string())?;
    }

    Ok(())
}
