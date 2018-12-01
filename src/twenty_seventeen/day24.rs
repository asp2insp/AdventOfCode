//! ```cargo
//! [dependencies]
//! itertools = "0.7"
//! rust-crypto = "0.2"
//! chomp = "0.3.1"
//! regex = "0.2.0"
//! rayon = "0.9.0"
//! permutohedron = "0.2"
//! ```
#![allow(unused)]


extern crate regex;
extern crate itertools;
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

fn get_parts() -> Vec<(usize,usize)> {
    INPUT.lines()
        .map(|l| l.split_terminator('/').flat_map(|n| n.parse::<usize>()).collect::<Vec<_>>())
        .map(|v| (v[0],v[1]))
        .collect()
}

fn strength(bridge: &[(usize,usize)]) -> usize {
    bridge.iter().map(|&(l,r)| l+r).sum()
}

fn find_strongest(need: usize, parts: Vec<(usize,usize)>, so_far: Vec<(usize,usize)>) -> Vec<(usize,usize)> {
    parts.par_iter()
        .enumerate()
        .filter(|&(_, &(l, r))| l == need || r == need)
        .map(|(i, &(l, r))| {
            let next_need = if l == need { r } else { l };
            let mut next_parts = parts.clone();
            let mut next_so_far = so_far.clone();
            next_so_far.push((l,r));
            next_parts.remove(i);
            find_strongest(next_need, next_parts, next_so_far)
        })
        .max_by(|a, b| strength(&a).cmp(&strength(&b)))
        .unwrap_or(so_far)
}

fn find_longest(need: usize, parts: Vec<(usize,usize)>, so_far: Vec<(usize,usize)>) -> Vec<(usize,usize)> {
    parts.par_iter()
        .enumerate()
        .filter(|&(_, &(l, r))| l == need || r == need)
        .map(|(i, &(l, r))| {
            let next_need = if l == need { r } else { l };
            let mut next_parts = parts.clone();
            let mut next_so_far = so_far.clone();
            next_so_far.push((l,r));
            next_parts.remove(i);
            find_longest(next_need, next_parts, next_so_far)
        })
        .max_by(|a, b| a.len().cmp(&b.len()).then(strength(&a).cmp(&strength(&b))))
        .unwrap_or(so_far)
}

fn part_one() -> usize {
    let parts = get_parts();
    let bridge = find_strongest(0, parts.clone(), Vec::new());
    // println!("{:?}", bridge);
    strength(&bridge)
}

fn part_two() -> usize {
    let parts = get_parts();
    let bridge = find_longest(0, parts.clone(), Vec::new());
    // println!("{:?}", bridge);
    strength(&bridge)
}

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}

const TEST: &str =
"0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10";

const INPUT: &str =
"48/5
25/10
35/49
34/41
35/35
47/35
34/46
47/23
28/8
27/21
40/11
22/50
48/42
38/17
50/33
13/13
22/33
17/29
50/0
20/47
28/0
42/4
46/22
19/35
17/22
33/37
47/7
35/20
8/36
24/34
6/7
7/43
45/37
21/31
37/26
16/5
11/14
7/23
2/23
3/25
20/20
18/20
19/34
25/46
41/24
0/33
3/7
49/38
47/22
44/15
24/21
10/35
6/21
14/50";
