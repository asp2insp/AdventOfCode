use itertools::*;
use chrono::{NaiveDateTime,Timelike};
use std::collections::HashMap;
use time::Duration;
use rayon::prelude::*;
use regex::*;
use std::mem;

// 
lazy_static! {
    static ref RE: Regex = Regex::new(r"Step (?P<dep>[A-Z]) must be finished before step (?P<item>[A-Z]) can begin.").unwrap();
}

fn parse_lines(s: &str) -> Vec<(char, char)>{
    s.lines()
        .map(|l|{
            let cap = RE.captures(l).unwrap();
            (
                cap["item"].chars().next().unwrap(),
                cap["dep"].chars().next().unwrap(),
            )
        })
        .collect()
}


pub fn part1(input: String) -> String {
    let mut deps = parse_lines(&input);
    "".to_owned()
}

#[derive(Copy, Clone, Debug)]
struct Computer {
    reg: [usize; 4];
}

impl Computer {
    fn run(&mut self, i: Instruction) {

    }
}

struct Instruction {
    op: Opcode,
    a: usize,
    b: usize,
    c: usize,
}

enum Opcode {
    Addr, Addi, Mulr, Muli, Banr, Bani, Borr, Bori, Setr, Seti,
    Gtri, Gtir, Gtrr, Eqir, Eqri, Eqrr
}
use Opcode::*;

impl <'a> From<&'a str> for Opcode {
    fn from(s: &'a str) -> Opcode {
        match s {
            "addr" => Addr,
            "addi" => Addi,
            "mulr" => m,
            "muli" => Muli,
            "banr" => Banr,
            "bani" => Bani,
            "borr" => Borr,
            "bori" => Bori,
            "setr" => Setr,
            "seti" => Seti,
            "gtri" => Gtri,
            "gtir" => Gtir,
            "gtrr" => Gtrr,
            "eqir" => Eqir,
            "eqri" => Eqri,
            "eqrr" => Eqrr,
            other => panic!("Unknown opcode {}", other),
        }
    }
}

pub fn part2(input: String) -> String {
    "".to_owned()
}