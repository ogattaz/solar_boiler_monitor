//! Module de l'automate de collecte.

pub mod actions;
pub mod counters;
pub mod machine;
mod machine_tests;
pub mod state;

pub use counters::Counters;
pub use machine::Automate;
pub use state::{Event, State};
