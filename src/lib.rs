#![allow(unused)]
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;

#[macro_use]
pub mod macros;

pub mod utils;

pub fn get_input(mod_name: &str, prefix: &str) -> String {
    let path = format!("input/{}/{}", prefix, mod_name);
    let mut f = File::open(path).unwrap();
    let mut s = String::new();
    assert!(f.read_to_string(&mut s).is_ok());
    s.trim_end().to_string()
}

#[macro_export]
macro_rules! run_day {
    ($mod_name:ident) => {
        run_day!($mod_name, "now");
    };
    // this macro takes an argument of "type" `ident`
    // the `ident` designator is used for variable/function names
    ($mod_name:ident, $prefix:expr) => {
        mod $mod_name;
        fn main() {
            println!("Running {:?}...", stringify!($mod_name));
            let args: std::collections::HashSet<String> = std::env::args().skip(1).collect();
            let do_time = args.contains("-t") || args.contains("--time");
            let mut start_time = std::time::Instant::now();
            if args.contains("1") {
                println!(
                    "1> {:?}",
                    $mod_name::part1(get_input(stringify!($mod_name), $prefix))
                );
                if do_time {
                    println!("Part 1 took {} ms", start_time.elapsed().as_millis());
                }
            }
            if args.contains("2") {
                start_time = std::time::Instant::now();
                println!(
                    "2> {:?}",
                    $mod_name::part2(get_input(stringify!($mod_name), $prefix))
                );
                if do_time {
                    println!("Part 2 took {} ms", start_time.elapsed().as_millis());
                }
            }
            if !args.contains("1") && !args.contains("2") {
                println!("Specify part 1 or part 2");
            }
        }
    };
}

// run_day!(day19);
