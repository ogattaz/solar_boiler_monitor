use clap::Parser;
use log::LevelFilter;
use std::path::PathBuf;

/**
--user-id util --boiler-id SC1Z20230801 --boiler-hostname 192.168.0.125 --reading-values-delay 30000 --log-level debug --dry-running
*/
#[derive(Parser, Debug, Clone)]
#[command(
    author = "Olivier Gattaz",
    version = "1.0",
    about = "Solar boiler monitor application",
    long_about = "
    This program monitors a solar boiler and sends data to a server.
    It can run in dry-run mode to test without sending HTTP requests to the solar boiler.
    "
)]
pub struct AppMonitorConfig {
    /// The user ID used to connect to the boiler's HTTP server.
    #[arg(long)]
    pub user_id: String,

    /// Path to the file containing the encrypted password.
    #[arg(long, default_value = "./.boiler_password.txt")]
    pub password_file_path: PathBuf,

    /// The ID of your boiler installation.
    #[arg(long)]
    pub boiler_id: String,

    /// The hostname or IP address of the boiler in your home LAN.
    #[arg(long)]
    pub boiler_hostname: String,

    /// The delay between each reading, in milliseconds.
    #[arg(long, default_value_t = 30000)]
    pub reading_values_delay: u64,

    /// The log level (error, warn, info, debug, or trace).
    #[arg(long, default_value = "info", value_parser = parse_log_level,)]
    pub log_level: LevelFilter,

    /// Run the monitor without sending HTTP requests to the boiler.
    #[arg(long, default_value_t = false)]
    pub dry_running: bool,
}

/**
Parse the log level from a string.
Called by clap
*/
fn parse_log_level(level: &str) -> Result<LevelFilter, String> {
    match level.to_lowercase().as_str() {
        "error" => Ok(LevelFilter::Error),
        "warn" => Ok(LevelFilter::Warn),
        "info" => Ok(LevelFilter::Info),
        "debug" => Ok(LevelFilter::Debug),
        "trace" => Ok(LevelFilter::Trace),
        _ => Err(format!(
            "Invalid log level: {}. Use error, warn, info, debug, or trace.",
            level
        )),
    }
}

impl Default for AppMonitorConfig {
    fn default() -> Self {
        AppMonitorConfig {
            user_id: "test_user".to_string(),
            password_file_path: PathBuf::from("./.boiler_password.txt"),
            boiler_id: "TEST_BOILER".to_string(),
            boiler_hostname: "192.168.0.1".to_string(),
            reading_values_delay: 30000,
            log_level: LevelFilter::Debug,
            dry_running: true,
        }
    }
}

impl AppMonitorConfig {
    /// Returns a formatted string with the configuration details."
    pub fn print(&self) -> String {
        format!(
            "Config:\n\
            \t    user_id=[{}]\n\
            \tpassword_file_path=[{}]\n\
            \t      boiler_id=[{}]\n\
            \tboiler_hostname=[{}]\n\
            \treading_values_delay=[{}] milliseconds\n\
            \t       log_level=[{:?}]\n\
            \t     dry_running=[{}]",
            self.user_id,
            self.password_file_path.display(),
            self.boiler_id,
            self.boiler_hostname,
            self.reading_values_delay,
            self.log_level,
            self.dry_running
        )
    }

    //Returns the password stored
    pub fn read_password(&self) -> String {
        log::debug!(
            "Getting password from [{}]",
            self.password_file_path.display()
        );

        // TODO: implement the reading
        "???".to_string()
    }
}
