use chrono::{NaiveDateTime, Timelike};
use itertools::*;
use rayon::prelude::*;
use regex::*;
use std::collections::HashMap;
use std::mem;

fn is_safe(a: char, b: char) -> bool {
    a == b || a.to_ascii_lowercase() != b.to_ascii_lowercase()
}

fn len_after_reacting(mut chain: Vec<char>) -> usize {
    let mut stack = Vec::with_capacity(chain.len());
    for c in chain {
        if stack.is_empty() {
            stack.push(c);
        } else {
            let t = stack.pop().unwrap();
            if is_safe(t, c) {
                stack.push(t);
                stack.push(c);
            }
        }
    }
    stack.len()
}

pub fn part1(input: String) -> String {
    let chain = input.chars().collect::<Vec<_>>();
    let len = len_after_reacting(chain);
    format!("{}", len)
}

pub fn part2(input: String) -> String {
    let mut chain = input.chars().collect::<Vec<_>>();
    let abc = (65u8..90).map(|ci| ci as char).collect::<Vec<char>>();
    let m = abc
        .into_par_iter()
        .map(|c| {
            let chain2 = chain
                .iter()
                .filter(|i| **i != c && **i != c.to_ascii_lowercase())
                .cloned()
                .collect();
            len_after_reacting(chain2)
        })
        .min()
        .unwrap();
    format!("{}", m)
}
