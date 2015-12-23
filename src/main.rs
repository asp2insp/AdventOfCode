#[macro_use] extern crate chomp;

use std::io::prelude::*;
use std::fs::File;

fn get_input(mod_name: &str) -> String {
    let path = format!("src/{}/input.txt", mod_name);
    let mut f = File::open(path).unwrap();
    let mut s = String::new();
    assert!(f.read_to_string(&mut s).is_ok());
    s
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
    run_day!(day1);
    run_day!(day2);
}
