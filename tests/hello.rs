extern crate mhlog;

use mhlog::{fatal,error,important,info,debug,trace};

#[test]
fn mhlog_test() {
    mhlog::init(mhlog::Lvl::Debug, "logtest", true)
        .expect("failed to initialize mhlog");

    trace!("log trace");
    debug!("log debug");
    info!("log info");
    important!("log important");
    error!("log error");
    fatal!("log fatal");
}