#[cfg(test)]
mod tests {
    use crate::logger::AppMonitorLogger;
    use log::LevelFilter;

    #[test]
    fn test_new() {
        // WHEN
        let app_logger = AppMonitorLogger::new();

        app_logger
            .init(LevelFilter::Debug)
            .expect("Logger initialization failed");

        app_logger.demo();

        // THEN
        assert!(true);
    }
}
