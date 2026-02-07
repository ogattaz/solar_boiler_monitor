//! Modèles de données pour VictoriaMetrics.

use std::collections::HashMap;

/// Représente une métrique VictoriaMetrics.
#[derive(Debug, Clone)]
pub struct Metric {
    pub name: String,
    pub labels: HashMap<String, String>,
    pub value: f64,
    pub timestamp: u64,
}

impl Metric {
    /// Crée une nouvelle métrique.
    pub fn new(name: &str, value: f64, timestamp: u64) -> Self {
        Metric {
            name: name.to_string(),
            labels: HashMap::new(),
            value,
            timestamp,
        }
    }

    /// Ajoute un label à la métrique.
    pub fn add_label(&mut self, key: &str, value: &str) {
        self.labels.insert(key.to_string(), value.to_string());
    }
}
