pub mod day18;
pub mod intcode;

use std::fs::File;
use std::io::prelude::*;

pub fn get_input(mod_name: &str) -> String {
    let path = format!("input/{}", mod_name);
    let mut f = File::open(path).unwrap();
    let mut s = String::new();
    assert!(f.read_to_string(&mut s).is_ok());
    s.trim().to_string()
}

pub fn main() {
    let input = crate::get_input("twenty_nineteen/day18");
    println!("1> {:?}", day18::part1(input.clone()));
    println!("2> {:?}", day18::part2(input.clone()));
}