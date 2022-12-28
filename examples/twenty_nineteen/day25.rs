use aoc::utils::*;
use crate::intcode::*;
use itertools::Itertools;

pub fn part1(s: String) -> String {
    let prog = gimme_nums(&s).into_iter().flatten().collect_vec();
    let mut c = Computer::new(prog);
    let mut line = String::new();
    println!("Commands: take $, drop $, inv, n/e/s/w");
    loop {
        while let ProgYield::Output(c) = c.run_and_return_output() {
            print!("{}", char::from_u32(c as u32).unwrap());
        }
        std::io::stdin().read_line(&mut line).unwrap();
        c.input_ascii(&[&line.trim()]);
        line.clear();
    }
}


pub fn part2(s: String) -> String {
    "part2".to_owned()
}
