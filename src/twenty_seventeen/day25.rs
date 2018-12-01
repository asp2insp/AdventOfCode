//! ```cargo
//! [dependencies]
//! itertools = "0.7"
//! rust-crypto = "0.2"
//! chomp = "0.3.1"
//! regex = "0.2.0"
//! rayon = "0.9.0"
//! permutohedron = "0.2"
//! lazy_static = "1.0.0"
//! ```
#![allow(unused)]


extern crate regex;
extern crate itertools;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate chomp;
extern crate rayon;
extern crate permutohedron;

use permutohedron::heap_recursive;
use rayon::prelude::*;
use regex::Regex;
use std::{iter,str};
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

lazy_static! {
    static ref LINES: Vec<&'static str> = { INPUT.lines().collect() };
}

fn execute(state: u8, value: u8) -> (u8, isize, u8) {
    let mut instrs = LINES.iter()
        .skip(10 * (state - b'A') as usize)
        .skip(if value == 0 {2} else {6});
    (
        instrs.next().map(|s| s.as_bytes()[s.len()-2]).map(|b| b - b'0').unwrap(),
        instrs.next().map(|dir| if dir.as_bytes()[dir.len()-6] == b'r' {1} else {-1}).unwrap(),
        instrs.next().map(|s| s.as_bytes()[s.len()-2]).unwrap(),
    )
}

fn part_one() -> usize {
    let mut state = b'A';
    let mut tape = VecDeque::with_capacity(5000);
    let mut i = 0;
    tape.push_front(0u8);
    for _ in 0..12_368_930 {
        let (write, dir, s2) = execute(state, tape[i]);
        tape[i] = write;
        if i == 0 && dir == -1 {
            tape.push_front(0);
        } else if i == tape.len()-1 && dir == 1 {
            tape.push_back(0);
            i += 1;
        } else {
            i = (i as isize + dir) as usize;
        }
        state = s2;
    }
    tape.iter().filter(|i| **i == 1).count()
}

fn part_two() -> usize {
    let mut state = b'A';
    let mut tape = VecDeque::with_capacity(5000);
    let mut i = 0;
    tape.push_front(0u8);
    let mut instrs = HashMap::new();
    instrs.insert((b'A', 0), (1, 1, b'B'));
    instrs.insert((b'A', 1), (0, 1, b'C'));
    instrs.insert((b'B', 0), (0, -1, b'A'));
    instrs.insert((b'B', 1), (0, 1, b'D'));
    instrs.insert((b'C', 0), (1, 1, b'D'));
    instrs.insert((b'C', 1), (1, 1, b'D'));
    instrs.insert((b'D', 0), (1, -1, b'E'));
    instrs.insert((b'D', 1), (0, -1, b'D'));
    instrs.insert((b'E', 0), (1, 1, b'F'));
    instrs.insert((b'E', 1), (1, -1, b'B'));
    instrs.insert((b'F', 0), (1, 1, b'A'));
    instrs.insert((b'F', 1), (1, 1, b'E'));
    for _ in 0..12_368_930 {
        let &(write, dir, s2) = instrs.get(&(state, tape[i])).unwrap();
        tape[i] = write;
        if i == 0 && dir == -1 {
            tape.push_front(0);
        } else if i == tape.len()-1 && dir == 1 {
            tape.push_back(0);
            i += 1;
        } else {
            i = (i as isize + dir) as usize;
        }
        state = s2;
    }
    tape.iter().filter(|i| **i == 1).count()
}

fn main() {
    println!("{}", part_one());
    // println!("{}", part_two());
}

const INPUT: &str = "In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the right.
    - Continue with state C.

In state B:
  If the current value is 0:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the right.
    - Continue with state D.

In state C:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state D.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.

In state D:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state E.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state D.

In state E:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state F.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state B.

In state F:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state E.";
