// tests/timeseries/processor.rs

use home_automation::logger::AppMonitorLogger;
use log::LevelFilter;

#[test]
fn test_processor() {
    // GIVEN
    AppMonitorLogger::new()
        .init(LevelFilter::Debug)
        .expect("Logger initialization failed");

    // THEN
    assert!(true);
}
