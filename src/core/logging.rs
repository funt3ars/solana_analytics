use std::{path::PathBuf, sync::Once};
use tracing::{Level, Subscriber};
use tracing_subscriber::{
    fmt::{format::FmtSpan, time::UtcTime, writer::MakeWriterExt},
    prelude::*,
    EnvFilter,
    layer::Layer,
};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use std::sync::Arc;

/// Configuration for logging
#[derive(Debug, Clone)]
pub struct LogConfig {
    /// Log level
    pub level: Level,
    
    /// Whether to use JSON format
    pub json: bool,
    
    /// Log file path
    pub file_path: Option<PathBuf>,
    
    /// Log file rotation
    pub rotation: Rotation,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: Level::INFO,
            json: false,
            file_path: None,
            rotation: Rotation::DAILY,
        }
    }
}

impl LogConfig {
    /// Create a new logging configuration
    pub fn new(
        level: Level,
        json: bool,
        file_path: Option<PathBuf>,
        rotation: Rotation,
    ) -> Self {
        Self {
            level,
            json,
            file_path,
            rotation,
        }
    }
}

static INIT: Once = Once::new();

/// Initialize logging with the given configuration
pub fn init(config: LogConfig) {
    INIT.call_once(|| {
        let env_filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new(format!("{}", config.level)));

        if let Some(file_path) = config.file_path {
            let file_appender = RollingFileAppender::new(
                config.rotation,
                file_path.parent().unwrap_or(&PathBuf::from(".")),
                file_path.file_name().unwrap().to_str().unwrap(),
            );

            let file_layer = tracing_subscriber::fmt::layer()
                .with_writer(file_appender)
                .with_timer(UtcTime::rfc_3339())
                .with_span_events(FmtSpan::CLOSE)
                .with_target(true)
                .with_thread_ids(true)
                .with_file(true)
                .with_line_number(true);

            tracing_subscriber::registry()
                .with(env_filter)
                .with(file_layer)
                .init();
        } else if config.json {
            tracing_subscriber::registry()
                .with(env_filter)
                .with(
                    tracing_subscriber::fmt::layer()
                        .json()
                        .with_timer(UtcTime::rfc_3339())
                        .with_span_events(FmtSpan::CLOSE)
                        .with_target(true)
                        .with_thread_ids(true)
                        .with_file(true)
                        .with_line_number(true),
                )
                .init();
        } else {
            tracing_subscriber::registry()
                .with(env_filter)
                .with(
                    tracing_subscriber::fmt::layer()
                        .with_timer(UtcTime::rfc_3339())
                        .with_span_events(FmtSpan::CLOSE)
                        .with_target(true)
                        .with_thread_ids(true)
                        .with_file(true)
                        .with_line_number(true),
                )
                .init();
        }
    });
}

/// Initialize logging with default level (INFO)
pub fn init_default_logging() {
    init(LogConfig::default());
}

/// Initialize logging with debug level
pub fn init_debug_logging() {
    let mut config = LogConfig::default();
    config.level = Level::DEBUG;
    init(config);
}

/// Initialize logging with trace level
pub fn init_trace_logging() {
    let mut config = LogConfig::default();
    config.level = Level::TRACE;
    init(config);
}

/// Initialize logging with error level
pub fn init_error_logging() {
    let mut config = LogConfig::default();
    config.level = Level::ERROR;
    init(config);
}

/// Initialize logging with JSON output
pub fn init_json_logging() {
    let mut config = LogConfig::default();
    config.json = true;
    init(config);
}

/// Initialize logging with file output
pub fn init_file_logging(file_path: PathBuf) {
    let mut config = LogConfig::default();
    config.file_path = Some(file_path);
    init(config);
}

/// Initialize logging with rotation
pub fn init_rotating_logging(file_path: PathBuf, rotation: Rotation) {
    let mut config = LogConfig::default();
    config.file_path = Some(file_path);
    config.rotation = rotation;
    init(config);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;
    use tracing::{debug, error, info, trace, warn};

    #[test]
    fn test_log_config_default() {
        let config = LogConfig::default();
        assert_eq!(config.level, Level::INFO);
        assert!(!config.json);
        assert!(config.file_path.is_none());
        assert_eq!(config.rotation, Rotation::DAILY);
    }

    #[test]
    fn test_log_config_new() {
        let level = Level::DEBUG;
        let json = true;
        let file_path = Some(PathBuf::from("test.log"));
        let rotation = Rotation::NEVER;
        let rotation_clone = rotation.clone();

        let config = LogConfig::new(level, json, file_path.clone(), rotation);
        assert_eq!(config.level, level);
        assert_eq!(config.json, json);
        assert_eq!(config.file_path, file_path);
        assert_eq!(config.rotation, rotation_clone);
    }

    #[test]
    fn test_logging_initialization() {
        init_default_logging();
        info!("Test info message");
        debug!("Test debug message");
        trace!("Test trace message");
        warn!("Test warning message");
        error!("Test error message");
    }

    #[test]
    fn test_debug_logging() {
        init_debug_logging();
        debug!("Debug message should be visible");
    }

    #[test]
    fn test_trace_logging() {
        init_trace_logging();
        trace!("Trace message should be visible");
    }

    #[test]
    fn test_error_logging() {
        init_error_logging();
        error!("Error message should be visible");
    }

    #[test]
    fn test_json_logging() {
        init_json_logging();
        info!("JSON formatted log message");
    }

    #[test]
    fn test_file_logging() {
        let temp_dir = tempdir().unwrap();
        let log_file = temp_dir.path().join("test.log");
        
        init_file_logging(log_file.clone());
        info!("Test file log message");
        
        assert!(log_file.exists());
        let contents = fs::read_to_string(&log_file).unwrap();
        assert!(contents.contains("Test file log message"));
    }

    #[test]
    fn test_rotating_logging() {
        let temp_dir = tempdir().unwrap();
        let log_file = temp_dir.path().join("rotating.log");
        
        init_rotating_logging(log_file.clone(), Rotation::DAILY);
        info!("Test rotating log message");
        
        assert!(log_file.exists());
        let contents = fs::read_to_string(&log_file).unwrap();
        assert!(contents.contains("Test rotating log message"));
    }
} 