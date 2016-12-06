// #![feature(step_by)]
#[macro_use] extern crate chomp;
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

mod day1;

fn main() {
    run_day!(day1);
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
    // run_day!(day13);
    // run_day!(day14);
    // run_day!(day15);
    // run_day!(day16);
    // run_day!(day17);
    // run_day!(day18);
    // run_day!(day19);
    // run_day!(day20);
    // run_day!(day21);
    // run_day!(day22);
    // run_day!(day23);
    // run_day!(day24);
    //run_day!(day25);
}
