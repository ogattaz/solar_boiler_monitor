use clap::Parser;
use home_automation::automaton::Automaton;
use home_automation::config::MonitorConfig;
use home_automation::data::Value;
use home_automation::logger::MonitorLogger;
use home_automation::processor::processor::Processor;
use home_automation::web::UIServer;
use tokio::sync::{mpsc, oneshot, watch};
use tokio::task::JoinHandle;

#[tokio::main]
async fn main() {
    // Parse the command line arguments into a Config object.
    let config = MonitorConfig::parse();

    MonitorLogger::new()
        .init(config.log_level)
        .expect("Logger initialization failed");

    log::info!("Monitor begin. {}", config.print());

    // Create an asynchronous channel
    let (value_sender, value_receiver) = mpsc::channel::<Value>(100);

    // Flag to signal tasks to stop
    let (shutdown_sender, shutdown_receiver) = watch::channel(false);

    // Create channels to notify when tasks are done
    let (automate_done_tx, automate_done_rx) = oneshot::channel();
    let (processor_done_tx, processor_done_rx) = oneshot::channel();
    let (http_done_tx, http_done_rx) = oneshot::channel();

    let automate_shutdown_receiver = shutdown_receiver.clone();
    let processor_shutdown_receiver = shutdown_receiver.clone();
    let http_shutdown_receiver = shutdown_receiver.clone();

    // Spawn the automaton task
    let mut automate = Automaton::new(value_sender, config.clone());
    let automate_handle: JoinHandle<()> = tokio::spawn(async move {
        automate.run(automate_shutdown_receiver).await;
        let _ = automate_done_tx.send(());
    });

    // Spawn the processor task
    let mut processor = Processor::new(value_receiver);
    let processor_handle: JoinHandle<()> = tokio::spawn(async move {
        processor.run(processor_shutdown_receiver).await;
        let _ = processor_done_tx.send(());
    });

    // Spawn the HTTP server task
    let ui_server = UIServer::new(config.clone());
    let http_handle: JoinHandle<()> = tokio::spawn(async move {
        ui_server.run(http_shutdown_receiver).await;
        let _ = http_done_tx.send(());
    });

    // Configure Ctrl+C handler
    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
        log::info!("Received SIGINT, shutting down gracefully...");
        shutdown_sender
            .send(true)
            .expect("Failed to send shutdown signal");
    });

    log::info!("Waiting for tasks to finish...");

    // Wait for all tasks to notify completion
    let (automate_result, processor_result, http_result) =
        tokio::join!(automate_done_rx, processor_done_rx, http_done_rx);

    if automate_result.is_ok() {
        log::info!("Automate task completed");
    }
    if processor_result.is_ok() {
        log::info!("Processor task completed");
    }
    if http_result.is_ok() {
        log::info!("HTTP server task completed");
    }

    // Wait for all tasks to finish
    let _ = tokio::join!(automate_handle, processor_handle, http_handle);

    log::info!("Monitor end. All tasks are stopped.");
}
