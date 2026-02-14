use reqwest::{Client, Response, Error};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;
use log::info;
use crate::automaton::xml_utils::{VarDescriptions, ValueSlots, decode_value, encode_value};
use thiserror::Error;


#[derive(Error, Debug)]
pub enum HttpClientError {
    #[error("HTTP request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("XML parsing error: {0}")]
    XmlParsingError(#[from] quick_xml::DeError),
}

#[derive(Clone)]
pub struct HttpClient {
    client: Client,
    boiler_base_url: String,
    boiler_id_b64:String,
    timeout: Duration,
}

impl HttpClient {
    pub fn new(boiler_base_url: String, boiler_id: String, timeout_secs: u64) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .build()
            .expect("Failed to create HTTP client");

        HttpClient {
            client,
            boiler_base_url,
            boiler_id_b64: boiler_id,
            timeout: Duration::from_secs(timeout_secs),
        }
    }

    pub async fn post(&self, endpoint: &str, headers: HeaderMap, body: &str) -> Result<Response, Error> {
        let url = format!("{}{}", self.boiler_base_url, endpoint);
        info!("Sending POST request to: {}", url);

        let response = self.client
            .post(&url)
            .headers(headers)
            .body(body.to_string())
            .send()
            .await?;

        Ok(response)
    }

    pub async fn get(&self, endpoint: &str, headers: HeaderMap) -> Result<Response, Error> {
        let url = format!("{}{}", self.boiler_base_url, endpoint);
        info!("Sending GET request to: {}", url);

        let response = self.client
            .get(&url)
            .headers(headers)
            .send()
            .await?;

        Ok(response)
    }

    pub fn create_form_data(params: HashMap<&str, &str>) -> String {
        params.into_iter()
            .map(|(key, value)| format!("{}={}", key, value))
            .collect::<Vec<String>>()
            .join("&")
    }

    pub fn create_headers(content_type: &str, cookie: Option<&str>) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_str(content_type).unwrap());

        if let Some(cookie_value) = cookie {
            headers.insert("Cookie", HeaderValue::from_str(cookie_value).unwrap());
        }

        headers
    }

    pub async fn get_cookie(&self, endpoint: &str) -> Result<Option<String>, Error> {
        let headers = Self::create_headers("application/x-www-form-urlencoded", None);
        let response = self.get(endpoint, headers).await?;

        if let Some(cookie_header) = response.headers().get("set-cookie") {
            let cookie_value = cookie_header.to_str().unwrap();
            let cookie = cookie_value.split(';').next().unwrap().to_string();
            Ok(Some(cookie))
        } else {
            Ok(None)
        }
    }

    pub async fn get_var_descriptions(&self, endpoint: &str, cookie: Option<&str>) -> Result<VarDescriptions, HttpClientError> {
        let mut params = HashMap::new();
        params.insert("id", self.boiler_id_b64.as_str());

        let body = Self::create_form_data(params);
        let headers = Self::create_headers("application/x-www-form-urlencoded", cookie);

        let response = self.post(endpoint, headers, &body).await?;
        let response_body = response.text().await?;
        let var_descriptions = VarDescriptions::from_xml(&response_body)?;

        Ok(var_descriptions)
    }

    pub async fn get_value_slots(&self, endpoint: &str, cookie: Option<&str>, last_read_value_ticks: u64) -> Result<ValueSlots, HttpClientError> {
        let ticks = &last_read_value_ticks.to_string();
        let mut params = HashMap::new();
        params.insert("id", self.boiler_id_b64.as_str() );
        params.insert("heure", &ticks );
        params.insert("periode", "1");

        let body = Self::create_form_data(params);
        let headers = Self::create_headers("application/x-www-form-urlencoded", cookie);

        let response = self.post(endpoint, headers, &body).await?;
        let response_body = response.text().await?;
        let value_slots = ValueSlots::from_xml(&response_body)?;

        Ok(value_slots)
    }
}
