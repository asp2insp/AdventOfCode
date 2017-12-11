//! ```cargo
//! [dependencies]
//! itertools = "0.7"
//! rust-crypto = "0.2"
//! ```
#![allow(unused)]

extern crate itertools;
extern crate crypto;

use std::collections::VecDeque;

use crypto::md5::*;
use crypto::digest::*;

use std::iter;
use itertools::*;


fn incr_non_zero(i: usize, a: &[u8]) -> usize {
    let mut j = (i+1) % a.len();
    while a[j] == 0 {
        j = (j + 1) % a.len()
    }
    j
}

fn part_one() {
    let length = INPUT;
    let mut elves = vec![1u8; length];
    let mut counter = 0;
    loop {
        let next = incr_non_zero(counter, &elves);
        if next == counter {
            break
        }
        elves[next] = 0;
        counter = incr_non_zero(next, &elves);
    }
    println!("{}", counter + 1);
}

fn part_two(length: usize) {
    let mut elves = (1u32..(length as u32)+1).collect::<Vec<_>>();
    let mut counter = 0;
    while elves.len() > 1 {
        let next = (counter + elves.len()/2) % elves.len();
        if next < counter {
            counter -= 1;
        }
        elves.remove(next);
        counter = (counter + 1) % elves.len();
        // if elves.len() % 10_000 == 0 {
        //     println!("{}", (INPUT - elves.len()) as f32 * 100.0f32 / INPUT as f32);
        // }
    }
    println!("{} = {}", length, elves[0]);
    // it's log base 3
}

fn main() {
    // part_one();
    for i in 1..100 {
        part_two(i);
    }
}

// const INPUT: &'static str = "";
const INPUT: usize = 3012210;
