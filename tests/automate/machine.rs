use home_automation::automate::Automate;
use home_automation::logger::AppMonitorLogger;
use home_automation::queue::Value;
use log::LevelFilter;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::Duration;

#[test]
fn test_automate_integration() {
    // GIVEN
    AppMonitorLogger::new()
        .init(LevelFilter::Debug)
        .expect("Logger initialization failed");

    // Create an mpsc channel
    let (sender, _receiver) = mpsc::channel::<Value>();
    // Flag to signal threads to stop
    let running = Arc::new(AtomicBool::new(true));

    // Clone the running flag for each thread
    let running_automate = Arc::clone(&running);

    // WHEN
    let mut automate = Automate::new(sender);
    let automate_handle = thread::spawn(move || {
        automate.run(running_automate);
    });
    thread::sleep(Duration::from_secs(6));
    running.store(false, Ordering::Relaxed);
    automate_handle.join().unwrap();

    // THEN
    assert!(true);
}
