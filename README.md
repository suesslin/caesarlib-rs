# caesarlib
Caesar cipher capabilities for Rust.<br>
[![Current Version](http://meritbadge.herokuapp.com/caesarlib)](https://crates.io/crates/caesarlib)
[![Build Status](https://travis-ci.org/caesarlib/caesarlib.rs.svg)](https://travis-ci.org/caesarlib/caesarlib.rs)

## Usage
Add this to your `Cargo.toml`:

```toml
[dependencies]
caesarlib = "0.1.2"
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
$ caesarlib encipher "Carolus Magnus" --offset 87
OmDAxGE YmszGE

$ caesarlib decipher "OmDAxGE YmszGE" --offset 87
Carolus Magnus

$ caesarlib encipher "Carolus Magnus" --offset 87 | caesarlib decipher --offset 87
Carolus Magnus
```

## Contributors
* [Lukas Mueller](https://github.com/luki)
* [Rémy Hubscher](https://github.com/natim)

## Background
This was inspired by my [Caesar Cipher algorithm
implentation in Swift](https://github.com/luki/CaesarCy/blob/master/Caesar/Algorithms.swift) used in my [iOS application caesarlib](https://github.com/luki/CaesarCy)
