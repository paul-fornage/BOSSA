BOSSA (Rust-edition)
--------------------

[![Audit Dependencies](https://github.com/kiibohd/BOSSA/actions/workflows/audit.yml/badge.svg)](https://github.com/kiibohd/BOSSA/actions/workflows/audit.yml)
[![Legacy Makefile Build Environment](https://github.com/kiibohd/BOSSA/actions/workflows/build.yml/badge.svg)](https://github.com/kiibohd/BOSSA/actions/workflows/build.yml)
[![Rust Linux](https://github.com/kiibohd/BOSSA/actions/workflows/rust_linux.yml/badge.svg)](https://github.com/kiibohd/BOSSA/actions/workflows/rust_linux.yml)
[![Rust macOS](https://github.com/kiibohd/BOSSA/actions/workflows/rust_macos.yml/badge.svg)](https://github.com/kiibohd/BOSSA/actions/workflows/rust_macos.yml)
[![Rust Windows](https://github.com/kiibohd/BOSSA/actions/workflows/rust_windows.yml/badge.svg)](https://github.com/kiibohd/BOSSA/actions/workflows/rust_windows.yml)


[![docs.rs](https://docs.rs/bossa/badge.svg)](https://docs.rs/bossa)
[![Crates.io](https://img.shields.io/crates/v/bossa.svg)](https://crates.io/crates/bossa)
[![Crates.io](https://img.shields.io/crates/l/bossa.svg)](https://crates.io/crates/bossa)
[![Crates.io](https://img.shields.io/crates/d/bossa.svg)](https://crates.io/crates/bossa)

BOSSA is a flash programming utility for Atmel's SAM family of flash-based ARM microcontrollers.
The motivation behind BOSSA is to create a simple, easy-to-use, open source utility to replace Atmel's SAM-BA software.
BOSSA is an acronym for Basic Open Source SAM-BA Application to reflect that goal.

The software was originally by Scott Shumate with contributions from several [contributors](../../graphs/contributors). Scott has not been active in a number of years and HaaTa needed a rust-lib version of BOSSA, hence this fork.

The software is released under the terms of the BSD license as specified in the LICENSE file.

Supported Builds
----------------

* Rust (libbossa, bossac)
  - `cargo build`
  - `cd bossac; cargo build; cargo run -- --help`
  - Supports: msvc, macos, linux
* C++ (bossa, bossac, bossash)
  - `make -j`
  - Supports: mingw, macos, linux
    msvc support hasn't been tested recently

Please see the [GitHub Actions](.github/workflows) for build specifics.

Installation
------------

If you already have rust on your system, you can use it to easily install bossac.

```bash
cargo install bossac
```

Supported Device Families
-------------------------
 * SAM7S
 * SAM7SE
 * SAM7X
 * SAM7XC
 * SAM3N
 * SAM3S
 * SAM3U
 * SAM4E
 * SAM4S
 * SAMD21
 * SAMD51
 * SAM3X\*
 * SAM3A\*
 * SAM7L\*
 * SAM9XE\*
 * SAMR21\*
 * SAML21\*
 * SAME51\*
 * SAME53\*
 * SAME54\*
 * SAME70\*
 * SAMS70\*
 * SAMV70\*
 * SAMV71\*

\* Device families which are not tested for each release and could stop working.

Please submit a PR if you're interested in fixing or adding support.

*Actively looking for a co-maintainer for BOSSA*

The following individuals and companies graciously provided development boards to assist the BOSSA project.
 * Atmel Corporation (SAM3N, SAM3S, SAM3U)
 * David Crocker (SAM4E, SAM4S)
 * Adafruit Industries (SAMD21, SAMD51)
