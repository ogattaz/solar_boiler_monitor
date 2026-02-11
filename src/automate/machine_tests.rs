#[cfg(test)]
mod tests {
    use crate::automate::{Automate, State};
    use crate::logger::AppMonitorLogger;
    use crate::queue::Value;
    use log::LevelFilter;
    use std::sync::mpsc;

    #[test]
    fn test_new() {
        // GIVEN
        AppMonitorLogger::new()
            .init(LevelFilter::Debug)
            .expect("Logger initialization failed");

        // Create an mpsc channel
        let (tx, rx) = mpsc::channel::<Value>();

        // WHEN
        let automate = Automate::new(tx);

        let currentState = automate.state;
        log::info!("Current State: {:?}", currentState);

        // THEN
        assert!(currentState == State::Created);
    }
}
