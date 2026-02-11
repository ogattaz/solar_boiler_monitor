#[cfg(test)]
mod tests {
    use crate::automate::{Automate, State};
    use crate::logger::AppMonitorLogger;
    use crate::queue::Value;
    use log::LevelFilter;

    #[test]
    fn test_new() {
        // GIVEN
        AppMonitorLogger::new()
            .init(LevelFilter::Debug)
            .expect("Logger initialization failed");

        // Create an mpsc channel
        let (sender, _receiver) = tokio::sync::mpsc::channel::<Value>(100);

        // WHEN
        let automate = Automate::new(sender);

        let current_state = automate.state;
        log::info!("Current State: {:?}", current_state);

        // THEN
        assert_eq!(current_state, State::Created);
    }
}
