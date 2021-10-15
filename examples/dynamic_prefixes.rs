#[macro_use]
extern crate mhlog;
extern crate chrono;

fn timestamp() -> String {
    use std::time::SystemTime;
    use chrono::Utc;
    chrono::DateTime::<Utc>::from(
        SystemTime::now()
    ).to_rfc3339()
}

fn main() {
    info!("Default info prefix");
    warn!("Default warning prefix");
    err!("Default error prefix");

    mhlog::info_prefix_fn(|| format!("[{}] Info:", timestamp()));
    mhlog::warning_prefix_fn(|| format!("[{}] Warning:", timestamp()));
    mhlog::error_prefix_fn(|| format!("[{}] Error:", timestamp()));

    info!("Dynamic info prefix");
    warn!("Dynamic warning prefix");
    err!("Dynamic prefix");
}
