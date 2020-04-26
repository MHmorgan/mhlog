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
mhlog = "~2.0.0"
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


Changelog
---------

### v2.0.0

- Even simpler interface
- Reduced interface to only `err`, `warn`, `info`, and `log`
- No writing to file, only stdout (should pipe log output instead)


### v1.1.0

- Add support for verbose logging messages


### v1.0.1

- Added example
