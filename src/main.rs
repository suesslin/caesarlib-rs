// Copyright 2017 Rémy HUBSCHER.  All rights reserved.
// Use of this source code is governed by a MIT style
// license that can be found in the LICENSE.txt file.

#[macro_use]
extern crate clap;
extern crate caesarlib;

use caesarlib::{encipher,decipher};
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
        .arg(Arg::with_name("MODE")
             .help("The action you want to do on the text.")
             .possible_values(&["encipher", "decipher"])
             .required(true)
             .index(1))
        .arg(Arg::with_name("offset")
             .short("s")
             .long("offset")
             .default_value("13")
             .help("Set the offset of the caesar code.")
             .takes_value(true))
        .arg(Arg::with_name("TEXT")
             .index(2)
             .help("The text to process. If not present, will read from stdin."))
        .get_matches();

    let mut lines : Vec<String> = Vec::new();
    if matches.is_present("TEXT") {
        lines.push(String::from(matches.value_of("TEXT").unwrap()));
    } else {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            lines.push(line.unwrap());
        }
    }


    let offset_parser = matches.value_of("offset").unwrap().parse::<i32>();

    match offset_parser {
        Ok(offset) => {
            match matches.value_of("MODE").unwrap() {
                "encipher" => {
                    println!("{}", encipher(offset, &lines.join("\n")));
                },
                "decipher" => {
                    println!("{}", decipher(offset, &lines.join("\n")));
                },
                _ => unreachable!()
            }
        },

        Err(why) => {
            writeln!(&mut std::io::stderr(), "Offset should be a number: {}", why).unwrap();
            process::exit(2);
        },
    }
}
