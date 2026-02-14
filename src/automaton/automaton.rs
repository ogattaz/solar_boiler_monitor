use std::collections::HashMap;
use super::counters::Counters;
use super::state::{Event, State};
use crate::config::MonitorConfig;
use crate::data::Value;
use log::{error, info};
use tokio::runtime::Runtime;
use tokio::sync::{mpsc, watch};
use tokio::time::{sleep, Duration, Instant};
use crate::automaton::actions::{run_initialize, run_login, run_read_desc};
use crate::automaton::client::HttpClient;
use crate::automaton::xml_utils::{encode_value, VarDescriptions};

pub struct Automaton {
    pub state: State,
    pub counters: Counters,
    pub start_time: Instant,
    sender: mpsc::Sender<Value>,
    config: MonitorConfig,
    http_client: HttpClient,
    cookie:Option<String>,
    connected:Option<bool>,
    var_descriptions: Option<VarDescriptions>,
}

impl Automaton {
    pub fn new(sender: mpsc::Sender<Value>, config: MonitorConfig) -> Self {
        info!("New Automate");

        // e.g. http://192.168.0.125
        let boiler_base_url = format!("http://{}",config.boiler_hostname);
        let boiler_id = encode_value(config.boiler_id.as_str());
        let http_client = HttpClient::new(boiler_base_url, boiler_id,10);

        Automaton {
            state: State::Created,
            counters: Counters::new(),
            start_time: Instant::now(),
            sender,
            config,
            http_client,
            cookie:None,
            connected:None,
            var_descriptions: None,
        }
    }

    pub fn uptime(&self) -> Duration {
        Instant::now() - self.start_time
    }

    pub async fn run(&mut self, shutdown_receiver: watch::Receiver<bool>) {
        self.handle_event(Event::Start);

        let read_interval = Duration::from_millis(self.config.reading_values_delay);
        // Initialize to zero instant (far in the past) to trigger immediate read
        let mut last_read_time = Instant::now() - read_interval;

        loop {
            sleep(Duration::from_secs(1)).await;

            if *shutdown_receiver.borrow() {
                info!("Automate shutting down...");
                break;
            }

            match self.state {
                State::Idle => {
                    self.diagnose_network().await;
                }
                State::Tested => {
                    self.initialize().await;
                }
                State::Initialized => {
                    self.logging().await;
                }
                State::Connected => {
                    self.read_description().await;
                }
                State::Ready => {
                    if last_read_time.elapsed() >= read_interval {
                        self.read_values().await;
                        last_read_time = Instant::now();
                    }
                }
                _ => {}
            }
        }

        if self.state != State::Idle && self.state != State::Tested {
            self.logoff().await;
        }
        info!("Automate End.");
    }

    pub fn handle_event(&mut self, event: Event) {
        match (&self.state, event) {
            (State::Created, Event::Start) => {
                info!("Automate started.");
                self.state = State::Idle;
            }
            (State::Idle, Event::Stop) => {}
            _ => println!("Unknown transition: {:?} -> {:?}", self.state, event),
        }
    }

    async fn diagnose_network(&mut self) {
        info!("Diagnosing network...");
        self.counters.increment("diagnose_network");
        self.state = State::Tested;
    }

    // get the cookie
    async fn initialize(&mut self) {
        info!("Initializing...");
        self.counters.increment("initialize");

        match run_initialize(self.http_client.clone()).await {
            Ok(ookie_value)=>{
                self.cookie = Option::from(ookie_value);
                self.state = State::Initialized
            }
            Err(e)=>{
                error!("Error during initialization: {}", e);
                self.state = State::Idle
            }
        }
    }

    async fn logging(&mut self) {
        info!("Logging...");
        self.counters.increment("logging");

        let cookie_value = self.cookie.clone().unwrap();

        match run_login(self.http_client.clone(),self.config.user_id.clone(),self.config.read_password(),cookie_value).await {
            Ok(connected)=>{
                self.connected = Option::from(connected);
                self.state = State::Connected
            }
            Err(e)=>{
                error!("Error during initialization: {}", e);
                self.logoff().await;
                self.state = State::Idle
            }
        }
    }

    async fn read_description(&mut self) {
        info!("Reading description...");
        self.counters.increment("read_description");

        let cookie_value = self.cookie.clone().unwrap();

        match run_read_desc(self.http_client.clone(),cookie_value).await {
            Ok(connected)=>{
                self.connected = Option::from(connected);
                self.state = State::Connected
            }
            Err(e)=>{
                error!("Error during initialization: {}", e);
                self.logoff().await;
                self.state = State::Idle
            }
        }

        self.state = State::Ready;
    }

    async fn read_values(&mut self) {
        info!("Reading values...");
        self.counters.increment("read_values");

        let cookie_value = self.cookie.clone().unwrap();

        let value = Value {
            id: 125,
            timestamp: 0,
            value: "23.5 dC".to_string(),
        };

        if let Err(e) = self.sender.send(value).await {
            log::error!("Failed to send value: {:?}", e);
        }
    }

    async fn logoff(&mut self) {
        info!("Logoff...");
        self.counters.increment("logoff");
        self.state = State::Idle;
    }

    pub fn ping (&self){
        info!("Ping...");

        let rt = Runtime::new().unwrap(); // Crée un runtime Tokio
        rt.block_on(async {

            let headers = HttpClient::create_headers("text/plain", None);

            match self.http_client.get("/",headers).await{
                Ok(response)=>{
                    info!("Ping OK. status=[{:?}]", response.status().to_string())
                }
                Err(e)=>{
                    error!("Failed to ping: {:?}", e);
                }
            }
        });
    }
}
