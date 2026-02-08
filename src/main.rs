mod queue;

use log::LevelFilter;
use fern::colors::{ColoredLevelConfig, Color};
use std::sync::Arc;
use home_automation::automate::{Automate, Event};
use home_automation::timeseries::processor;
use home_automation::victoriametrics::{VMClient, Metric};
use home_automation::queue::{Queue, Value};
use std::thread;
use std::time::Duration;
use home_automation::timeseries::processor::RawData;

fn main() {

    init_logger().expect("TODO: panic message");

    log::info!("Begin");

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
                    // Traiter la valeur en TimeSeries
                    let processed_data = processor::process_data(raw_data);

                    // Convertir en métriques VictoriaMetrics
                    // let metrics: Vec<Metric> = processed_data
                    //     .into_iter()
                    //     .map(|(name, value, timestamp)| {
                    //         let mut metric = Metric::new(&name, value, timestamp);
                    //         metric.add_label("source", "home_automation");
                    //         metric
                    //     })
                    //     .collect();

                    // Envoyer les métriques (blocage car pas dans un contexte async)
                    // En pratique, il faudrait utiliser un runtime Tokio dans un thread dédié
                    // ou utiliser un canal pour communiquer avec un thread async.
                    // Ici, on simule simplement l'envoi.
                    //println!("Envoi des métriques: {:?}", metrics);
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


fn init_logger() -> Result<(), fern::InitError> {

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
                chrono::Local::now().format("%H:%M:%S"),  // Timestamp
                colors.color(record.level()),           // Niveau de log coloré
                record.target(),                       // Module source
                message                                // Message
            ))
        })
        .level(LevelFilter::Debug)  // Niveau minimal de log
        .chain(std::io::stdout())   // Destination : stdout
        .apply()?;                   // Applique la configuration

    log::info!("Logger initialisé avec Fern et Chrono");
    log::debug!("Ceci est un message de debug");
    log::error!("Ceci est une erreur");

    Ok(())
}
