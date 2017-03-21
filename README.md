# caesarlib
Caesar cipher capabilities for Rust.<br>
[![Current Version](http://meritbadge.herokuapp.com/caesarlib)](https://crates.io/crates/caesarlib)
[![Build Status](https://travis-ci.org/caesarlib/caesarlib.rs.svg)](https://travis-ci.org/caesarlib/caesarlib.rs)

## Usage
Add this to your `Cargo.toml`:

```toml
[dependencies]
caesarlib = "0.1.3"
```

and this to your crate root:

```rust
extern crate caesarlib;

use caesarlib::*;
```

## Methods
```rust
fn encipher(offset: u16, message: &str) -> String;

fn decipher(offset: u16, message: &str) -> String;

// Returns randomly-generated offset and enciphered text
fn rdm_encipher(message: &str) -> (u16, String)
```

## CLI tool

You can try the lib using the `caesar-cli` tool:

```sh
$ cargo run -- --method encipher --offset 87 --text "Carolus Magnus"
Result: lJaXUdb vJPWdb
With Offset: 87

$ cargo run -- --method decipher --offset 87 —text "lJaXUdb vJPWdb"
Result: Carolus Magnus
With Offset: 87

$ cargo run -- --random --text "Carolus Magnus"
Result: oMdaXge yMSZge
With Offset: xyz
```

## Contributors
* [Lukas Mueller](https://github.com/luki)
* [Rémy Hubscher](https://github.com/natim)

## Background
This was inspired by my [Caesar Cipher algorithm
implentation in Swift](https://github.com/luki/CaesarCy/blob/master/Caesar/Algorithms.swift) used in my [iOS application caesarlib](https://github.com/luki/CaesarCy)
