#[macro_use]
extern crate mhlog;

#[test]
fn logging() {
    log!("{} statement!", "log!()");
    info!("{} statement!", "info!()");
    warn!("{} statement!", "warn!()");
    err!("{} statement!", "err!()");
}