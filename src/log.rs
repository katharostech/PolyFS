//! Contains logging configuration and setup functions

/// Configuration struct for PolyFS's logging behavior
pub struct LoggingConfig {
    /// The level of logs to include in output
    pub log_level: log::LevelFilter,
    /// Whether or not to output logs to stderr
    pub quiet: bool,
    /// Whether or not to log to syslog
    pub syslog: bool,
    /// File to log messages to
    pub log_file: Option<String>
}

impl Default for LoggingConfig {
    fn default() -> Self {
        LoggingConfig {
            log_level: log::LevelFilter::Warn,
            quiet: false,
            syslog: false,
            log_file: None
        }
    }
}

use fern::colors::{Color, ColoredLevelConfig};

/// Configure the logger for PolyFS
pub fn setup_logging(settings: LoggingConfig) -> Result<(), fern::InitError> {
    let colors = ColoredLevelConfig::new()
        .trace(Color::White)
        .debug(Color::BrightWhite)
        .info(Color::Green)
        .warn(Color::Yellow)
        .error(Color::Red);

    let mut config = fern::Dispatch::new()
        .level(settings.log_level);

    // Configuration for file and syslog loggers
    let mut file_config = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.target(),
                message
            ))
        });

    if settings.syslog {
        let syslog_fmt = syslog::Formatter3164 {
            facility: syslog::Facility::LOG_USER,
            hostname: None,
            process: "PolyFS".into(),
            pid: 0,
        };

        file_config = file_config.chain(
            syslog::unix(syslog_fmt).expect("FIXME: Need to convert this error")
        );
    }

    if let Some(log_file) = settings.log_file {
        file_config = file_config.chain(fern::log_file(log_file)?);
    }

    config = config.chain(file_config);

    if !settings.quiet {
        // Configuration for console logger
        let console_config = fern::Dispatch::new()
            .format(move |out, message, record| {
                out.finish(format_args!(
                    "[{} {} {}] {}",
                    chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"),
                    // Add colors to terminal output
                    colors.color(record.level()),
                    record.target(),
                    message
                ))
            })
            .chain(std::io::stderr());
        config = config.chain(console_config);
    }

    config.apply()?;

    Ok(())
}
