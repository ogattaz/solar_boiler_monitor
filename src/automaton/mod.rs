//! Module de l'automaton de collecte.

pub mod actions;
pub mod automaton;
mod automaton_tests;
pub mod counters;
pub mod state;

pub use automaton::Automaton;
pub use counters::Counters;
pub use state::{Event, State};
