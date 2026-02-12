use super::counters::Counters;
use super::state::{Event, State};
use crate::config::AppMonitorConfig;
use crate::queue::Value;
use log::info;
use tokio::sync::{mpsc, watch};
use tokio::time::{sleep, Duration, Instant};

pub struct Automate {
    pub state: State,
    pub counters: Counters,
    pub start_time: Instant,
    sender: mpsc::Sender<Value>,
    config: AppMonitorConfig,
}

impl Automate {
    pub fn new(sender: mpsc::Sender<Value>, config: AppMonitorConfig) -> Self {
        info!("New Automate");
        Automate {
            state: State::Created,
            counters: Counters::new(),
            start_time: Instant::now(),
            sender,
            config,
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

    async fn initialize(&mut self) {
        info!("Initializing...");
        self.counters.increment("initialize");
        self.state = State::Initialized;
    }

    async fn logging(&mut self) {
        info!("Logging...");
        self.counters.increment("logging");
        self.state = State::Connected;
    }

    async fn read_description(&mut self) {
        info!("Reading description...");
        self.counters.increment("read_description");
        self.state = State::Ready;
    }

    async fn read_values(&mut self) {
        info!("Reading values...");
        self.counters.increment("read_values");

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
}
