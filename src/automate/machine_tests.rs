#[cfg(test)]
mod tests {
    use crate::automate::{Automate, State};
    use crate::config::AppMonitorConfig;
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

        let config = AppMonitorConfig::default();

        // WHEN
        let automate = Automate::new(sender, config);

        let current_state = automate.state;
        log::info!("Current State: {:?}", current_state);

        // THEN
        assert_eq!(current_state, State::Created);
    }
}
