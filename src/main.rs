// Copyright 2017 Rémy HUBSCHER.  All rights reserved.
// Use of this source code is governed by a MIT style
// license that can be found in the LICENSE.txt file.

#[macro_use]
extern crate clap;
extern crate caesarlib;

use caesarlib::{encipher,decipher,rdm_encipher};
use clap::{Arg, App};
use std::io;
use std::io::prelude::*;
use std::io::Write;
use std::process;

// Demo
fn main() {
    let matches = App::new("caesar-cli")
        .version(crate_version!())
        .about("Demo the caesarlib library with a simple CLI tool.")
        .arg(Arg::with_name("RANDOM")
            .long("random")
            .short("rdm"))
        .arg(Arg::with_name("METHOD")
            .long("method")
            .short("m")
            .possible_values(&["encipher", "decipher"])
            .takes_value(true)
            .required_unless("RANDOM"))
        .arg(Arg::with_name("OFFSET")
            .long("offset")
            .short("off")
            .takes_value(true)
            .required_unless("RANDOM"))
        .arg(Arg::with_name("TEXT")
            .long("text")
            .short("txt")
            .takes_value(true)
            .required(true))
        .get_matches();

    let input_text = matches.value_of("TEXT").unwrap();
    
    let mut result;

    match matches.is_present("RANDOM") {
        true => {
            result = rdm_encipher(input_text);
        },
        false => {
            let method = matches.value_of("METHOD").unwrap();
            let parsed_offset = match matches.value_of("OFFSET").unwrap().parse::<u16>() {
                Ok(num) => num,
                Err(why) => {
                    writeln!(&mut std::io::stderr(), "Offset should be a number: {}", why).unwrap();
                    process::exit(2);
                }
            };
            if method == "encipher" {
                result = (parsed_offset, encipher(parsed_offset, input_text));
            } else if method == "decipher" {
                result = (parsed_offset, decipher(parsed_offset, input_text));
            } else {
                panic!("An unknown method has been chosen.")
            }
        }
    }

    println!("Result: {}\nOffset: {}", result.0, result.1)
}
