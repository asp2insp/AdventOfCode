//! ```cargo
//! [dependencies]
//! itertools = "0.7"
//! rust-crypto = "0.2"
//! ```
#![allow(unused)]

extern crate itertools;
extern crate crypto;

use std::collections::VecDeque;
use std::collections::HashSet;

use crypto::md5::*;
use crypto::digest::*;

use std::iter;
use itertools::*;

fn part_one(length: usize) {
    let mut configs = HashSet::new();
    let mut blocks = INPUT.split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let mut counter = 0;
    loop {
        if configs.contains(&blocks) {
            break
        }
        configs.insert(blocks.clone());
        println!("{:?}", blocks);
        let mut i = blocks.iter().enumerate().rev().max_by_key(|&(_, b)| b).unwrap().0;
        let mut b = blocks[i];
        blocks[i] = 0;
        while b > 0 {
            i = (i + 1) % blocks.len();
            blocks[i] += 1;
            b -= 1;
        }
    }

    println!("{}", configs.len());
}

fn part_two(length: usize) {
    let mut configs = HashSet::new();
    let mut blocks = INPUT.split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let mut counter = 0;
    let mut seq = vec![];
    loop {
        if seq.is_empty() {
            if configs.contains(&blocks) {
                seq = blocks.clone();
                println!("{:?}", seq);
            }
            configs.insert(blocks.clone());
        } else {
            counter += 1;
            if blocks.iter().zip(seq.iter()).all(|(a,b)| a == b) {
                break
            }
        }
        let mut i = blocks.iter().enumerate().rev().max_by_key(|&(_, b)| b).unwrap().0;
        let mut b = blocks[i];
        blocks[i] = 0;
        while b > 0 {
            i = (i + 1) % blocks.len();
            blocks[i] += 1;
            b -= 1;
        }
    }

    println!("{}", counter);
}

fn main() {
    part_one(0);
    part_two(0);
}

// const INPUT: &'static str = "0 2 7 0";
const INPUT: &'static str = "2	8	8	5	4	2	3	1	5	5	1	2	15	13	5	14";
// const INPUT: usize = 3012210;
