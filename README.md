mhlog
=====

![Latest version](https://img.shields.io/crates/v/mhlog.svg)
![Documentation](https://docs.rs/mhlog/badge.svg)
![GitHub license](https://img.shields.io/github/license/MHmorgan/rustmhlog)

mhlog is a simple, thread-safe logging library.

Usage
-----

```toml
[dependencies]
mhlog = "1.0"
```

```rust
extern crate mhlog;

use mhlog::{info,error};

mhlog::init(mhlog::Lvl::Info, "test", false);
info!("an info message");
error!("an error message...");
```

Writes log messages to _stdout_/_stderr_, and optionally to a log file.
If an error occurs when writing to the log file it panics.

Each log message is prefixed, indicating the logging level. Logging levels are:

- Fatal
- Error
- Warning
- Important
- Info
- Debug
- Trace


Changelog
---------
