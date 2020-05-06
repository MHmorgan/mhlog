//!
//! A tiny, simple, thread-safe logging library.
//! No configuration options, take it or leave it.
//!
//! Writes log messages to `stdout`/`stderr`. The writes are thread-safe.
//! If an error occurs when writing to the log file it panics.
//!
//! Provided logging macros:
//!
//! - `log!()`
//! - `info!()`
//! - `warn!()`
//! - `err!()`
//!
//! Usage
//! -----
//!
//! Cargo.toml:
//! ```toml
//! [dependencies]
//! mhlog = "~2.0.0"
//! ```
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

extern crate chrono;

use chrono::prelude::*;
use std::fmt::Display;

// -----------------------------------------------------------------------------
// Globals variables

// Time format in logging messages
const TIME_FMT: &'static str = "%F %T";

#[doc(hidden)]
pub fn _log(prefix: impl Display, msg: String, err: bool) {
    use std::io::{stderr, stdout, Write};

    let timestamp = Local::now().format(TIME_FMT).to_string();
    let txt = format!("{} {}{}\n", timestamp, prefix, msg);

    if err || cfg!(not(feature = "log2stdout")) {
        let _ = stderr().lock().write_all(txt.as_bytes());
    } else {
        let _ = stdout().lock().write_all(txt.as_bytes());
    }
}

/*******************************************************************************
 *                                                                             *
 *  macros
 *                                                                             *
 *******************************************************************************/

/// Print a log message, prefixed by a timestamp.
#[macro_export]
macro_rules! log {
    ($($arg:tt)+) => (
        $crate::_log("", format!($($arg)+), false);
    )
}

/// Print an info log message, prefixed by a timestamp and _Info_.
#[macro_export]
macro_rules! info {
    ($($arg:tt)+) => ({
        $crate::_log("Info: ", format!($($arg)+), false);
    })
}

/// Print a warning log message, prefixed by a timestamp and _Warning_.
#[macro_export]
macro_rules! warn {
    ($($arg:tt)+) => (
        $crate::_log("Warning: ", format!($($arg)+), true);
    )
}

/// Print an error log message, prefixed by a timestamp and _Error_.
#[macro_export]
macro_rules! err {
    ($($arg:tt)+) => (
        $crate::_log("Error: ", format!($($arg)+), true);
    )
}
