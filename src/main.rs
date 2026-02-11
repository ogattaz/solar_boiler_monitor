use clap::Parser;
use ctrlc;
use home_automation::automate::Automate;
use home_automation::config::AppMonitorConfig;
use home_automation::logger::AppMonitorLogger;
use home_automation::queue::Value;
use home_automation::timeseries::processor::Processor;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::thread;

fn main() {
    // Parse the command line arguments into a Config object.
    let config = AppMonitorConfig::parse();

    AppMonitorLogger::new()
        .init(config.log_level)
        .expect("Logger initialization failed");

    log::info!("{}", config.print());

    // Create an mpsc channel
    let (sender, receiver) = mpsc::channel::<Value>();

    // Flag to signal threads to stop
    let running = Arc::new(AtomicBool::new(true));

    // Clone the running flag for each thread
    let running_automate = Arc::clone(&running);
    let running_processor = Arc::clone(&running);

    let mut automate = Automate::new(sender);

    let automate_handle = thread::spawn(move || {
        automate.run(running_automate);
    });

    let mut processor = Processor::new(receiver);

    let processor_handle = thread::spawn(move || {
        processor.run(running_processor);
    });

    // Configure Ctrl+C handler
    ctrlc::set_handler({
        let running = Arc::clone(&running);
        move || {
            log::info!("Received SIGINT, shutting down gracefully...");
            running.store(false, Ordering::Relaxed);
        }
    })
    .expect("Error setting Ctrl-C handler");

    log::info!("Waiting for threads to finish...");

    automate_handle.join().unwrap();
    processor_handle.join().unwrap();

    log::info!("End");
}
