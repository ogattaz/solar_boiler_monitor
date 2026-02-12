#[cfg(test)]
mod tests {
    use crate::logger::MonitorLogger;
    use log::LevelFilter;

    #[test]
    fn test_new() {
        // WHEN
        let app_logger = MonitorLogger::new();

        app_logger
            .init(LevelFilter::Debug)
            .expect("Logger initialization failed");

        app_logger.demo();

        // THEN
        assert!(true);
    }
}
