use fern::colors::{Color, ColoredLevelConfig};
use log::LevelFilter;

pub struct AppMonitorLogger {}

impl AppMonitorLogger {
    pub fn new() -> Self {
        AppMonitorLogger {}
    }

    pub fn print(&self) -> String {
        " ".to_string()
    }

    pub fn get_colors(&self) -> ColoredLevelConfig {
        ColoredLevelConfig::new()
            .error(Color::Red)
            .warn(Color::Yellow)
            .info(Color::Green)
            .debug(Color::Blue)
            .trace(Color::BrightBlack)
    }

    pub fn init(&self, level_filter: LevelFilter) -> Result<(), fern::InitError> {
        let colors = self.get_colors();

        // Configure logger with `fern`
        fern::Dispatch::new()
            .format(move |out, message, record| {
                out.finish(format_args!(
                    "[{} {} {}] {}",
                    chrono::Local::now().format("%H:%M:%S"), // Timestamp
                    colors.color(record.level()),            // colored log level
                    record.target(),                         // Module source
                    message                                  // Message
                ))
            })
            .level(level_filter) // Minimum log level
            .chain(std::io::stdout()) // Destination: stdout
            .apply()?; // Apply configuration

        Ok(())
    }

    pub fn demo(&self) {
        log::info!("Logger initialized with Fern and Chrono");
        log::debug!("This is a debug message");
        log::error!("This is an error");
    }
}
