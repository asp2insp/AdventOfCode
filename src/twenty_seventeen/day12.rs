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
use std::collections::{VecDeque,HashSet};
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
    File::open("input12.txt").expect("!!").read_to_string(&mut s);
    s
}

pub fn part_one() -> String {
    let mut connections = get_input().lines()
        .flat_map(|l| {
            let mut parts = l.split_terminator(" <-> ");
            let n = parts.next().unwrap().parse::<usize>().unwrap();
            let rest = parts.next().unwrap();
            let edges = rest.split_terminator(", ")
                .flat_map(|s| s.parse::<usize>())
                .collect::<Vec<_>>();
            iter::repeat(n).zip(edges)
        })
        .collect::<Vec<_>>();
    let mut temp = connections.iter()
        .map(|&(a,b)| (b,a))
        .collect::<Vec<_>>();
    connections.append(&mut temp);
    let mut graph = HashSet::new();
    graph.insert(0);
    loop {
        let old_len = graph.len();
        for edge in &connections {
            if graph.contains(&edge.0) {
                graph.insert(edge.1);
            }
        }
        if old_len == graph.len() {
            break
        }
    }
    format!("{}", graph.len())
}


pub fn part_two() -> String {
    let mut connections = get_input().lines()
        .flat_map(|l| {
            let mut parts = l.split_terminator(" <-> ");
            let n = parts.next().unwrap().parse::<usize>().unwrap();
            let rest = parts.next().unwrap();
            let edges = rest.split_terminator(", ")
                .flat_map(|s| s.parse::<usize>())
                .collect::<Vec<_>>();
            iter::repeat(n).zip(edges)
        })
        .collect::<Vec<_>>();
    let mut temp = connections.iter()
        .map(|&(a,b)| (b,a))
        .collect::<Vec<_>>();
    connections.append(&mut temp);
    let mut all_nodes: HashSet<usize> = connections.iter()
        .map(|&(a, _)| a)
        .collect();
    let mut count = 0;
    loop {
        if all_nodes.is_empty() {
            break
        }
        let mut graph: HashSet<usize> = HashSet::new();
        graph.insert(*all_nodes.iter().next().unwrap());
        loop {
            let old_len = graph.len();
            for edge in &connections {
                if graph.contains(&edge.0) {
                    graph.insert(edge.1);
                }
            }
            if old_len == graph.len() {
                break
            }
        }
        count += 1;
        all_nodes = &all_nodes - &graph;
    }
    format!("{}", count)
}

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}
