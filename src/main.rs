#![allow(unused)]
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn get_input(mod_name: &str) -> String {
    let path = format!("input/{}", mod_name);
    let mut f = File::open(path).unwrap();
    let mut s = String::new();
    assert!(f.read_to_string(&mut s).is_ok());
    s.trim().to_string()
}

macro_rules! veci{
    ( $( $e:expr, if $b:expr),*$(,)* ) => ({
        vec![
            $(
                if $b {Some($e)} else {None}
            ),*
        ].into_iter()
        .flat_map(|a| a)
        .collect()
    })
}

macro_rules! run_day {
    // this macro takes an argument of "type" `ident`
    // the `ident` designator is used for variable/function names
    ($mod_name:ident) => {
        mod $mod_name;
        fn main() {
            println!("Running {:?}...", stringify!($mod_name));
            let args: HashSet<String> = env::args().skip(1).collect();
            if args.contains("1") {
                println!(
                    "1> {:?}",
                    $mod_name::part1(get_input(stringify!($mod_name)))
                );
            }
            if args.contains("2") {
                println!(
                    "2> {:?}",
                    $mod_name::part2(get_input(stringify!($mod_name)))
                );
            }
            if !args.contains("1") && !args.contains("2") {
                println!("Specify part 1 or part 2");
            }
        }
    };
}

run_day!(day8);
