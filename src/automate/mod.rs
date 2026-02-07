//! Module de l'automate de collecte.

pub mod state;
pub mod machine;
pub mod actions;
pub mod counters;

pub use state::{State, Event};
pub use machine::Automate;
pub use counters::Counters;