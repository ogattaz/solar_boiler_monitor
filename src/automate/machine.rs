use super::state::{State, Event};
use super::counters::Counters;
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::thread;
use crate::queue::{Queue,Value};

pub struct Automate {
    pub state: State,
    pub counters: Counters,
    pub start_time: Instant,
    queue: Arc<Queue>,  // Ajoutez la Queue comme membre de la structure

}

impl Automate {
    /// Crée une nouvelle instance de l'automate avec une Queue.
    pub fn new(queue: Arc<Queue>) -> Self {
        Automate {
            state: State::Created,
            counters: Counters::new(),
            start_time: Instant::now(),
            queue,  // Initialise la Queue
        }
    }

    pub fn uptime(&mut self) -> Duration {
        Instant::now() - self.start_time
    }

    /// Méthode principale pour exécuter la boucle de l'automate.
    pub fn run(&mut self) {
        // Démarrer l'automate
        self.handle_event(Event::Start);

        loop {
            thread::sleep(Duration::from_secs(1));
            match self.state {
                State::Idle => {
                    self.diagnose_network();
                },
                State::Tested => {
                    self.initialize();
                },
                State::Initialized => {
                    self.loggin();
                },
                State::Connected => {
                    self.read_description();
                },
                State::Ready => {
                    self. read_values();
                    thread::sleep(Duration::from_secs(30)); // Attendre 30 secondes
                },
                _ => {
                    thread::sleep(Duration::from_secs(1));
                }
            }
        }
    }

    /// Gère un événement et déclenche les transitions.
    pub fn handle_event(&mut self, event: Event) {
        match (&self.state, event) {
            (State::Created, Event::Start) => {
                println!("Automate started.");
                self.state = State::Idle; // Exemple de transition
            },
            (State::Idle, Event::Stop) => {

            },

            _ => println!("Transition non gérée : {:?} -> {:?}", self.state, event),
        }
    }

    /// Exemple d'action : diagnostique réseau.
    fn diagnose_network(&mut self)  {
        println!("Diagnosing network...");
        self.counters.increment("diagnose_network");

    // let resultat =
    // match resultat {
    // Ok(valeur) => println!("Résultat : {}", valeur),
    // Err(e) => println!("Erreur : {}", e),
    // }
        // Transition to Tested
        self.state = State::Tested;
    }

    fn initialize(&mut self)  {
        println!("Initializing...");
        self.counters.increment("initialize");

        self.state = State::Initialized;
    }

    fn loggin(&mut self)  {
        println!("Logging...");
        self.counters.increment("loggin");

        self.state = State::Connected;
    }

    fn read_description(&mut self)  {
        println!("reading description...");
        self.counters.increment("loggin");


        self.state = State::Ready;
    }

    fn read_values(&mut self)  {
        println!("Read_values en cours...");
        self.counters.increment("read_values");

        // Simuler la lecture des valeurs
        let value = Value {
            id: 125,
            timestamp:0,
            value: "23.5 dC".to_string(), // Exemple de valeur
        };
        self.queue.enqueue(value);
    }

    fn logoff(&mut self)  {
        // Transition to Ready
        self.state = State::Idle;
    }
}