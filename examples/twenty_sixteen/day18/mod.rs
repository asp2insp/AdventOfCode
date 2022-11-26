//! ```cargo
//! [dependencies]
//! itertools = "0.7"
//! ```
#![allow(unused)]

extern crate itertools;

use std::iter;
use itertools::*;

const INPUT: &'static str = ".^^^.^.^^^^^..^^^..^..^..^^..^.^.^.^^.^^....^.^...^.^^.^^.^^..^^..^.^..^^^.^^...^...^^....^^.^^^^^^^";
// const INPUT: &'static str = ".^^.^.^^^^";


fn do_the_thing(length: usize) {
    let last = INPUT.len()-1;
    let mut b = INPUT.chars().collect::<Vec<_>>();
    let mut sum = b.iter().filter(|b| **b == '.').count();
    // println!("{}", b.iter().join(""));
    for _ in 0..length-1 {
        let b_first = if b[1] == '^' { '^' } else { '.' };
        let b_last = if b[last-1] == '^' { '^' } else { '.' };
        let b2 = (1..last).map(|i| match (b[i-1], b[i], b[i+1]) {
            ('^','^','.') | ('^','.','.') | ('.','^','^') | ('.','.','^') => '^',
            _ => '.',
        }).collect::<Vec<_>>();
        b[1..last].copy_from_slice(&b2);
        b[0] = b_first;
        b[last] = b_last;
        sum += b.iter().filter(|b| **b == '.').count();
        // println!("{}", b.iter().join(""));
    }
    println!("{}", sum);
}

fn part_one() {
    do_the_thing(40);
}

fn part_two() {
    do_the_thing(400000);
}

fn main() {
    part_one();
    part_two();
}
