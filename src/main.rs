extern crate caesarcy;

use caesarcy::{encipher,decipher};
use std::io;
use std::io::prelude::*;
use std::env;
use std::io::Write;
use std::process;

// Demo
fn main() {
    let help = "USAGE: caesarcy {encipher|decipher} [Text]";
    let args: Vec<_> = env::args().collect();
    let argc: usize = args.len();
    
    if argc == 1 {
        writeln!(&mut std::io::stderr(), "{}", help).unwrap();
        process::exit(1);
    } else if argc == 2 {
        let stdin = io::stdin();
        if args[1] == "encipher" {
            for line in stdin.lock().lines() {
                println!("{}", encipher(13, &line.unwrap()));
            }
        } else if args[1] == "decipher" {
            for line in stdin.lock().lines() {
                println!("{}", decipher(13, &line.unwrap()));
            }
        } else {
            writeln!(&mut std::io::stderr(), "{}", help).unwrap();
            process::exit(1);
        }
    } else if argc == 3 {
        if args[1] == "encipher" {
            println!("{}", encipher(13, &args[2]));
        } else if args[1] == "decipher" {
            println!("{}", decipher(13, &args[2]));
        } else {
            writeln!(&mut std::io::stderr(), "{}", help).unwrap();
            process::exit(1);
        }
    }
}
