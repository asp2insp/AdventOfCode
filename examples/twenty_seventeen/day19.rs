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

fn get_input() -> Vec<Vec<char>> {
    let mut s = String::new();
    File::open("input19.txt").expect("!!").read_to_string(&mut s);
    s.lines()
        .map(|l| l.chars().chain(iter::once(' ')).collect())
        .chain(iter::once(vec![' '; 200]))
        .collect()
}


fn part_one() -> String {
    let v = get_input();
    let (mut x, mut y) = (
        v[0].iter().enumerate().find(|&(_, c)| *c == '|').unwrap().0,
        0
    );
    let mut path = String::new();
    let mut current = '|';
    let mut velocity = 1isize;
    let mut count = 0;
    loop {
        count += 1;
        assert!(y < v.len() && x < v[y].len(), "Out Of Bounds {},{}", y, x);
        match (current, v[y][x]) {
            ('|', '|') | ('|', '-') => {
                y = (y as isize + velocity) as usize;
            },
            ('-', '-') | ('-', '|') => {
                x = (x as isize + velocity) as usize;
            },
            ('|', '+') => {
                if v[y][x-1] != ' ' {
                    velocity = -1;
                } else if v[y][x+1] != ' ' {
                    velocity = 1;
                } else {
                    panic!("Corner? {},{}: {}", x, y, v[y][x]);
                }
                x = (x as isize + velocity) as usize;
                current = '-';
            },
            ('-', '+') => {
                if v[y-1][x] != ' ' {
                    velocity = -1;
                } else if v[y+1][x] != ' ' {
                    velocity = 1;
                } else {
                    panic!("Corner? {},{}: {}", x, y, v[y][x]);
                }
                y = (y as isize + velocity) as usize;
                current = '|';
            },
            (_, 'N') => {
                path += "N";
                break
            },
            ('|', c) if c != ' ' => {
                y = (y as isize + velocity) as usize;
                path += format!("{}", c).as_str();
            },
            ('-', c) if c != ' ' => {
                x = (x as isize + velocity) as usize;
                path += format!("{}", c).as_str();
            },
            _ => unreachable!(),
        }
    }
    println!("{}", count);
    path
}

fn main() {
    println!("{}", part_one());
}

const INPUT: &str = "";
