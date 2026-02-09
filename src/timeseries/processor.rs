//! Traitement des données en séries temporelles.

use crate::automate::{Counters, State};
use crate::queue::{Queue, Value};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};

pub struct Processor {
    pub start_time: Instant,
    queue: Arc<Queue>, // Ajoutez la Queue comme membre de la structure
}

impl Processor {
    pub fn new(queue: Arc<Queue>) -> Processor {
        Processor {
            start_time: Instant::now(),
            queue: queue,
        }
    }

    pub fn run(&mut self, running: Arc<AtomicBool>) {
        log::info!("Processor started.");
        loop {
            thread::sleep(Duration::from_secs(5));
            if (!running.load(Ordering::Relaxed)) {
                break;
            }
            log::info!("Processor running...");

            if !self.queue.is_empty() {
                if let Some(value) = self.queue.dequeue() {
                    let raw_data = RawData::from(value);
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
