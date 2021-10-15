extern crate mhlog;

use mhlog::{info,warn,err};

fn main() {
    info!("Information");
    warn!("Warning");
    err!("An error");
}