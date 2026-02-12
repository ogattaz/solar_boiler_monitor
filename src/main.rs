use axum::{routing::get, Json, Router};
use clap::Parser;
use home_automation::automate::Automate;
use home_automation::config::AppMonitorConfig;
use home_automation::logger::AppMonitorLogger;
use home_automation::queue::Value;
use home_automation::timeseries::processor::Processor;
use std::net::SocketAddr;
use tokio::sync::{mpsc, oneshot, watch};
use tokio::task::JoinHandle;

#[tokio::main]
async fn main() {
    // Parse the command line arguments into a Config object.
    let config = AppMonitorConfig::parse();

    AppMonitorLogger::new()
        .init(config.log_level)
        .expect("Logger initialization failed");

    log::info!("Monitor begin. {}", config.print());

    // Create an asynchronous channel
    let (sender, receiver) = mpsc::channel::<Value>(100);

    // Flag to signal tasks to stop
    let (shutdown_sender, shutdown_receiver) = watch::channel(false);

    // Create channels to notify when tasks are done
    let (automate_done_tx, automate_done_rx) = oneshot::channel();
    let (processor_done_tx, processor_done_rx) = oneshot::channel();
    let (http_done_tx, http_done_rx) = oneshot::channel();

    let automate_shutdown_receiver = shutdown_receiver.clone();
    let processor_shutdown_receiver = shutdown_receiver.clone();
    let http_shutdown_receiver = shutdown_receiver.clone();

    // Spawn the automate task
    let mut automate = Automate::new(sender);
    let automate_handle: JoinHandle<()> = tokio::spawn(async move {
        automate.run(automate_shutdown_receiver).await;
        let _ = automate_done_tx.send(());
    });

    // Spawn the processor task
    let mut processor = Processor::new(receiver);
    let processor_handle: JoinHandle<()> = tokio::spawn(async move {
        processor.run(processor_shutdown_receiver).await;
        let _ = processor_done_tx.send(());
    });

    // Spawn the HTTP server task
    let http_handle: JoinHandle<()> = tokio::spawn(async move {
        start_http_server(http_shutdown_receiver).await;
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

async fn start_http_server(mut shutdown_receiver: watch::Receiver<bool>) {
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/metrics", get(get_metrics));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    log::info!("HTTP server started on http://{}/health", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(async move {
            shutdown_receiver.changed().await.ok();
            log::info!("HTTP server stopping...");
        })
        .await
        .unwrap();

    log::info!("HTTP server stopped");
}

async fn health_check() -> &'static str {
    "OK"
}

async fn get_metrics() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "metrics": {
            "temperature": 23.5,
            "pressure": 1.2
        }
    }))
}
