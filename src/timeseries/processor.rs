//! Traitement des données en séries temporelles.

use crate::queue::Value;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, mpsc};
use std::time::Instant;

pub struct Processor {
    pub start_time: Instant,
    rx: mpsc::Receiver<Value>, // Canal de réception des valeurs
}

impl Processor {
    pub fn new(rx: mpsc::Receiver<Value>) -> Processor {
        Processor {
            start_time: Instant::now(),
            rx,
        }
    }

    pub fn run(&mut self, running: Arc<AtomicBool>) {
        log::info!("Processor started.");
        loop {
            // Vérifier si on doit s'arrêter
            if !running.load(Ordering::Relaxed) {
                log::info!("Stop signal received.");
                break;
            }

            // Attente avec recv (bloquant jusqu'à réception d'un message)
            match self.rx.recv() {
                Ok(value) => {
                    log::info!(
                        "Received value: id={}, timestamp={}, value={}",
                        value.id,
                        value.timestamp,
                        value.value
                    );
                    let raw_data = RawData::from(value);
                    process_data(raw_data);
                }
                Err(_) => {
                    // Le canal a été fermé (tous les senders ont été droppés)
                    log::info!("Channel closed, stopping processor.");
                    break;
                }
            }
        }

        log::info!("Processor End.");
    }
}

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

pub fn process_data(raw_data: RawData) {}
