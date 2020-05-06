//!
//! A tiny, simple, thread-safe logging library.
//! No configuration options, take it or leave it.
//!
//! Writes log messages to `stdout`/`stderr`. The writes are thread-safe.
//! If an error occurs when writing to the log file it panics.
//!
//! Provided logging macros:
//!
//! - [`log!()`]
//! - [`info!()`]
//! - [`warn!()`]
//! - [`err!()`]
//!
//! Usage
//! -----
//!
//! ```rust
//! extern crate mhlog;
//!
//! use mhlog::{log,info,warn,err};
//!
//! log!("Log message. Prefixed with a timestamp. It's {}", "thread-safe!");
//! info!("Logging message prefixed by '<timestamp> Info:' ");
//! warn!("Warning message prefixed by '<timestamp> Warning:' ");
//! err!("Error message prefixed by '<timestamp> Error:' ");
//! ```
//!
//! Features
//! --------
//!
//! ### Writing to stdout
//!
//! By default all log messages are printed to **stderr**. To make [`log!()`] and [`info!()`] print to **stdout** instead, enable the `log2stdout` feature.
//!
//! ```toml
//! [dependencies]
//! mhlog = { version = "*", features = ["log2stdout"] }
//! ```
//!
//! ### Coloured log messages
//!
//! Coloured log messages can be enabled with the `colours` feature.
//!
//! ```toml
//! [dependencies]
//! mhlog = { version = "*", features = ["colours"] }
//! ```
//!
//! [`log!()`]: macro.log.html
//! [`info!()`]: macro.info.html
//! [`warn!()`]: macro.warn.html
//! [`err!()`]: macro.err.html

extern crate chrono;
#[cfg(feature = "colours")]
extern crate console;

#[cfg(feature = "colours")]
use console::style;

use chrono::prelude::*;
use std::fmt::{self, Display};

// Time format in logging messages
const TIME_FMT: &'static str = "%F %T";

#[doc(hidden)]
pub fn _log(prefix: impl Display, msg: String, err: bool) {
    use std::io::{stderr, stdout, Write};

    let timestamp = Local::now().format(TIME_FMT).to_string();
    // Style the timestamp if colours enabled. Must be handlet differently
    // for stderr and stdout.
    #[cfg(feature = "colours")]
    let timestamp = match err || cfg!(not(feature = "log2stdout")) {
        true => style(timestamp).for_stderr().cyan().dim(),
        false => style(timestamp).for_stdout().cyan().dim(),
    };

    let txt = format!("{} {}{}\n", timestamp, prefix, msg);

    // Unless log2stdout enabled, always print to stderr.
    if err || cfg!(not(feature = "log2stdout")) {
        let _ = stderr().lock().write_all(txt.as_bytes());
    } else {
        let _ = stdout().lock().write_all(txt.as_bytes());
    }
}

/// Print a log message, prefixed by a timestamp.
#[macro_export]
macro_rules! log {
    ($($arg:tt)+) => (
        $crate::_log($crate::Prefix::None, format!($($arg)+), false);
    )
}

/// Print an info log message, prefixed by a timestamp and _Info_.
#[macro_export]
macro_rules! info {
    ($($arg:tt)+) => ({
        $crate::_log($crate::Prefix::Info, format!($($arg)+), false);
    })
}

/// Print a warning log message, prefixed by a timestamp and _Warning_.
#[macro_export]
macro_rules! warn {
    ($($arg:tt)+) => (
        $crate::_log($crate::Prefix::Warning, format!($($arg)+), true);
    )
}

/// Print an error log message, prefixed by a timestamp and _Error_.
#[macro_export]
macro_rules! err {
    ($($arg:tt)+) => (
        $crate::_log($crate::Prefix::Error, format!($($arg)+), true);
    )
}

#[doc(hidden)]
pub enum Prefix {
    None,
    Info,
    Warning,
    Error,
}

impl Display for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[cfg(not(feature = "colours"))]
        match self {
            Prefix::Info => write!(f, "Info: "),
            Prefix::Warning => write!(f, "Warning: "),
            Prefix::Error => write!(f, "Error: "),
            _ => write!(f, ""),
        }
        #[cfg(feature = "colours")]
        match self {
            Prefix::Info if cfg!(feature = "log2stdout") => {
                write!(f, "{}", style("Info: ").for_stdout().bold().green())
            }
            Prefix::Info => write!(f, "{}", style("Info: ").for_stderr().bold().green()),
            Prefix::Warning => write!(f, "{}", style("Warning: ").for_stderr().bold().yellow()),
            Prefix::Error => write!(f, "{}", style("Error: ").for_stderr().bold().red()),
            _ => write!(f, ""),
        }
    }
}
