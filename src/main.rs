mod queue;

use clap::Parser;
use fern::colors::{Color, ColoredLevelConfig};
use home_automation::automate::{Automate, Event};
use home_automation::config::AppMonitorConfig;
use home_automation::queue::{Queue, Value};
use home_automation::timeseries::processor;
use home_automation::timeseries::processor::RawData;
use home_automation::victoriametrics::{Metric, VMClient};
use log::LevelFilter;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    
    // clap parses the command line arguments into a Config object.
    let config = AppMonitorConfig::parse();

    init_logger(config.log_level).expect("TODO: panic message");

    log::info!("{}", config.print());

    // Créer une file FIFO partagée
    let queue = Arc::new(Queue::new());

    // Instancier l'automate avec la Queue
    let mut automate = Automate::new(Arc::clone(&queue));

    // Lancer l'automate dans un thread
    let automate_handle = thread::spawn(move || {
        automate.run();
    });

    // Instancier le client VictoriaMetrics
    let vm_client = VMClient::new("http://localhost:8428");

    // Instancier et lancer le processeur dans un autre thread
    let processor_handle = thread::spawn(move || {
        loop {
            if !queue.is_empty() {
                if let Some(value) = queue.dequeue() {
                    let raw_data = RawData::from(value);

                }
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Attendre la fin des threads (exemple : Ctrl+C pour arrêter)
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
