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

const L: usize = 256;

pub fn part_one() -> usize {
    let mut map = String::new();
    for i in 0..128 {
        let mut v = vec![0; L];
        for i in 0..L {
            v[i] = i;
        }
        let lengths: Vec<_> = "hfdlxzhv".chars()
            .chain(format!("-{}", i).chars())
            .map(|c| (c as u8) as usize)
            .chain(vec![17, 31, 73, 47, 23].into_iter())
            .collect();
        let mut skip_size = 0;
        let mut offset = 0;
        for _ in 0..64 {
            for length in lengths.clone() {
                if offset + length < L {
                    v[offset..offset+length].reverse();
                } else {
                    let leftover = (offset + length) - L;
                    let mut subset = vec![0; length];
                    subset[0..L-offset].copy_from_slice(&v[offset..L]);
                    subset[L-offset..length].copy_from_slice(&v[0..leftover]);
                    subset.reverse();
                    v[offset..L].copy_from_slice(&subset[0..L-offset]);
                    v[0..leftover].copy_from_slice(&subset[L-offset..length]);
                }
                offset = (offset + length + skip_size) % L;
                skip_size += 1;
            }
        }
        let hash = v.chunks(16)
            .map(|c| c.iter().fold(0, |a,b| a ^ b))
            .map(|n| format!("{:b}", n))
            .join("");
        map += &hash;
    }
    map.chars().filter(|c| *c == '1').count()
}

pub fn part_two() -> usize {
    let mut map = vec![];
    for i in 0..128 {
        let mut v = vec![0; L];
        for i in 0..L {
            v[i] = i;
        }
        let lengths: Vec<_> = "hfdlxzhv".chars()
            .chain(format!("-{}", i).chars())
            .map(|c| (c as u8) as usize)
            .chain(vec![17, 31, 73, 47, 23].into_iter())
            .collect();
        let mut skip_size = 0;
        let mut offset = 0;
        for _ in 0..64 {
            for length in lengths.clone() {
                if offset + length < L {
                    v[offset..offset+length].reverse();
                } else {
                    let leftover = (offset + length) - L;
                    let mut subset = vec![0; length];
                    subset[0..L-offset].copy_from_slice(&v[offset..L]);
                    subset[L-offset..length].copy_from_slice(&v[0..leftover]);
                    subset.reverse();
                    v[offset..L].copy_from_slice(&subset[0..L-offset]);
                    v[0..leftover].copy_from_slice(&subset[L-offset..length]);
                }
                offset = (offset + length + skip_size) % L;
                skip_size += 1;
            }
        }
        let hash = v.chunks(16)
            .map(|c| c.iter().fold(0, |a,b| a ^ b))
            .map(|n| format!("{:08b}", n))
            .join("");
        map.push(hash.chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<_>>());
    }
    let mut group_id = 1;
    let mut groups = vec![vec![0; 128]; 128];
    for x in 0..128 {
        for y in 0..128 {
            if map[x][y] == 1 {
                let mut has_group = false;
                if y > 0 && groups[x][y-1] != 0 {
                    groups[x][y] = groups[x][y-1];
                    has_group = true;
                }
                if x > 0 && groups[x-1][y] != 0 {
                    if has_group {
                        let o = groups[x][y];
                        let n = groups[x-1][y];
                        for i in 0..128 {
                            for j in 0..128 {
                                if groups[i][j] == o {
                                    groups[i][j] = n;
                                }
                            }
                        }
                    }
                    groups[x][y] = groups[x-1][y];
                    has_group = true;
                }
                if !has_group {
                    groups[x][y] = group_id;
                    group_id += 1;
                }
            }
        }
    }

    // for x in 0..10 {
    //     println!("{}",
    //         groups[x][0..8].iter()
    //             .map(|b| format!("{: ^3}|", b))
    //             .join(""));
    // }

    groups.iter().flat_map(|r| r.iter()).unique().count() - 1
}

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}
