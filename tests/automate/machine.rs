use std::sync::{mpsc, Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use home_automation::automate::Automate;
use home_automation::queue::Value;

#[test]
fn test_automate_integration() {

    // GIVEN
    init_logger();

    // Create an mpsc channel
    let (tx, rx) = mpsc::channel::<Value>();
    // Flag to signal threads to stop
    let running = Arc::new(AtomicBool::new(true));

    // Clone the running flag for each thread
    let running_automate = Arc::clone(&running);


    // WHEN
    let mut automate = Automate::new(tx);
    let automate_handle = thread::spawn(move || {
        automate.run(running_automate);
    });
    thread::sleep(Duration::from_secs(6));
    running.store(false, Ordering::Relaxed);
    automate_handle.join().unwrap();

    // THEN
    assert!(true);
}


fn init_logger()  {
    fern::Dispatch::new()
        .level(log::LevelFilter::Debug) // maximum log level
        .chain(std::io::stdout()) // Destination: stdout
        .apply().expect("Unable to init logger"); // Apply configuration
}