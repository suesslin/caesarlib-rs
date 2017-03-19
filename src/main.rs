// Copyright 2017 ______.  All rights reserved.
// Use of this source code is governed by a MIT style
// license that can be found in the LICENSE file.

extern crate caesarlib;

use caesarlib::{encipher,decipher};
use std::io;
use std::io::prelude::*;
use std::env;
use std::io::Write;
use std::process;

static HELP: &'static str = "USAGE: caesarlib {encipher|decipher} [Text]";

fn handle_cli_params(mode : &String, offset : i32, text : String) -> i32 {
    if mode == "encipher" {
        println!("{}", encipher(offset, &text));
    } else if mode == "decipher" {
        println!("{}", decipher(offset, &text));
    } else {
        writeln!(&mut std::io::stderr(), "{}", HELP).unwrap();
        return 1;
    }
    return 0;
}

// Demo
fn main() {
    let args: Vec<_> = env::args().collect();
    let argc: usize = args.len();

    if argc == 1 {
        writeln!(&mut std::io::stderr(), "{}", HELP).unwrap();
        process::exit(1);
    } else if argc == 2 {
        let mode = args[1].clone();
        let stdin = io::stdin();
        let mut ret = 0;
        for line in stdin.lock().lines() {
            ret = handle_cli_params(&mode, 13, line.unwrap());
        }
        process::exit(ret);
    } else if argc == 3 {
        let mode = args[1].clone();
        let text = args[2].clone();
        process::exit(handle_cli_params(&mode, 13, text));
    }
}
