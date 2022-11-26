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
use std::mem;
use std::fmt;
use chomp::prelude::{U8Input, SimpleResult, parse_only, many1, any, string, token, take_while};
use chomp::ascii::{decimal, skip_whitespace, is_whitespace, is_alphanumeric, is_alpha, signed};
use chomp::types::Buffer;
use std::fs::File;
use std::io::prelude::*;

struct Gen {
    last: u64,
    factor: u64,
}

impl Gen {
    fn new(init: u64, factor: u64) -> Gen {
        Gen {
            last: init,
            factor: factor,
        }
    }
}

impl Iterator for Gen {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        self.last = (self.last * self.factor) % 2147483647;
        Some(self.last)
    }
}

const MASK: u64 = 0xffff;

fn part_one() -> usize {
    let mut a = Gen::new(679, 16807);
    let mut b = Gen::new(771, 48271);

    a.zip(b)
        .take(40*1_000_000)
        .filter(|&(l,r)| (l & MASK) ^ (r & MASK) == 0)
        .count()
}

fn part_two() -> usize {
    let mut a = Gen::new(679, 16807);
    let mut b = Gen::new(771, 48271);

    a.filter(|l| l % 4 == 0).zip(b.filter(|r| r % 8 == 0))
        .take(5*1_000_000)
        .filter(|&(l,r)| (l & MASK) ^ (r & MASK) == 0)
        .count()
}

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}
