# caesarcy
Caesar cipher capabilities for Rust.<br>
[![Current Version](http://meritbadge.herokuapp.com/caesarcy)](https://crates.io/crates/caesarcy)

## Usage
Add this to your `Cargo.toml`:

```toml
[dependencies]
caesarcy = "0.1.1"
```

and this to your crate root:

```rust
extern crate caesarcy;

use caesarcy::*;
```

## Methods
```rust
fn encipher(offset: i32, message: &str) -> String;

fn decipher(offset: i32, message: &str) -> String;
```

## Author
* [Lukas Mueller](https://crates.io/crates/caesarcy)

## Background
This was inspired by my [Caesar Cipher algorithm
implentation in Swift](https://github.com/luki/CaesarCy/blob/master/Caesar/Algorithms.swift) used in my [iOS application CaesarCy](https://github.com/luki/CaesarCy)
