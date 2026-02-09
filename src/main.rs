mod queue;

use clap::Parser;
use ctrlc;
use fern::colors::{Color, ColoredLevelConfig};
use home_automation::automate::{Automate, Event};
use home_automation::config::AppMonitorConfig;
use home_automation::queue::{Queue, Value};
use home_automation::timeseries::processor;
use home_automation::timeseries::processor::{Processor, RawData};
use home_automation::victoriametrics::{Metric, VMClient};
use log::LevelFilter;
use signal_hook::{consts::SIGINT, iterator::Signals};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

fn main() {
    // clap parses the command line arguments into a Config object.
    let config = AppMonitorConfig::parse();

    init_logger(config.log_level).expect("TODO: panic message");

    log::info!("{}", config.print());

    // Créer une file FIFO partagée
    let queue = Arc::new(Queue::new());

    // Flag pour indiquer aux threads de s'arrêter
    let running = Arc::new(AtomicBool::new(true));

    // Clone the running flag for each thread
    let running_automate = Arc::clone(&running);
    let running_processor = Arc::clone(&running);

    let mut automate = Automate::new(Arc::clone(&queue));

    let automate_handle = thread::spawn(move || {
        automate.run(running_automate);
    });

    let vm_client = VMClient::new("http://localhost:8428");

    let mut processor = Processor::new(Arc::clone(&queue));

    let processor_handle = thread::spawn(move || {
        processor.run(running_processor);
    });

    ctrlc::set_handler(move || {
        log::info!("Received SIGINT, shutting down gracefully...");
        Arc::clone(&running).store(false, Ordering::Relaxed);
    })
    .expect("Error setting Ctrl-C handler");

    log::info!("Waiting for gracefully...");

    automate_handle.join().unwrap();
    processor_handle.join().unwrap();

    log::info!("End");
}

fn init_logger(levelFilter: LevelFilter) -> Result<(), fern::InitError> {
    // Configuration des couleurs pour les niveaux de log
    let colors = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Green)
        .debug(Color::Blue)
        .trace(Color::BrightBlack);

    // Configuration du logger avec `fern`
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
        .level(levelFilter) // Niveau minimal de log
        .chain(std::io::stdout()) // Destination : stdout
        .apply()?; // Applique la configuration

    log::info!("Logger initialisé avec Fern et Chrono");
    log::debug!("Ceci est un message de debug");
    log::error!("Ceci est une erreur");

    Ok(())
}
