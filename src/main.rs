use clap::Parser;
use ctrlc;
use fern::colors::{Color, ColoredLevelConfig};
use home_automation::automate::Automate;
use home_automation::config::AppMonitorConfig;
use home_automation::queue::Value;
use home_automation::timeseries::processor::Processor;
use home_automation::victoriametrics::VMClient;
use log::LevelFilter;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::thread;

fn main() {
    // Parse the command line arguments into a Config object.
    let config = AppMonitorConfig::parse();

    init_logger(config.log_level).expect("Failed to initialize logger");

    log::info!("{}", config.print());

    // Create an mpsc channel
    let (tx, rx) = mpsc::channel::<Value>();

    // Flag to signal threads to stop
    let running = Arc::new(AtomicBool::new(true));

    // Clone the running flag for each thread
    let running_automate = Arc::clone(&running);
    let running_processor = Arc::clone(&running);

    let mut automate = Automate::new(tx);

    let automate_handle = thread::spawn(move || {
        automate.run(running_automate);
    });

    let vm_client = VMClient::new("http://localhost:8428");

    let mut processor = Processor::new(rx);

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

fn init_logger(levelFilter: LevelFilter) -> Result<(), fern::InitError> {
    // Configure colors for log levels
    let colors = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Green)
        .debug(Color::Blue)
        .trace(Color::BrightBlack);

    // Configure logger with `fern`
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                chrono::Local::now().format("%H:%M:%S"), // Timestamp
                colors.color(record.level()),            // colored log level
                record.target(),                         // Module source
                message                                  // Message
            ))
        })
        .level(levelFilter) // Minimum log level
        .chain(std::io::stdout()) // Destination: stdout
        .apply()?; // Apply configuration

    log::info!("Logger initialized with Fern and Chrono");
    log::debug!("This is a debug message");
    log::error!("This is an error");

    Ok(())
}
