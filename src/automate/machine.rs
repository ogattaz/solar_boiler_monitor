use super::counters::Counters;
use super::state::{Event, State};
use crate::queue::Value;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::{Duration, Instant};

pub struct Automate {
    pub state: State,
    pub counters: Counters,
    pub start_time: Instant,
    tx: mpsc::Sender<Value>, // Channel sender for values
}

impl Automate {
    /// Creates a new instance of the automaton with an mpsc Sender.
    pub fn new(tx: mpsc::Sender<Value>) -> Self {
        log::info!("New Automate");
        Automate {
            state: State::Created,
            counters: Counters::new(),
            start_time: Instant::now(),
            tx, // Initialize the Sender
        }
    }

    pub fn uptime(&mut self) -> Duration {
        Instant::now() - self.start_time
    }

    pub fn run(&mut self, running: Arc<AtomicBool>) {
        self.handle_event(Event::Start);

        let mut last_read_time = Instant::now();
        let read_interval = Duration::from_secs(20);

        loop {
            thread::sleep(Duration::from_secs(1));
            if !running.load(Ordering::Relaxed) {
                log::info!("Automate shutting down...");
                break;
            }
            match self.state {
                State::Idle => {
                    self.diagnose_network();
                }
                State::Tested => {
                    self.initialize();
                }
                State::Initialized => {
                    self.logging();
                }
                State::Connected => {
                    self.read_description();
                }
                State::Ready => {
                    // Check if enough time has elapsed since last read
                    if last_read_time.elapsed() >= read_interval {
                        self.read_values();
                        last_read_time = Instant::now();
                    }
                }
                _ => {}
            }
        }

        if self.state != State::Idle && self.state != State::Tested {
            self.logoff();
        }
        log::info!("Automate End.");
    }

    pub fn handle_event(&mut self, event: Event) {
        match (&self.state, event) {
            (State::Created, Event::Start) => {
                log::info!("Automate started.");
                self.state = State::Idle; // Example transition
            }
            (State::Idle, Event::Stop) => {}

            _ => println!("Unknown transition: {:?} -> {:?}", self.state, event),
        }
    }

    fn diagnose_network(&mut self) {
        log::info!("Diagnosing network...");
        self.counters.increment("diagnose_network");

        self.state = State::Tested;
    }

    fn initialize(&mut self) {
        log::info!("Initializing...");
        self.counters.increment("initialize");

        self.state = State::Initialized;
    }

    fn logging(&mut self) {
        log::info!("Logging...");
        self.counters.increment("logging");

        self.state = State::Connected;
    }

    fn read_description(&mut self) {
        log::info!("reading description...");
        self.counters.increment("read_description");

        self.state = State::Ready;
    }

    fn read_values(&mut self) {
        log::info!("Reading values...");
        self.counters.increment("read_values");

        // Simulate reading values
        let value = Value {
            id: 125,
            timestamp: 0,
            value: "23.5 dC".to_string(), // Example value
        };

        // Send via the channel
        match self.tx.send(value) {
            Ok(_) => log::debug!("Value sent successfully"),
            Err(e) => log::error!("Failed to send value: {:?}", e),
        }
    }

    fn logoff(&mut self) {
        log::info!("Logoff...");
        self.counters.increment("logoff");

        self.state = State::Idle;
    }
}
