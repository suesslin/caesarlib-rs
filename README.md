# caesarlib
Caesar cipher capabilities for Rust.<br>
[![Current Version](http://meritbadge.herokuapp.com/caesarlib)](https://crates.io/crates/caesarlib)
[![Build Status](https://travis-ci.org/caesarlib/caesarlib.rs.svg)](https://travis-ci.org/caesarlib/caesarlib.rs)

## Usage
Add this to your `Cargo.toml`:

```toml
[dependencies]
caesarlib = "0.2.0"
```

and this to your crate root:

```rust
extern crate caesarlib;

use caesarlib::*;
```

## Methods
```rust

// With custom base sequence
// Can look like "ABC...Zabc...z"

custom_encipher(base_seq: &str, offset: u16, message: &str) -> String

custom_decipher(base_seq: &str, offset: u16, message: &str) -> String

// With roman/latin characters
// Looks like the example for custom base sequences

fn encipher(offset: u16, message: &str) -> String;

fn decipher(offset: u16, message: &str) -> String;

// Returns randomly-generated offset and enciphered text
fn rdm_encipher(message: &str) -> (u16, String)
```

## CLI tool

You can try the lib using the `caesar-cli` tool:

```sh
$ caesarlib --method encipher --offset 87 --text "Carolus Magnus"
Result: lJaXUdb vJPWdb
With Offset: 87

$ caesarlib --method decipher --offset 87 —text "lJaXUdb vJPWdb"
Result: Carolus Magnus
With Offset: 87

$ caesarlib --random --text "Carolus Magnus"
Result: qOfcZig AOUbig
With Offset: 51312
```

## Tests
First, the directory of the location of caesarlib must be selected.
```sh
cd caesarlib.rs
```
Subsequently, cargo helps out with testing.
```sh
cargo run --verbose
```

## Dependencies
* [clap](https://github.com/kbknapp/clap-rs) - Command Line Argument Parser for Rust
* [rand](https://github.com/rust-lang-nursery/rand) - A Rust library for random number generators and other randomness functionality.

## Contributors
* [Lukas Mueller](https://github.com/luki)
* [Rémy Hubscher](https://github.com/natim)

# Versioning
[SemVer](http://semver.org/) applies for versioning. For the versions available, see the [crate](https://crates.io/crates/caesarlib)

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details

## Background
This was inspired by my [Caesar Cipher algorithm
implentation in Swift](https://github.com/luki/CaesarCy/blob/master/Caesar/Algorithms.swift) used in my [iOS application caesarlib](https://github.com/luki/CaesarCy)
