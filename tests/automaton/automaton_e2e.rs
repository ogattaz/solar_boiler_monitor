use home_automation::automaton::Automaton;
use home_automation::config::MonitorConfig;
use home_automation::logger::MonitorLogger;
use home_automation::data::Value;
use log::LevelFilter;
use tokio::sync::{mpsc, watch};
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_automaton_integration() {
    // GIVEN
    MonitorLogger::new()
        .init(LevelFilter::Debug)
        .expect("Logger initialization failed");

    // Create an mpsc channel
    let (sender, _receiver) = mpsc::channel::<Value>(100);

    // Flag to signal tasks to stop
    let (shutdown_sender, shutdown_receiver) = watch::channel(false);

    // Create test config
    let config = MonitorConfig::default();

    // WHEN
    let mut automate = Automaton::new(sender, config);
    let automate_handle = tokio::spawn(async move {
        automate.run(shutdown_receiver).await;
    });

    // Attendre un peu pour simuler le travail de l'automaton
    sleep(Duration::from_secs(6)).await;

    // Envoyer le signal d'arrêt
    shutdown_sender.send(true).expect("Failed to send shutdown signal");

    // Attendre la fin de la tâche automaton
    automate_handle.await.expect("Automate task panicked");

    log::info!("Test end");

    // THEN
    assert!(true);
}