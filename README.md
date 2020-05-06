MHlog
=====

[![Travis CI build status](https://img.shields.io/travis/com/MHmorgan/mhlog/master?style=flat-square)](https://travis-ci.com/MHmorgan/mhlog)
[![Crates.io latest version](https://img.shields.io/crates/v/mhlog?style=flat-square)](https://crates.io/crates/mhlog)
![Crates.io downloads](https://img.shields.io/crates/d/mhlog?style=flat-square)
![GitHub license](https://img.shields.io/github/license/MHmorgan/mhlog?style=flat-square)

MHlog is a simple, thread-safe logging library.

Usage
-----

```toml
[dependencies]
mhlog = "2.1.0"
```

```rust
extern crate mhlog;

use mhlog::{log,info,warn,err};

log!("Log message. Prefixed with a timestamp. It's {}", "thread-safe!");
info!("Logging message prefixed by '<timestamp> Info:' ");
warn!("Warning message prefixed by '<timestamp> Warning:' ");
err!("Error message prefixed by '<timestamp> Error:' ");
```

Writes log messages to `stdout`/`stderr`. The writes are thread-safe.
If an error occurs when writing to the log file it panics.

Provided logging macros:

- `log!()`
- `info!()`
- `warn!()`
- `err!()`

Features
--------

### Writing to stdout

By default all log messages are printed to **stderr**. To make `log!()` and `info!()` print to **stdout** instead, enable the `log2stdout` feature.

```toml
[dependencies]
mhlog = { version = "*", features = ["log2stdout"] }
```

### Coloured log messages

Coloured log messages can be enabled with the `colours` feature.

```toml
[dependencies]
mhlog = { version = "*", features = ["colours"] }
```

Changelog
---------

### v2.1.2

- Bugfix

### v2.1.1

- Update crate documentation.

### v2.1.0

- Log all messages to **stderr** by default. Can be disabled with feature `log2stdout`.
- Add support for coloured log messages with `coloured` feature.

### v2.0.0

- Even simpler interface
- Reduced interface to only `err`, `warn`, `info`, and `log`
- No writing to file, only stdout (should pipe log output instead)


### v1.1.0

- Add support for verbose logging messages


### v1.0.1

- Added example
