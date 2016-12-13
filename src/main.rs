// #![feature(step_by)]
#[macro_use]
extern crate chomp;
extern crate time;
extern crate crypto;
extern crate itertools;
extern crate serde_json;
extern crate permutohedron;
extern crate rand;

use std::io::prelude::*;
use std::fs::File;

fn get_input(mod_name: &str) -> String {
    let path = format!("src/{}/input.txt", mod_name);
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
        println!("1> {:?}", $mod_name::part1(get_input(stringify!($mod_name))));
        println!("2> {:?}", $mod_name::part2(get_input(stringify!($mod_name))));
    )
}

//mod day1;
//mod day2;
//mod day3;
//mod day4;
//mod day5;
mod day6;

fn main() {
    run_day!(day6);
}
