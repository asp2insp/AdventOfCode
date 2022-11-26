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
use std::cell::Cell;
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
        // out: &Sender<isize>,
        // inp: &Receiver<isize>,
        muls: &Cell<usize>) -> isize {
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
        "sub" => {
            let val = imm_or_reg(instr[2], &reg);
            *reg.entry(instr[1]).or_insert(0) -= val;
            1
        },
        "mul" => {
            let val = imm_or_reg(instr[2], &reg);
            *reg.entry(instr[1]).or_insert(0) *= val;
            muls.set(muls.get()+1);
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
        "jnz" => {
            let val = imm_or_reg(instr[1], &reg);
            let n = imm_or_reg(instr[2], &reg);
            if val != 0 {n} else {1}
        },
        "rcv" => {
            // reg.insert(instr[1], inp.recv().unwrap());
            1
        },
        "snd" => {
            let val = imm_or_reg(instr[1], &reg);
            // out.send(val).unwrap();
            1
        }
        _ => unreachable!("{}", instr[0]),
    }
}

fn part_one() -> usize {
    let mut reg = HashMap::new();
    let mut i = 0isize;
    let program = INPUT.lines()
        .map(|l| l.trim().split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let count = Cell::new(0);
    while (i as usize) < program.len() {
        let instr = &program[i as usize];
        i += execute(instr, &mut reg, &count);
    }
    count.get()
}

fn part_two() -> isize {
    ass()
}

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}

fn ass() -> isize {
    let mut b = 99isize * 100 + 100000;
    let mut h = 0isize;
    let mut c = b + 17000;
    loop {
        for i in 2..b/2 {
            if b % i == 0 {
                h += 1;
                break
            }
        }
        if b == c {
            break
        }
        b += 17;
    }
    h
}

const INPUT: &str = "set b 99
set c b
jnz a 2
  jnz 1 5
    mul b 100
    sub b -100000
    set c b
    sub c -17000
set f 1
set d 2
  set e 2
    set g d
    mul g e
    sub g b
    jnz g 2
    set f 0
    sub e -1
    set g e
    sub g b
  jnz g -8
  sub d -1
  set g d
  sub g b
jnz g -13
jnz f 2
  sub h -1
set g b
sub g c
jnz g 2
  jnz 1 3
sub b -17
jnz 1 -23";
