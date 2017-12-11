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
    Tgl(u8),
    Out(u8)
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
            &Out(r) => write!(f, "Out({})", char::from(r)),
        }
    }
}

impl Instr {
    fn apply(&self, regs: &mut HashMap<u8, isize>, i: isize, program: &mut Vec<Instr>, output: &mut VecDeque<isize>) -> isize {
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
                        Out(b) => Inc(b),
                    };
                }
                1
            },
            &Out(r) => {
                let j = *regs.get(&r).unwrap();
                output.push_back(j);
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

fn out<I: U8Input>(i: I) -> SimpleResult<I, Instr> {
    parse!{i;
        string(b"out ");
        let x = any();
                skip_whitespace();
        ret Out(x)
    }
}

fn instr<I: U8Input>(i: I) -> SimpleResult<I, Instr> {
    parse!{i;
        let instruction = increment() <|> decrement() <|> jump_not_zero() <|> copy() <|> toggle() <|> out();
        ret instruction
    }
}

fn all_instrs<I: U8Input>(i: I) -> SimpleResult<I, Vec<Instr>> {
    parse!{i;
        let v = many1(instr);
        ret v
    }
}

fn run(program: &mut Vec<Instr>, regs: &mut HashMap<u8, isize>) -> bool {
    let mut i = 0isize;
    let mut output = VecDeque::new();
    while i < program.len() as isize && output.len() < 10 {
        let prev = i;
        i += program[i as usize].clone().apply(regs, i, program, &mut output);
    }
    // println!("{:?}", output);
    Vec::from(output).chunks(2).all(|v| v[0] == 0 && v[1] == 1)
}

pub fn part_one() -> String {
    let mut instrs = parse_only(all_instrs, INPUT.as_bytes()).unwrap();
    // println!("{:?}", instrs);
    for i in 0.. {
        let mut regs = HashMap::with_capacity(4);
        regs.insert(b'a', i);
        let r = run(&mut instrs, &mut regs);
        if r {
            println!("{}: {}", i, r);
            break
        }
    }
    "".to_owned()
}


pub fn part_two() -> String {
    "".to_owned()
}

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}

const INPUT: &'static str = "cpy a d
cpy 11 c
cpy 231 b
inc d
dec b
jnz b -2
dec c
jnz c -5
cpy d a
jnz 0 0
cpy a b
cpy 0 a
cpy 2 c
jnz b 2
jnz 1 6
dec b
dec c
jnz c -4
inc a
jnz 1 -7
cpy 2 b
jnz c 2
jnz 1 4
dec b
dec c
jnz 1 -4
jnz 0 0
out b
jnz a -19
jnz 1 -21";
