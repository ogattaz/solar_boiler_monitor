//! Time series data processing.

use crate::queue::Value;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, mpsc};
use std::thread;
use std::time::{Duration, Instant};

pub struct Processor {
    pub start_time: Instant,
    rx: mpsc::Receiver<Value>, // Channel receiver for values
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

        let mut last_message_time = Instant::now();
        let message_interval = Duration::from_secs(5);

        loop {
            // Check if we should stop
            if !running.load(Ordering::Relaxed) {
                log::info!("Processor shutting down...");
                break;
            }
            // Check if enough time has elapsed since new message read
            if last_message_time.elapsed() >= message_interval {
                log::debug!("Processor waiting for messages...");
                last_message_time = Instant::now();
            }

            // Wait with 1 second timeout to regularly check the running flag
            match self.rx.recv_timeout(Duration::from_secs(1)) {
                Ok(value) => {
                    log::debug!(
                        "Received value: id={}, timestamp={}, value={}",
                        value.id,
                        value.timestamp,
                        value.value
                    );
                    let raw_data = RawData::from(value);
                    process_data(raw_data);
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    // Timeout reached, continue loop to check the running flag
                    continue;
                }
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    // Channel closed (all senders have been dropped)
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

// Implementation of the `From` trait to convert `Value` into `RawData`
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
