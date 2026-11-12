//! Structured logging primitives for discord-rs.
//!
//! All output uses `eprintln!` / `writeln!` on `stderr` — no external crates.
//! Records carry a timestamp (seconds since `UNIX_EPOCH`), severity level,
//! scope string, and message, formatted as:
//!
//! ```text
//! [2026-11-12T18:00:00Z] [INFO ] [gateway] Connected to wss://...
//! ```

#![forbid(unsafe_code)]

use std::fmt;
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

/// Diagnostic severity level.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    /// Fine-grained protocol tracing.
    Debug,
    /// Normal operational information.
    #[default]
    Info,
    /// Recoverable problems or unexpected conditions.
    Warn,
    /// Failed operations; action required.
    Error,
}

impl Level {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Debug => "DEBUG",
            Self::Info => "INFO ",
            Self::Warn => "WARN ",
            Self::Error => "ERROR",
        }
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A structured log record.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Record<'a> {
    /// Wall-clock seconds since the Unix epoch (UTC).
    pub timestamp_secs: u64,
    /// Severity.
    pub level: Level,
    /// Component / subsystem name (e.g. `"gateway"`, `"rest"`).
    pub scope: &'a str,
    /// The formatted message text.
    pub message: &'a str,
}

impl fmt::Display for Record<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format: [YYYY-MM-DDTHH:MM:SSZ] [LEVEL] [scope] message
        let secs = self.timestamp_secs;
        let (y, mo, d, h, mi, s) = secs_to_datetime(secs);
        write!(
            f,
            "[{y:04}-{mo:02}-{d:02}T{h:02}:{mi:02}:{s:02}Z] [{level}] [{scope}] {message}",
            level = self.level,
            scope = self.scope,
            message = self.message,
        )
    }
}

/// Converts Unix seconds to `(year, month, day, hour, minute, second)`.
fn secs_to_datetime(secs: u64) -> (u64, u64, u64, u64, u64, u64) {
    let s = secs % 60;
    let total_min = secs / 60;
    let mi = total_min % 60;
    let total_hours = total_min / 60;
    let h = total_hours % 24;
    let total_days = total_hours / 24;

    // Gregorian calendar approximation sufficient for log display.
    let mut year = 1970u64;
    let mut remaining = total_days;
    loop {
        let leap = is_leap(year);
        let days_in_year = if leap { 366 } else { 365 };
        if remaining < days_in_year {
            break;
        }
        remaining -= days_in_year;
        year += 1;
    }

    let leap = is_leap(year);
    let month_days: [u64; 12] = [
        31,
        if leap { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];
    let mut month = 1u64;
    for &md in &month_days {
        if remaining < md {
            break;
        }
        remaining -= md;
        month += 1;
    }
    let day = remaining + 1;
    (year, month, day, h, mi, s)
}

const fn is_leap(year: u64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// A logging sink that receives structured [`Record`]s.
pub trait Logger: Send + Sync {
    /// Emits one structured log record.
    fn log(&self, record: &Record<'_>);

    /// Returns `true` if the logger will emit records at `level`.
    fn enabled(&self, level: Level) -> bool;
}

/// A no-op logger that discards all records.
#[derive(Clone, Copy, Debug, Default)]
pub struct NullLogger;

impl Logger for NullLogger {
    fn log(&self, _record: &Record<'_>) {}
    fn enabled(&self, _level: Level) -> bool {
        false
    }
}

/// A logger that writes structured records to `stderr`.
#[derive(Clone, Debug)]
pub struct StderrLogger {
    /// Minimum level to emit.
    pub min_level: Level,
}

impl StderrLogger {
    /// Creates a logger emitting all levels ≥ `min_level`.
    #[must_use]
    pub const fn new(min_level: Level) -> Self {
        Self { min_level }
    }
}

impl Default for StderrLogger {
    fn default() -> Self {
        Self::new(Level::Info)
    }
}

impl Logger for StderrLogger {
    fn log(&self, record: &Record<'_>) {
        if record.level >= self.min_level {
            let _ = writeln!(io::stderr(), "{record}");
        }
    }

    fn enabled(&self, level: Level) -> bool {
        level >= self.min_level
    }
}

/// A logger that captures records in memory — useful for testing.
#[derive(Debug)]
pub struct MemoryLogger {
    records: std::sync::Mutex<Vec<String>>,
    pub min_level: Level,
}

impl MemoryLogger {
    #[must_use]
    pub fn new(min_level: Level) -> Self {
        Self {
            records: std::sync::Mutex::new(Vec::new()),
            min_level,
        }
    }

    /// Returns all captured log lines.
    #[must_use]
    pub fn lines(&self) -> Vec<String> {
        self.records
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone()
    }

    /// Returns `true` if any line contains `needle`.
    #[must_use]
    pub fn contains(&self, needle: &str) -> bool {
        self.lines().iter().any(|l| l.contains(needle))
    }
}

impl Logger for MemoryLogger {
    fn log(&self, record: &Record<'_>) {
        if record.level >= self.min_level {
            let line = record.to_string();
            self.records
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .push(line);
        }
    }

    fn enabled(&self, level: Level) -> bool {
        level >= self.min_level
    }
}

// ----- Helper functions -------------------------------------------------------

/// Emits a [`Record`] through `logger` if the level is enabled.
pub fn emit(logger: &dyn Logger, level: Level, scope: &str, message: &str) {
    if logger.enabled(level) {
        let timestamp_secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        logger.log(&Record {
            timestamp_secs,
            level,
            scope,
            message,
        });
    }
}

/// Convenience wrappers.
pub fn debug(logger: &dyn Logger, scope: &str, message: &str) {
    emit(logger, Level::Debug, scope, message);
}

pub fn info(logger: &dyn Logger, scope: &str, message: &str) {
    emit(logger, Level::Info, scope, message);
}

pub fn warn(logger: &dyn Logger, scope: &str, message: &str) {
    emit(logger, Level::Warn, scope, message);
}

pub fn error(logger: &dyn Logger, scope: &str, message: &str) {
    emit(logger, Level::Error, scope, message);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn level_ordering_and_display() {
        assert!(Level::Debug < Level::Info);
        assert!(Level::Info < Level::Warn);
        assert!(Level::Warn < Level::Error);
        assert_eq!(Level::Debug.to_string(), "DEBUG");
        assert_eq!(Level::Info.to_string(), "INFO ");
        assert_eq!(Level::Warn.to_string(), "WARN ");
        assert_eq!(Level::Error.to_string(), "ERROR");
    }

    #[test]
    fn record_formats_correctly() {
        let record = Record {
            // 2024-01-01 00:00:00 UTC
            timestamp_secs: 1_704_067_200,
            level: Level::Info,
            scope: "gateway",
            message: "Connected",
        };
        let s = record.to_string();
        assert!(s.contains("[INFO ]"), "got: {s}");
        assert!(s.contains("[gateway]"), "got: {s}");
        assert!(s.contains("Connected"), "got: {s}");
        assert!(s.starts_with('['), "got: {s}");
    }

    #[test]
    fn memory_logger_captures_and_filters() {
        let logger = MemoryLogger::new(Level::Warn);
        info(&logger, "rest", "this should be filtered");
        warn(&logger, "rest", "this is a warning");
        error(&logger, "gateway", "something failed");

        let lines = logger.lines();
        assert_eq!(lines.len(), 2);
        assert!(logger.contains("[WARN ]"));
        assert!(logger.contains("[ERROR]"));
        assert!(!logger.contains("filtered"));
    }

    #[test]
    fn null_logger_discards_everything() {
        let logger = NullLogger;
        assert!(!logger.enabled(Level::Error));
        // Should not panic or write anything.
        emit(&logger, Level::Error, "test", "discarded");
    }

    #[test]
    fn secs_to_datetime_epoch() {
        let (y, mo, d, h, mi, s) = secs_to_datetime(0);
        assert_eq!((y, mo, d, h, mi, s), (1970, 1, 1, 0, 0, 0));
    }

    #[test]
    fn secs_to_datetime_known_timestamp() {
        // 2024-01-01 00:00:00 UTC = 1704067200
        let (y, mo, d, h, mi, s) = secs_to_datetime(1_704_067_200);
        assert_eq!((y, mo, d), (2024, 1, 1));
        assert_eq!((h, mi, s), (0, 0, 0));
    }
}
