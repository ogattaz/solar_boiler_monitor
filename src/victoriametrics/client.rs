//! Client HTTP pour VictoriaMetrics.

use reqwest::Client;
use serde_json::Value;
use crate::victoriametrics::models::Metric;
use std::error::Error;

/// Client pour VictoriaMetrics.
pub struct VMClient {
    client: Client,
    base_url: String,
}

impl VMClient {
    /// Crée un nouveau client VictoriaMetrics.
    pub fn new(base_url: &str) -> Self {
        VMClient {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    /// Envoie des métriques à VictoriaMetrics.
    pub async fn send_metrics(
        &self,
        metrics: &[Metric],
    ) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/api/v1/import", self.base_url);
        let mut query_string = String::new();

        for metric in metrics {
            query_string.push_str(&format!(
                "{}{} {} {}\n",
                metric.name,
                metric.labels
                    .iter()
                    .map(|(k, v)| format!(",{}=\"{}\"", k, v))
                    .collect::<String>(),
                metric.value,
                metric.timestamp
            ));
        }

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "text/plain")
            .body(query_string)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!(
                "Erreur lors de l'envoi des métriques: {}",
                response.status()
            ).into());
        }

        Ok(())
    }
}
