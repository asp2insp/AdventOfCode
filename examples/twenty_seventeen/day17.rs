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

const INPUT: usize = 301;

fn part_one() -> usize {
    let mut v = VecDeque::new();
    v.push_back(0);
    let mut last = 0;
    for i in 1..2018 {
        let j = (last + INPUT) % v.len() + 1;
        v.insert(j, i);
        last = j;
    }
    v[(last + 1) % v.len()]
}

fn part_two() -> usize {
    let mut last = 0;
    let mut sec = 0;
    for i in 1..50000000 {
        let j = (last + INPUT) % i + 1;
        last = j;
        if last == 1 {
            sec = i;
            // println!("{} ({}): {}", i, last, sec);
        }
    }
    sec
}

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}
