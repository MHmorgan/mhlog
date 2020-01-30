mhlog
=====

[![Latest version](https://img.shields.io/crates/v/mhlog.svg)](https://crates.io/crates/mhlog)
![Downloads](https://img.shields.io/crates/d/mhlog)
[![Documentation](https://docs.rs/mhlog/badge.svg)](https://docs.rs/mhlog/)
[![GitHub license](https://img.shields.io/github/license/MHmorgan/rustmhlog)](https://github.com/MHmorgan/rustmhlog/blob/master/LICENSE)

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

### v1.1.0

- Add support for verbose logging messages


### v1.0.1

- Added example
