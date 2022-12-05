pub mod day18;
pub mod intcode;

use std::fs::File;
use std::io::prelude::*;

use itertools::Itertools;

pub fn get_input(mod_name: &str) -> String {
    let path = format!("input/{}", mod_name);
    let mut f = File::open(path).unwrap();
    let mut s = String::new();
    assert!(f.read_to_string(&mut s).is_ok());
    s.trim().to_string()
}

pub fn main() {
    let args = std::env::args().skip(1).collect_vec();
    let input = crate::get_input("twenty_nineteen/day18");
    if args.contains(&"1".to_owned()) {
        println!("1> {:?}", day18::part1(input.clone()));
    }
    if args.contains(&"2".to_owned()) {
        println!("2> {:?}", day18::part2(input.clone()));
    }
}
