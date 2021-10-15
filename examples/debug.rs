#[macro_use]
extern crate mhlog;

fn main() {
    debug!("This will not be printed.");
    mhlog::set_debug(true);
    debug!("Hello debug world!");
}