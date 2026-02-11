use crate::queue::Value;
use log::info;
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, watch};
use tokio::time::{Duration, Instant};

pub struct Processor {
    pub start_time: Instant,
    receiver: mpsc::Receiver<Value>,
}

impl Processor {
    pub fn new(receiver: mpsc::Receiver<Value>) -> Processor {
        Processor {
            start_time: Instant::now(),
            receiver,
        }
    }

    pub async fn run(&mut self, mut shutdown_receiver: watch::Receiver<bool>) {
        info!("Processor started.");

        let mut last_message_time = Instant::now();
        let message_interval = Duration::from_secs(5);

        loop {
            tokio::select! {
                // Check for shutdown signal
                _ = shutdown_receiver.changed() => {
                    if *shutdown_receiver.borrow() {
                        info!("Processor shutting down...");
                        break;
                    }
                },
                // Check for new messages
                result = self.receiver.recv() => {
                    match result {
                        Some(value) => {
                            info!(
                                "Received value: id={}, timestamp={}, value={}",
                                value.id,
                                value.timestamp,
                                value.value
                            );
                            let raw_data = RawData::from(value);
                            process_data(raw_data).await;
                        }
                        None => {
                            info!("Channel closed, stopping processor.");
                            break;
                        }
                    }
                }
            }

            // Check if enough time has elapsed since last message read
            if last_message_time.elapsed() >= message_interval {
                info!("Processor waiting for messages...");
                last_message_time = Instant::now();
            }
        }

        info!("Processor End.");
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RawData {
    pub id: u16,
    pub timestamp: u64,
    pub value: String,
}

impl From<Value> for RawData {
    fn from(value: Value) -> Self {
        RawData {
            id: value.id,
            timestamp: value.timestamp,
            value: value.value,
        }
    }
}

async fn process_data(raw_data: RawData) {
    info!("Processing data: {:?}", raw_data);
    // Ajoutez ici la logique de traitement des données
}
