use crate::config::MonitorConfig;
use axum::extract::State;
use axum::response::Html;
use axum::routing::get;
use axum::{Json, Router};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::watch;
use tokio::time::Instant;

#[derive(Clone)]
pub struct UIServer {
    pub start_time: Instant,
    config: MonitorConfig,
}

impl UIServer {
    pub fn new(config: MonitorConfig) -> Self {
        log::info!("New HttpServer");
        UIServer {
            start_time: Instant::now(),
            config,
        }
    }

    pub fn uptime(&self) -> Duration {
        Instant::now() - self.start_time
    }

    pub async fn run(&self, mut shutdown_receiver: watch::Receiver<bool>) {
        let server = Arc::new(self.clone());

        let app = Router::new()
            .route("/", get(health_home_handler))
            .route("/health", get(health_check_handler))
            .route("/counters", get(get_counters_handler))
            .route("/config", get(get_config_handler))
            .with_state(server);

        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        log::info!("HTTP server started on http://{}/", addr);

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
}

async fn health_home_handler(State(server): State<Arc<UIServer>>) -> Html<String> {
    let up_time = server.uptime().as_secs().to_string();
    let html_content = format!(
        "<!doctype html><html><head><title>Solar boiler home</title></head><body>
    <h1>Solar Boiler Monitor</h1>
    <h2>Services:</h2>
    <ul>
    <li><a href=\"./config\">config</a></li>
    <li><a href=\"./counters\">counters</a></li>
    <li><a href=\"./health\">health</a></li>
    </ul>
    <p></p>
    <p>Current time is: {}, upTime: {} secondes</p>
    </body></html>",
        chrono::Local::now().format("%H:%M:%S"),
        up_time
    );

    Html(html_content)
}

async fn health_check_handler(State(server): State<Arc<UIServer>>) -> String {
    let up_time = server.uptime().as_secs().to_string();
    format!("health: OK, upTime: {} secondes", up_time)
}

async fn get_counters_handler(State(server): State<Arc<UIServer>>) -> Json<serde_json::Value> {
    let up_time = server.uptime().as_secs().to_string();
    Json(serde_json::json!({
        "status": "ok",
        "upTime": up_time.to_string(),
    }))
}

async fn get_config_handler(State(server): State<Arc<UIServer>>) -> Json<serde_json::Value> {
    server.config.get_json()
}
