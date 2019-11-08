extern crate mhlog;

use mhlog::{fatal,error,important,info,debug,trace};

fn main() {

    // Logging without initialization
    debug!("Debug messages are not shown without initialization");
    important!("Without initialization important, error and fatal messages are shown");

    mhlog::init(mhlog::Lvl::Trace, "logtest", false)
        .expect("failed to initialize mhlog");

    trace!("log trace");
    debug!("log debug");
    info!("log info");
    important!("log important");
    error!("log error");
    fatal!("log fatal\nmultiline messages are also handled\ngoodbye");
}