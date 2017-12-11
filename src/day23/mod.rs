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
use std::collections::VecDeque;
use std::mem;
use std::fmt;
use chomp::prelude::{U8Input, SimpleResult, parse_only, many1, any, string, token, take_while};
use chomp::ascii::{decimal, skip_whitespace, is_whitespace, is_alphanumeric, is_alpha, signed};
use std::collections::HashMap;
use chomp::types::Buffer;
use self::Instr::*;

#[derive(Clone)]
enum Instr {
    Cpy(isize, u8),
    Inc(u8),
    Dec(u8),
    Jnz(isize, isize),
    Tgl(u8)
}

impl fmt::Debug for Instr {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &Inc(r) => write!(f, "Inc({})", char::from(r)),
            &Dec(r) => write!(f, "Dec({})", char::from(r)),
            &Cpy(a, r) => if is_alpha(a as u8) {
                write!(f, "Cpy({}, {})", char::from(a as u8), char::from(r))
            } else {
                write!(f, "Cpy({}, {})", a, char::from(r))
            },
            &Jnz(a, b) => if is_alpha(a as u8) {
                write!(f, "Jnz({}, {})", char::from(a as u8), b)
            } else {
                write!(f, "Jnz({}, {})", a, b)
            },
            &Tgl(r) => write!(f, "Tgl({})", char::from(r)),
        }
    }
}

impl Instr {
    fn apply(&self, regs: &mut HashMap<u8, isize>, i: isize, program: &mut Vec<Instr>) -> isize {
        match self {
            &Inc(r) => {
                *regs.entry(r).or_insert(0) += 1;
                1
            },
            &Dec(r) => {
                *regs.entry(r).or_insert(0) -= 1;
                1
            },
            &Cpy(x, y) => {
                if is_alpha(x as u8) {
                    let v = *regs.get(&(x as u8)).unwrap();
                    regs.insert(y, v);
                } else {
                    regs.insert(y, x);
                }
                1
            },
            &Jnz(r, i) => {
                if is_alpha(r as u8) {
                    if *regs.get(&(r as u8)).unwrap() != 0 { if is_alpha(i as u8) {*regs.get(&(i as u8)).unwrap()} else {i} } else { 1 }
                } else {
                    if r != 0 { if is_alpha(i as u8) {*regs.get(&(i as u8)).unwrap()} else {i} } else { 1 }
                }
            },
            &Tgl(r) => {
                let j = (*regs.get(&r).unwrap()+i) as usize;
                if j < program.len() {
                    program[j] = match program[j] {
                        Jnz(b, i) => Cpy(b as isize, i as u8),
                        Cpy(a, b) => Jnz(a, b as isize),
                        Inc(b) => Dec(b),
                        Dec(b) => Inc(b),
                        Tgl(b) => Inc(b),
                    };
                }
                1
            },
        }
    }
}

fn copy<I: U8Input>(i: I) -> SimpleResult<I, Instr> {
    parse!{i;
        string(b"cpy ");
        let x = take_while(|i| is_alphanumeric(i) || i == b'-');
                token(b' ');
        let y = any();
                skip_whitespace();
        ret if is_alpha(x.to_vec()[0]) {
            Cpy(x.to_vec()[0] as isize, y)
        } else {
            Cpy(
                String::from_utf8(x.to_vec())
                    .unwrap()
                    .parse::<isize>()
                    .unwrap(),
                y
            )
        }
    }
}

fn increment<I: U8Input>(i: I) -> SimpleResult<I, Instr> {
    parse!{i;
        string(b"inc ");
        let x = any();
                skip_whitespace();
        ret Inc(x)
    }
}



fn alpha_or_isize<I: U8Input>(i: I) -> SimpleResult<I, isize> {
    parse!{i;
        let y = take_while(|i| is_alphanumeric(i) || i == b'-');
        ret if is_alpha(y.to_vec()[0]) {
            y.to_vec()[0] as isize
        } else {
            String::from_utf8(y.to_vec())
                .unwrap()
                .parse::<isize>()
                .unwrap()
        }
    }
}

fn jump_not_zero<I: U8Input>(i: I) -> SimpleResult<I, Instr> {
    parse!{i;
        string(b"jnz ");
        let x = alpha_or_isize();
                token(b' ');
        let y = alpha_or_isize();
                skip_whitespace();
        ret Jnz(x, y)
    }
}

fn decrement<I: U8Input>(i: I) -> SimpleResult<I, Instr> {
    parse!{i;
        string(b"dec ");
        let x = any();
                skip_whitespace();
        ret Dec(x)
    }
}

fn toggle<I: U8Input>(i: I) -> SimpleResult<I, Instr> {
    parse!{i;
        string(b"tgl ");
        let x = any();
                skip_whitespace();
        ret Tgl(x)
    }
}

fn instr<I: U8Input>(i: I) -> SimpleResult<I, Instr> {
    parse!{i;
        let instruction = increment() <|> decrement() <|> jump_not_zero() <|> copy() <|> toggle();
        ret instruction
    }
}

fn all_instrs<I: U8Input>(i: I) -> SimpleResult<I, Vec<Instr>> {
    parse!{i;
        let v = many1(instr);
        ret v
    }
}

fn run(program: &mut Vec<Instr>, regs: &mut HashMap<u8, isize>) -> isize {
    let mut i = 0isize;
    while i < program.len() as isize {
        let prev = i;
        i += program[i as usize].clone().apply(regs, i, program);
        // println!("{}> {:?} => {:?} > {}", prev+240, program[prev as usize], regs.iter().map(|(r, i)| format!("{}:{}", char::from(*r), i)).sorted().join(" "), i+240);
    }
    *regs.get(&b'a').unwrap()
}

pub fn part_one() -> String {
    let mut regs = HashMap::with_capacity(4);
    regs.insert(b'a', 7);
    regs.insert(b'b', 0);
    regs.insert(b'c', 0);
    regs.insert(b'd', 0);

    let mut instrs = parse_only(all_instrs, INPUT.as_bytes()).unwrap();
    let r = run(&mut instrs, &mut regs);
    println!("Instructions {:?}", instrs);
    format!("{}", r)
}


pub fn part_two() -> String {
    let mut regs = HashMap::with_capacity(4);
    regs.insert(b'a', 12);
    regs.insert(b'b', 0);
    regs.insert(b'c', 0);
    regs.insert(b'd', 0);

    let mut instrs = parse_only(all_instrs, INPUT.as_bytes()).unwrap();
    let r = run(&mut instrs, &mut regs);
    // println!("Instructions {:?}", instrs);
    format!("{}", r)
}

fn main() {
    // println!("{}", part_one());
    println!("{}", part_two());
}

const I2: &'static str = "cpy 2 a
tgl a
tgl a
tgl a
cpy 1 a
dec a
dec a";

const INPUT: &'static str = "cpy a b
dec b
cpy a d
cpy 0 a
cpy b c
inc a
dec c
jnz c -2
dec d
jnz d -5
dec b
cpy b c
cpy c d
dec d
inc c
jnz d -2
tgl c
cpy -16 c
jnz 1 c
cpy 96 c
jnz 95 d
inc a
inc d
jnz d -2
inc c
jnz c -5";
