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
#[cfg(feature = "colours")]
extern crate console;

#[cfg(feature = "colours")]
#[doc(hidden)]
pub use console::style;

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
    // Style the timestamp if colours enabled. Must be handlet differently
    // for stderr and stdout.
    #[cfg(feature = "colours")]
    let timestamp = match err || cfg!(not(feature = "log2stdout")) {
        true  => style(timestamp).for_stderr().cyan().dim(),
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
        #[cfg(not(feature = "colours"))]
        $crate::_log("Info: ", format!($($arg)+), false);
        #[cfg(feature = "colours")]
        match cfg!(feature = "log2stdout") {
            true  => $crate::_log($crate::style("Info: ").for_stdout().bold().green(), format!($($arg)+), false),
            false => $crate::_log($crate::style("Info: ").for_stderr().bold().green(), format!($($arg)+), false),
        }
    })
}

/// Print a warning log message, prefixed by a timestamp and _Warning_.
#[macro_export]
macro_rules! warn {
    ($($arg:tt)+) => (
        #[cfg(not(feature = "colours"))]
        $crate::_log("Warning: ", format!($($arg)+), true);
        #[cfg(feature = "colours")]
        $crate::_log($crate::style("Warning: ").for_stderr().bold().yellow(), format!($($arg)+), true);
    )
}

/// Print an error log message, prefixed by a timestamp and _Error_.
#[macro_export]
macro_rules! err {
    ($($arg:tt)+) => (
        #[cfg(not(feature = "colours"))]
        $crate::_log("Error: ", format!($($arg)+), true);
        #[cfg(feature = "colours")]
        $crate::_log($crate::style("Error: ").for_stderr().bold().red(), format!($($arg)+), true);
    )
}
