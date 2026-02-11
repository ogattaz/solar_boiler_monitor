use home_automation::automate::Automate;
use home_automation::logger::AppMonitorLogger;
use home_automation::queue::Value;
use log::LevelFilter;
use tokio::sync::{mpsc, watch};
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_automate_integration() {
    // GIVEN
    AppMonitorLogger::new()
        .init(LevelFilter::Debug)
        .expect("Logger initialization failed");

    // Create an mpsc channel
    let (sender, _receiver) = mpsc::channel::<Value>(100);

    // Flag to signal tasks to stop
    let (shutdown_sender, shutdown_receiver) = watch::channel(false);

    // WHEN
    let mut automate = Automate::new(sender);
    let automate_handle = tokio::spawn(async move {
        automate.run(shutdown_receiver).await;
    });

    // Attendre un peu pour simuler le travail de l'automate
    sleep(Duration::from_secs(6)).await;

    // Envoyer le signal d'arrêt
    shutdown_sender.send(true).expect("Failed to send shutdown signal");

    // Attendre la fin de la tâche automate
    automate_handle.await.expect("Automate task panicked");

    log::info!("Test end");

    // THEN
    assert!(true);
}