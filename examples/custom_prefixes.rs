#[macro_use]
extern crate mhlog;

fn main() {
    info!("Default info prefix");
    warn!("Default warning prefix");
    err!("Default error prefix");

    mhlog::info_prefix_str("Info:".to_string());
    mhlog::warning_prefix_str("Warning:".to_string());
    mhlog::error_prefix_str("Error:".to_string());

    info!("Custom info prefix");
    warn!("Custom warning prefix");
    err!("Custom prefix");
}
