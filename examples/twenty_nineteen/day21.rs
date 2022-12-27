use crate::intcode::*;
use aoc::utils::*;
use itertools::Itertools;

pub fn part1(s: String) -> String {
    let prog = gimme_nums(&s).into_iter().flatten().collect_vec();
    let mut c = Computer::new(prog);
    let prog = vec![
        // For hole 3 away, jump if there's land 4 away, otherwise wait
        "NOT C T", "AND D T", "OR T J",
        // For hole 2 away, jump if there's land 4 away, otherwise wait
        "NOT B T", "AND D T", "OR T J", // Jump if about to fall into hole
        "NOT A T", "OR T J", "WALK",
    ];
    c.input_ascii(&prog);
    while let ProgYield::Output(l) = c.run_and_return_output() {
        if l <= 128 {
            print!("{}", char::from_u32(l as u32).unwrap());
        } else {
            return l.to_string()
        }
    }
    "No answer".to_owned()
}

pub fn part2(s: String) -> String {
    let prog = gimme_nums(&s).into_iter().flatten().collect_vec();
    let mut c = Computer::new(prog);
    let prog = vec![
        // For hole 3 away, jump if there's land 4 away, AND
        // that won't put us in danger by having a forced jump into
        // a hole that's 4 away from the landing zone. Otherwise wait
        // J |= (~3) & 4
        "NOT C T", "AND D T", "OR T J",
        // For hole 2 away, jump if there's land 4 away, otherwise wait
        "NOT B T", "AND D T", "OR T J",
        // J &= 5 | 8
        "AND C T", "OR E T", "OR H T", "AND T J",
        // Jump if about to fall into hole
        "NOT A T", "OR T J",
        // Go
        "RUN",
    ];
    c.input_ascii(&prog);
    while let ProgYield::Output(l) = c.run_and_return_output() {
        if l <= 128 {
            print!("{}", char::from_u32(l as u32).unwrap());
        } else {
            return l.to_string()
        }
    }
    "No answer".to_owned()
}
