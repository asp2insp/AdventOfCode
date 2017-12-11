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
use std::fs::File;
use std::io::prelude::*;

fn get_input() -> String {
    let mut s = String::new();
    File::open("input.txt").expect("!!").read_to_string(&mut s);
    s
}

pub fn part_one() -> String {
    let mut score = 0;
    let mut depth = 1;
    let mut in_garbage = false;
    let mut cancel = false;
    for c in get_input().chars() {
        if cancel {
            cancel = false;
            continue
        }
        match c {
            '!' => cancel = true,
            '<' => in_garbage = true,
            '>' => in_garbage = false,
            '{' => if !in_garbage {
                score += depth;
                depth += 1;
            },
            '}' => if !in_garbage {
                depth -= 1;
            },
            ',' => {
                // ???
            },
            _ => {},
        }
    }
    format!("{}", score)
}


pub fn part_two() -> String {
    let mut score = 0;
    let mut depth = 1;
    let mut in_garbage = false;
    let mut cancel = false;
    for c in get_input().chars() {
        if cancel {
            cancel = false;
            continue
        }
        match c {
            '!' => cancel = true,
            '<' => {
                if in_garbage {
                    score += 1;
                }
                in_garbage = true;
            },
            '>' => in_garbage = false,
            '{' => if !in_garbage {
                depth += 1;
            } else {
                score += 1;
            },
            '}' => if !in_garbage {
                depth -= 1;
            } else {
                score += 1;
            },
            ',' => {
                if in_garbage {
                    score += 1;
                }
            },
            _ => {
                if in_garbage {
                    score += 1;
                }
            },
        }
    }
    format!("{}", score)
}

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}
