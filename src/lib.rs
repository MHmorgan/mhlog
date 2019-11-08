//! mhlog is a simple, thread-safe logging library.
//! 
//! ```
//! extern crate mhlog;
//! 
//! use mhlog::{info,error};
//! 
//! mhlog::init(mhlog::Lvl::Info, "test", false);
//! info!("an info message");
//! error!("an error message...");
//! ```
//! 
//! Writes log messages to _stdout_/_stderr_, and optionally to a log file.
//! If an error occurs when writing to the log file it panics.

// TODO: 
//  - Add to crates.io

extern crate chrono;
extern crate dirs;
#[macro_use]
extern crate lazy_static;

use std::env;
use std::error::Error;
use std::fmt;
use std::fs::{self,File,OpenOptions};
use std::io::{self,Write};
use std::sync::{Once, Mutex};
use chrono::prelude::*;


// -----------------------------------------------------------------------------
// Globals variables

// Time format in logging messages
const TIME_FMT: &'static str = "%F %T";

// Synchronization variables
lazy_static! {
    static ref INIT: Once = Once::new();
    static ref MTX: Mutex<u32> = Mutex::new(0);
}

// State variables
static mut LOGFILE: Option<File> = None;
static mut LEVEL: u32 = Lvl::Important as u32;

type Result<T> = std::result::Result<T, LogError>;


/*******************************************************************************
 *                                                                             *
 *  Log error
 *                                                                             *
 *******************************************************************************/

/// Error returned by mhlog methods.
#[derive(Debug, Clone)]
pub struct LogError {
    message: String,
}

impl LogError {
    fn new<T: ToString>(msg: T) -> LogError {
        LogError { 
            message: msg.to_string(),
        }
    }
}

impl From<io::Error> for LogError {
    fn from(err: io::Error) -> Self {
        LogError {
            message: err.to_string(),
        }
    }
}

impl From<env::VarError> for LogError {
    fn from(err: env::VarError) -> Self {
        LogError {
            message: err.to_string(),
        }
    }
}

impl fmt::Display for LogError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for LogError {
    fn description(&self) -> &str {
        self.message.as_ref()
    }
}


/*******************************************************************************
 *                                                                             *
 *  Levels
 *                                                                             *
 *******************************************************************************/

/// Logging levels.
#[derive(Debug)]
pub enum Lvl {
    Fatal     = 0,
    Error     = 1,
    Warning   = 2,
    Important = 3,
    Info      = 4,
    Debug     = 5,
    Trace     = 6,
}

impl Lvl {

    /// Return logging level by matching the given environment variable string.
    /// 
    /// If the environment variable is empty or is unknown `Lvl::default()` is 
    /// returned.
    pub fn from_env(name: &str) -> Result<Self> {
        match env::var(name) {
            Ok(var) => Ok(Lvl::from(var.as_ref())),
            Err(e) => Err(LogError::from(e)),
        }
    }
}

impl Default for Lvl {
    fn default() -> Self { Lvl::Important }
}

impl From<&str> for Lvl {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_ref() {
            "fatal"     => Lvl::Fatal,
            "error"     => Lvl::Error,
            "warning"   => Lvl::Warning,
            "important" => Lvl::Important,
            "info"      => Lvl::Info,
            "debug"     => Lvl::Debug,
            "trace"     => Lvl::Trace,
            _           => Lvl::default(),
        }
    }
}


/*******************************************************************************
 *                                                                             *
 *  functions
 *                                                                             *
 *******************************************************************************/

/// Initialize mhlog. It is not necessary to do initialization before calling
/// the logging macros. Default logging level before initialization is performed
/// is `Lvl::Important`. 
/// 
/// Initialization can only be done once. If called more than 
/// once nothing happens. 
/// 
/// During initialization the logging level is set, and if `write2file` argument 
/// is `true` the log file is opened. `prog_name` argument is used to name the 
/// log file: `~/.log/<prog_name>.log`.
pub fn init<T: ToString>(level: Lvl, prog_name: T, write2file: bool) -> Result<()> {

    let mut res: Result<()> = Ok(());

    // Initialization only happens once
    INIT.call_once( || {

        if write2file {
            if let Some(home) = dirs::home_dir() {

                // Logs are located under ~/.log/
                let fpath = home.join(format!(".log/{}.log", prog_name.to_string()));

                // Make sure the .log directory exists
                if let Err(err) = fs::create_dir_all(fpath.parent().unwrap()) {
                    res = Err(LogError::from(err));
                    return
                }

                // Open log file
                let mut opts = OpenOptions::new();
                unsafe { 
                    LOGFILE = opts.write(true)
                                  .truncate(true)
                                  .create(true)
                                  .open(fpath)
                                  .ok(); 
                }

            } else {
                res = Err(LogError::new("cannot find home directory"));
            }
        }

        unsafe { LEVEL = level as u32; }
    });

    res
}

#[doc(hidden)]
pub fn _log(lvl: Lvl, prefix: &str, msg: String) {

    // Get value of globals
    let level = unsafe { LEVEL };
    let logfile = unsafe { &mut LOGFILE };

    if level >= lvl as u32 {

        let now = Local::now();
        let mut lines = msg.lines();
        
        // Make sure log messages don't overlap
        let _lock = MTX.lock();

        if let Some(file) = logfile {

            // Print prefix and date
            print!("{} {}: ", prefix, now.format(TIME_FMT).to_string());
            write!(file, "{} {}: ", prefix, now.format(TIME_FMT).to_string())
                .expect("failed to write to log file");

            // Print first line of message, prefixed by $prefix and date
            if let Some(first_line) = lines.next() {
                println!("{}", first_line);
                writeln!(file, "{}", first_line).expect("failed to write to log file");
            }

            // Print rest of the lines prefixed by [/]
            for line in lines {
                println!("[/]\t{}", line);
                writeln!(file, "[/]\t{}", line).expect("failed to write to log file");
            }

        } else {

            // Print prefix and date
            print!("{} {}: ", prefix, now.format(TIME_FMT).to_string());

            // Print first line of message, prefixed by $prefix and date
            if let Some(first_line) = lines.next() {
                println!("{}", first_line);
            }

            // Print rest of the lines prefixed by [/]
            for line in lines {
                println!("[/]\t{}", line);
            }

        }
    }
}


/*******************************************************************************
 *                                                                             *
 *  macros
 *                                                                             *
 *******************************************************************************/

#[doc(hidden)]
#[macro_export]
macro_rules! log {
    ($lvl:expr, $prefix:tt, $($arg:tt)+) => (
        $crate::_log($lvl, $prefix, format!($($arg)+));
    )
}

/// Print a fatal log message.   
/// Prefix: `[!!]`
#[macro_export]
macro_rules! fatal {
    ($($arg:tt)+) => (
        $crate::log!($crate::Lvl::Fatal, "[!!]", $($arg)+);
    )
}

/// Print an error log message.  
/// Prefix: `[!]`
#[macro_export]
macro_rules! error {
    ($($arg:tt)+) => (
        $crate::log!($crate::Lvl::Error, "[!]", $($arg)+);
    )
}

/// Print a warning log message.  
/// Prefix: `[-]`
#[macro_export]
macro_rules! warning {
    ($($arg:tt)+) => (
        $crate::log!($crate::Lvl::Warning, "[-]", $($arg)+);
    )
}

/// Print an important log message.   
/// Prefix: `[*]`
#[macro_export]
macro_rules! important {
    ($($arg:tt)+) => (
        $crate::log!($crate::Lvl::Important, "[*]", $($arg)+);
    )
}

/// Print an info log message.   
/// Prefix: `[ ]`
#[macro_export]
macro_rules! info {
    ($($arg:tt)+) => ({
        $crate::log!($crate::Lvl::Info, "[ ]", $($arg)+);
    })
}

/// Print a debug log message.  
/// Prefix: `[~]`
#[macro_export]
macro_rules! debug {
    ($($arg:tt)+) => (
        $crate::log!($crate::Lvl::Debug, "[~]", $($arg)+);
    )
}

/// Print a trace log message.   
/// Prefix: `[.]`
#[macro_export]
macro_rules! trace {
    ($($arg:tt)+) => (
        $crate::log!($crate::Lvl::Trace, "[.]", $($arg)+);
    )
}


/*******************************************************************************
 *                                                                             *
 *  Tests
 *                                                                             *
 *******************************************************************************/

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_lvl_from_env() {

        let var_name = "__TEST_LOG_VARIABLE_NAME__";
        env::set_var(var_name, "Error");
        assert_eq!(Lvl::from_env(var_name).unwrap() as u32, Lvl::Error as u32);

    }
} 

