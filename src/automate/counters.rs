use std::collections::HashMap;

/// Structure dédiée à la gestion des compteurs.
#[derive(Debug, Default)]
pub struct Counters {
    counters: HashMap<String, u32>,
}

impl Counters {
    /// Crée une nouvelle instance de `Counter`.
    pub fn new() -> Self {
        Counters {
            counters: HashMap::new(),
        }
    }

    /// Incrémente le compteur associé à une action.
    pub fn increment(&mut self, action: &str) {
        *self.counters.entry(action.to_string()).or_insert(0) += 1;
    }

    /// Récupère la valeur d'un compteur.
    pub fn get(&self, action: &str) -> u32 {
        *self.counters.get(action).unwrap_or(&0)
    }

    /// Réinitialise tous les compteurs.
    pub fn reset(&mut self) {
        self.counters.clear();
    }

    /// Récupère une référence immutable à la HashMap interne (pour lecture seule).
    pub fn as_map(&self) -> &HashMap<String, u32> {
        &self.counters
    }
}