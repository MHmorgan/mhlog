//! A tiny, simple, thread-safe logging library.
//! No configuration options, take it or leave it.
//!
//! Writes log messages to `stdout`/`stderr`. The writes are thread-safe.
//! 
//! If any of the mutexes protecting the state data (prefixes values, and verbose
//! and debug values) becomes poisoned it will panic.
//!
//! Provided logging macros:
//!
//! [`err!()`]  
//! [`warn!()`]  
//! [`info!()`]  
//! [`verbose!()`]  
//! [`debug!()`]  
//!
//! Usage
//! -----
//!
//! ```rust
//! # extern crate mhlog;
//! # use mhlog::{info, warn, err};
//! info!("An info message.");
//! warn!("A warning message.");
//! err!("An error message.");
//! ```
//! 
//! ### Custom log prefix
//! 
//! The prefix of the log messages may be changed by the user:
//! 
//! ```rust
//! # extern crate mhlog
//! # use mhlog::info;
//! mhlog::info_prefix_str("Info:".to_string());
//! info!("Hello custom world!");
//! ```
//! 
//! ### Dynamic log prefix
//! 
//! Dynamic log prefixes are also supported:
//! 
//! ```rust
//! # extern crate mhlog
//! # use mhlog::info;
//! mhlog::info_prefix_fn(|| format!("[{}]", "INFO"));
//! info!("Hello dynamic world!");
//! ```
//!
//! Features
//! --------
//!
//! ### Writing to stdout and stderr
//!
//! By default [`err!()`] and [`warn!()`] writes to stderr. The rest writes to stdout.
//! 
//! To force all logging to stderr enable the `only_stderr` feature:
//!
//! ```toml
//! [dependencies]
//! mhlog = { version = "*", features = ["only_stderr"] }
//! ```
//! 
//! Or, to force all logging to stdout enable the `only_stdout` feature:
//!
//! ```toml
//! [dependencies]
//! mhlog = { version = "*", features = ["only_stdout"] }
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
//! [`debug!()`]: macro.debug.html
//! [`verbose!()`]: macro.verbose.html
//! [`info!()`]: macro.info.html
//! [`warn!()`]: macro.warn.html
//! [`err!()`]: macro.err.html

#[macro_use]
extern crate lazy_static;
#[cfg(feature = "colours")]
extern crate console;

use std::sync::RwLock;

lazy_static! {
    static ref PREFIXES: RwLock<LogPrefixes> = RwLock::new(LogPrefixes::new());
    static ref VERBOSE: RwLock<bool> = RwLock::new(false);
    static ref DEBUG: RwLock<bool> = RwLock::new(false);
}

/// Print a message with the error prefix.
/// 
/// By default `err` will write to stderr. This can be changed with the `only_stdout` feature.
/// 
/// To change the error prefix use [`error_prefix_str`] or [`error_prefix_fn`].
/// 
/// [`error_prefix_fn`]: fn.error_prefix_fn.html
/// [`error_prefix_str`]: fn.error_prefix_str.html
#[macro_export]
macro_rules! err {
    ($($arg:tt)+) => (
        $crate::_log($crate::Severity::Err, format!($($arg)+));
    )
}

/// Print a message with the warning prefix.
/// 
/// By default `warn` will write to stderr. This can be changed with the `only_stdout` feature.
/// 
/// To change the warning prefix use [`warning_prefix_str`] or [`warning_prefix_fn`].
/// 
/// [`warning_prefix_fn`]: fn.warning_prefix_fn.html
/// [`warning_prefix_str`]: fn.warning_prefix_str.html
#[macro_export]
macro_rules! warn {
    ($($arg:tt)+) => (
        $crate::_log($crate::Severity::Warn, format!($($arg)+));
    )
}

/// Print a message with the info prefix.
/// 
/// By default `info` will write to stdout. This can be changed with the `only_stderr` feature.
/// 
/// To change the info prefix use [`info_prefix_str`] or [`info_prefix_fn`].
/// 
/// [`info_prefix_fn`]: fn.info_prefix_fn.html
/// [`info_prefix_str`]: fn.info_prefix_str.html
#[macro_export]
macro_rules! info {
    ($($arg:tt)+) => ({
        $crate::_log($crate::Severity::Info, format!($($arg)+));
    })
}

/// Print a message with the info prefix if verbose printing is enabled.
/// 
/// To enable verbose messages use [`set_verbose`].
/// 
/// By default `verbose` will write to stdout. This can be changed with the `only_stderr` feature.
/// 
/// To change the verbose prefix use [`info_prefix_str`] or [`info_prefix_fn`].
/// 
/// [`set_verbose`]: fn.set_verbose.html
/// [`info_prefix_fn`]: fn.info_prefix_fn.html
/// [`info_prefix_str`]: fn.info_prefix_str.html
#[macro_export]
macro_rules! verbose {
    ($($arg:tt)+) => ({
        $crate::_log($crate::Severity::Verbose, format!($($arg)+));
    })
}

/// Print a message with the debug prefix if debug printing is enabled.
/// 
/// To enable debug messages use [`set_debug`].
/// 
/// By default `debug` will write to stdout. This can be changed with the `only_stderr` feature.
/// 
/// To change the debug prefix use [`debug_prefix_str`] or [`debug_prefix_fn`].
/// 
/// [`set_debug`]: fn.set_debug.html
/// [`debug_prefix_fn`]: fn.debug_prefix_fn.html
/// [`debug_prefix_str`]: fn.debug_prefix_str.html
#[macro_export]
macro_rules! debug {
    ($($arg:tt)+) => ({
        $crate::_log($crate::Severity::Debug, format!($($arg)+));
    })
}

/// Print a message with the error prefix and exit with error code 1.
/// See [`err!()`]
/// 
/// [`err!()`]: macro.err.html
#[macro_export]
macro_rules! bail {
    ($($arg:tt)+) => ({
        $crate::err($($arg)+);
        std::process::exit(1);
    });
}

#[doc(hidden)]
pub fn _log(severity: Severity, msg: String) {
    use std::io::{stderr, stdout, Write};

    if severity.suppressed() {
        return
    }

    let txt = format!("{} {}\n", severity.prefix(), msg);
    #[cfg(feature = "colours")]
    let txt = severity.style(txt).to_string();

    if severity.to_stderr() {
        let _ = stderr().lock().write_all(txt.as_bytes());
    } else {
        let _ = stdout().lock().write_all(txt.as_bytes());
    }
}

// -----------------------------------------------------------------------------
// Config

/// Enable/disable debug messages.
/// 
/// By default debug messages are suppressed.
/// 
pub fn set_debug(val: bool) {
    let mut dbg = DEBUG.write().unwrap();
    *dbg = val;
}

/// Enable/disable verbose messages.
/// 
/// By default verbose messages are suppressed.
/// 
pub fn set_verbose(val: bool) {
    let mut v = VERBOSE.write().unwrap();
    *v = val;
}

/// Change the error prefix to a new static value.
pub fn error_prefix_str(s: String) {
    let mut pre = PREFIXES.write().unwrap();
    pre.err_str = s;
}

/// Change the warning prefix to a new static value.
pub fn warning_prefix_str(s: String) {
    let mut pre = PREFIXES.write().unwrap();
    pre.warn_str = s;
}

/// Change the info prefix to a new static value.
pub fn info_prefix_str(s: String) {
    let mut pre = PREFIXES.write().unwrap();
    pre.info_str = s;
}

/// Change the debug prefix to a new static value.
pub fn debug_prefix_str(s: String) {
    let mut pre = PREFIXES.write().unwrap();
    pre.debug_str = s;
}

/// Change the error prefix to a dynamic value.
pub fn error_prefix_fn(f: PrefixFn) {
    let mut pre = PREFIXES.write().unwrap();
    pre.err_fn = Some(f);
}

/// Change the warning prefix to a dynamic value.
pub fn warning_prefix_fn(f: PrefixFn) {
    let mut pre = PREFIXES.write().unwrap();
    pre.warn_fn = Some(f);
}

/// Change the info prefix to a dynamic value.
pub fn info_prefix_fn(f: PrefixFn) {
    let mut pre = PREFIXES.write().unwrap();
    pre.info_fn = Some(f);
}

/// Change the debug prefix to a dynamic value.
pub fn debug_prefix_fn(f: PrefixFn) {
    let mut pre = PREFIXES.write().unwrap();
    pre.debug_fn = Some(f);
}

// -----------------------------------------------------------------------------
// Severity

/// Log severity provides a common interface to all functionality which
/// depends on the severity of a log message. Such as colouring, prefix,
/// and stdout/stderr.
/// 
#[doc(hidden)]
pub enum Severity {
    Err,
    Warn,
    Info,
    Verbose,
    Debug,
}

impl Severity {
    #[cfg(feature = "colours")]
    pub fn style(&self, txt: String) -> console::StyledObject<String> {
        use console::style;
        use Severity::*;

        let obj = match self {
            Err => style(txt).red(),
            Warn => style(txt).yellow(),
            Info|Verbose => style(txt),
            Debug => style(txt).dim(),
        };

        if self.to_stderr() {
            obj.for_stderr()
        } else {
            obj.for_stdout()
        }
    }

    pub fn to_stderr(&self) -> bool {
        use Severity::*;
        if cfg!(feature = "only_stderr") {
            return true
        }
        if cfg!(feature = "only_stdout") {
            return false
        }
        match self {
            Err|Warn => true,
            _ => false,
        }
    }

    pub fn prefix(&self) -> String {
        use Severity::*;
        let pre = PREFIXES.read().unwrap();
        match self {
            Err => pre.err(),
            Warn => pre.warn(),
            Info|Verbose => pre.info(),
            Debug => pre.debug(),
        }
    }

    pub fn suppressed(&self) -> bool {
        use Severity::*;
        match self {
            Debug => !*DEBUG.read().unwrap(),
            Verbose => !*VERBOSE.read().unwrap(),
            _ => false,
        }
    }
}


// -----------------------------------------------------------------------------
// Prefixes

/// Function signature for prefix generators.
pub type PrefixFn = fn() -> String;

/// LogPrefixes maintains the state of prefix values, either
/// constant or generated for each message.
/// 
#[derive(Debug, Default)]
struct LogPrefixes {
    err_fn: Option<PrefixFn>,
    warn_fn: Option<PrefixFn>,
    info_fn: Option<PrefixFn>,
    debug_fn: Option<PrefixFn>,
    err_str: String,
    warn_str: String,
    info_str: String,
    debug_str: String,
}

impl LogPrefixes {
    pub fn new() -> Self {
        LogPrefixes {
            err_str: "[!!]".to_string(),
            warn_str: "[!]".to_string(),
            info_str: "[*]".to_string(),
            debug_str: "[~]".to_string(),
            ..Default::default()
        }
    }

    pub fn err(&self) -> String {
        match self.err_fn {
            Some(f) => f(),
            None => self.err_str.clone(),
        }
    }

    pub fn warn(&self) -> String {
        match self.warn_fn {
            Some(f) => f(),
            None => self.warn_str.clone(),
        }
    }

    pub fn info(&self) -> String {
        match self.info_fn {
            Some(f) => f(),
            None => self.info_str.clone(),
        }
    }

    pub fn debug(&self) -> String {
        match self.debug_fn {
            Some(f) => f(),
            None => self.debug_str.clone(),
        }
    }
}
