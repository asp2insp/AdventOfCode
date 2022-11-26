//! ```cargo
//! [dependencies]
//! itertools = "0.7"
//! ```


extern crate itertools;

use std::iter;
use itertools::*;

const INPUT: [u8; 17] = [0,1,0,0,0,1,0,0,0,1,0,0,1,0,1,1,1];


fn do_the_thing(length: usize) {
    let mut v = vec![0; length];
    let mut l = INPUT.len();
    v[0..l].copy_from_slice(&INPUT);
    while l < length {
        let src_len = (length-l-1).min(l);
        let mut src = v[l-src_len..l].to_owned();
        for i in 0..src.len() {
            src[i] = if src[i] == 0 { 1 } else { 0 };
        }
        let dst = &mut v[l+1..l+1+src_len];
        dst.copy_from_slice(&src);
        dst.reverse();
        l = l*2+1;
    }
    // println!("{:?}", v.iter().map(|i| format!("{}", i)).join(""));
    while v.len() % 2 == 0 {
        v = v.chunks(2).map(|p| if p[0] == p[1] { 1 } else { 0 }).collect();
    }
    println!("{:?}", v.iter().map(|i| format!("{}", i)).join(""));
}

fn part_one() {
    do_the_thing(272);
}

fn part_two() {
    do_the_thing(35651584);
}

fn main() {
    part_one();
    part_two();
}
