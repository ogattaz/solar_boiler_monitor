// tests/processor/processor_e2e

use home_automation::logger::MonitorLogger;
use log::LevelFilter;

#[test]
fn test_processor_integration() {
    // GIVEN
    MonitorLogger::new()
        .init(LevelFilter::Debug)
        .expect("Logger initialization failed");

    // THEN
    assert!(true);
}
