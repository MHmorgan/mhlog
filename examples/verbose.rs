#[macro_use]
extern crate mhlog;

fn main() {
    verbose!("This will not be printed.");
    mhlog::set_verbose(true);
    verbose!("Hello verbose world!");
}