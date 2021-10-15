![MHlog](./mhlog.png)

[![Travis CI build status](https://img.shields.io/travis/com/MHmorgan/mhlog/master?style=flat-square)](https://travis-ci.com/MHmorgan/mhlog)
[![Crates.io latest version](https://img.shields.io/crates/v/mhlog?style=flat-square)](https://crates.io/crates/mhlog)
![Crates.io downloads](https://img.shields.io/crates/d/mhlog?style=flat-square)
![GitHub license](https://img.shields.io/github/license/MHmorgan/mhlog?style=flat-square)
[![Gitpod Ready-to-Code](https://img.shields.io/badge/Gitpod-Ready--to--Code-blue?logo=gitpod)](https://gitpod.io/#https://github.com/MHmorgan/mhlog) 

MHlog is a simple, thread-safe logging library.

See the [documentation](https://docs.rs/mhlog/)

Changelog
---------

### v3.0.1

- Bail bugfix

### v3.0.0

- Update interface: `err`, `warn`, `info`, `verbose`, and `debug`.
- Better stdout/stderr defaults.
- Allow customization of message prefixes.

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
