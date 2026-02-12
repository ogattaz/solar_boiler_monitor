#[cfg(test)]
mod tests {
    use crate::automaton::{Automaton, State};
    use crate::config::MonitorConfig;
    use crate::data::Value;
    use crate::logger::MonitorLogger;
    use log::LevelFilter;

    #[test]
    fn test_new() {
        // GIVEN
        MonitorLogger::new()
            .init(LevelFilter::Debug)
            .expect("Logger initialization failed");

        // Create an mpsc channel
        let (sender, _receiver) = tokio::sync::mpsc::channel::<Value>(100);

        let config = MonitorConfig::default();

        // WHEN
        let automate = Automaton::new(sender, config);

        let current_state = automate.state;
        log::info!("Current State: {:?}", current_state);

        // THEN
        assert_eq!(current_state, State::Created);
    }
}
