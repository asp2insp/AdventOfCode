#![allow(unused)]
#[macro_use]
extern crate chomp;
extern crate time;
extern crate crypto;
extern crate itertools;
extern crate serde_json;
extern crate permutohedron;
extern crate rand;
extern crate regex;
extern crate chrono;
extern crate parking_lot;
extern crate rayon;
extern crate image;

#[macro_use]
extern crate lazy_static;

use std::io::prelude::*;
use std::fs::File;
use std::env;

fn get_input(mod_name: &str) -> String {
    let path = format!("input/{}", mod_name);
    let mut f = File::open(path).unwrap();
    let mut s = String::new();
    assert!(f.read_to_string(&mut s).is_ok());
    s.trim().to_string()
}

macro_rules! run_day {
    // this macro takes an argument of "type" `ident`
    // the `ident` designator is used for variable/function names
    ($mod_name:ident) => (
        println!("Running {:?}...", stringify!($mod_name));
        let mut run_one = false;
        let mut run_two = false;
        let mut args = env::args().skip(1);
        match args.next() {
            Some(s) => {
                match s.as_str() {
                    "1" => run_one = true,
                    "2" => run_two = true,
                    _ => {},
                };
            },
            None => {},
        };
        match args.next() {
            Some(s) => {
                match s.as_str() {
                    "1" => run_one = true,
                    "2" => run_two = true,
                    _ => {},
                };
            },
            None => {},
        };
        if run_one {
            println!("1> {:?}", $mod_name::part1(get_input(stringify!($mod_name))));
        }
        if run_two {
            println!("2> {:?}", $mod_name::part2(get_input(stringify!($mod_name))));
        }
    )
}

mod day11;

fn main() {
    run_day!(day11);
}