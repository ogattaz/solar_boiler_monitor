#[cfg(test)]
mod tests {
    use crate::automate::{Automate, State};
    use crate::queue::Value;
    use std::sync::mpsc;

    #[test]
    fn test_new() {
        // GIVEN
        init_logger();
        // Create an mpsc channel
        let (tx, rx) = mpsc::channel::<Value>();

        // WHEN
        let automate = Automate::new(tx);

        let currentState = automate.state;
        log::info!("Current State: {:?}", currentState);

        // THEN
        assert!(currentState == State::Created);
    }

    fn init_logger() {
        fern::Dispatch::new()
            .format(move |out, message, record| {
                out.finish(format_args!(
                    "[{} {} {}] {}",
                    chrono::Local::now().format("%H:%M:%S"), // Timestamp
                    record.level(),                          // colored log level
                    record.target(),                         // Module source
                    message                                  // Message
                ))
            })
            .level(log::LevelFilter::Debug) // maximum log level
            .chain(std::io::stdout()) // Destination: stdout
            .apply()
            .expect("Unable to init logger"); // Apply configuration
    }
}
