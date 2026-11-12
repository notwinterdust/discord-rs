//! Standard-library logging helpers.

#![forbid(unsafe_code)]

/// Diagnostic severity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    /// Detailed protocol information.
    Debug,
    /// Informational lifecycle output.
    Info,
    /// Recoverable problems.
    Warn,
    /// Failed operations.
    Error,
}

/// A logging sink.
pub trait Logger {
    /// Emits one log record.
    fn log(&self, level: Level, message: &str);
}

/// A logger that writes records to stderr.
#[derive(Clone, Copy, Debug, Default)]
pub struct StderrLogger;

impl Logger for StderrLogger {
    fn log(&self, level: Level, message: &str) {
        eprintln!("[{level:?}] {message}");
    }
}

/// Emits a record through the supplied logger.
pub fn log(logger: &impl Logger, level: Level, message: impl AsRef<str>) {
    logger.log(level, message.as_ref());
}
