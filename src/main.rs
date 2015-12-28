#[macro_use] extern crate chomp;
extern crate crypto;
extern crate itertools;
extern crate serde_json;
extern crate permutohedron;


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
        mod $mod_name;
        println!("Running {:?}...", stringify!($mod_name));
        println!("1> {:?}", $mod_name::part1(get_input(stringify!($mod_name))));
        println!("2> {:?}", $mod_name::part2(get_input(stringify!($mod_name))));
    )
}

fn main() {
    // run_day!(day1);
    // run_day!(day2);
    // run_day!(day3);
    // run_day!(day4);
    // run_day!(day5);
    // run_day!(day6);
    // run_day!(day7);
    // run_day!(day8);
    // run_day!(day9);
    // run_day!(day10);
    // run_day!(day11);
    // run_day!(day12);
    run_day!(day13);

}
