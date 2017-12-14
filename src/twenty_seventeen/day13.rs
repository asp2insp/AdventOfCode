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

fn s<I: U8Input>(i: I) -> SimpleResult<I, String> {
    parse!{i;
        let s = take_while(is_alpha);
        ret String::from_utf8(s.to_vec()).unwrap()
    }
}

fn get_input() -> String {
    let mut s = String::new();
    File::open("input13.txt").expect("!!").read_to_string(&mut s);
    s
}

pub fn part_one(init: usize) -> usize {
    let nums = get_input().lines()
        .map(|s| s.split_terminator(": ").collect::<Vec<_>>())
        .map(|v| v.iter()
            .map(|n| n.parse::<usize>().unwrap()).collect::<Vec<_>>()
        )
        .map(|v| (v[0], v[1]))
        .collect::<Vec<_>>();
    let mut levels = vec![];
    let mut j = 0;
    for i in 0..nums.iter().last().unwrap().0+2 {
        if j < nums.len() && nums[j].0 == i {
            levels.push(nums[j].1);
            j += 1;
        } else {
            levels.push(0);
        }
    }
    let mut severity = 0;
    for (i, range) in levels.iter().enumerate() {
        if *range != 0 && (i+init) % (range * 2 - 2) == 0 {
            severity += 1;
        }
    }
    severity
}


pub fn part_two() -> usize {
    (0..).find(|d| part_one(*d) == 0).unwrap()
}

fn main() {
    println!("{}", part_one(0));
    println!("{}", part_two());
}
