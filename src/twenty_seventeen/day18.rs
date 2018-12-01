//! ```cargo
//! [dependencies]
//! itertools = "0.7"
//! rust-crypto = "0.2"
//! chomp = "0.3.1"
//! regex = "0.2.0"
//! ```
#![allow(unused)]


extern crate regex;
extern crate itertools;
#[macro_use]
extern crate chomp;

use regex::Regex;
use std::iter;
use itertools::*;
use std::collections::{VecDeque,HashSet,HashMap};
use std::{mem,fmt,thread};
use chomp::prelude::{U8Input, SimpleResult, parse_only, many1, any, string, token, take_while};
use chomp::ascii::{decimal, skip_whitespace, is_whitespace, is_alphanumeric, is_alpha, signed};
use chomp::types::Buffer;
use std::fs::File;
use std::io::prelude::*;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::time::Duration;


fn imm_or_reg(n: &'static str, reg: &HashMap<&'static str, isize>) -> isize {
    if n.len() == 1 && is_alpha(n.bytes().next().unwrap()) {
        *reg.get(n).unwrap_or(&0)
    } else {
        n.parse::<isize>().unwrap()
    }
}

fn execute(
        instr: &Vec<&'static str>,
        reg: &mut HashMap<&'static str, isize>,
        out: &Sender<isize>,
        inp: &Receiver<isize>) -> isize {
    match instr[0] {
        "set" => {
            let val = imm_or_reg(instr[2], &reg);
            reg.insert(instr[1], val);
            1
        },
        "add" => {
            let val = imm_or_reg(instr[2], &reg);
            *reg.entry(instr[1]).or_insert(0) += val;
            1
        },
        "mul" => {
            let val = imm_or_reg(instr[2], &reg);
            *reg.entry(instr[1]).or_insert(0) *= val;
            1
        },
        "mod" => {
            let val = imm_or_reg(instr[2], &reg);
            *reg.entry(instr[1]).or_insert(0) %= val;
            1
        },
        "jgz" => {
            let val = imm_or_reg(instr[1], &reg);
            let n = imm_or_reg(instr[2], &reg);
            if val > 0 {n} else {1}
        },
        "rcv" => {
            reg.insert(instr[1], inp.recv().unwrap());
            1
        },
        "snd" => {
            let val = imm_or_reg(instr[1], &reg);
            println!("{}: snd {}", reg.get("id").unwrap(), val);
            out.send(val).unwrap();
            1
        }
        _ => unreachable!("{}", instr[0]),
    }
}

fn run(id: isize, out: Sender<isize>, inp: Receiver<isize>) {
    let mut reg = HashMap::new();
    reg.insert("p", id);
    reg.insert("id", id);
    let mut i = 0isize;
    let program = INPUT.lines()
        .map(|l| l.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    loop {
        let instr = &program[i as usize];
        i += execute(instr, &mut reg, &out, &inp);
    }
}

fn part_one() -> isize {
    0
}

fn part_two() -> isize {
    let (a_out, a_in) = channel();
    let (b_out, b_in) = channel();
    thread::spawn(move || run(0, b_out, a_in));
    thread::spawn(move || run(1, a_out, b_in));
    thread::sleep(Duration::from_secs(5));
    0
}

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}

const INPUT: &str = "set i 31
set a 1
mul p 17
jgz p p
mul a 2
add i -1
jgz i -2
add a -1
set i 127
set p 464
mul p 8505
mod p a
mul p 129749
add p 12345
mod p a
set b p
mod b 10000
snd b
add i -1
jgz i -9
jgz a 3
rcv b
jgz b -1
set f 0
set i 126
rcv a
rcv b
set p a
mul p -1
add p b
jgz p 4
snd a
set a b
jgz 1 3
snd b
set f 1
add i -1
jgz i -11
snd a
jgz f -16
jgz a -19";
