extern crate mhlog;

use mhlog::{log,info,warn,err};

fn main() {
    log!("Log message. Prefixed with a timestamp. It's {}", "thread-safe!");
    info!("Logging message prefixed by '<timestamp> INFO:' ");
    warn!("Warning message prefixed by '<timestamp> WARN:' ");
    err!("Error message prefixed by '<timestamp> ERROR:' ");
}