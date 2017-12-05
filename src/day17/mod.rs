//! ```cargo
//! [dependencies]
//! rust-crypto = "0.2"
//! itertools = "0.7"
//! ```


extern crate itertools;
extern crate crypto;

use std::collections::VecDeque;
use itertools::*;

use crypto::md5::*;
use crypto::digest::*;

const INPUT: &'static str = "njfxhljp";

fn is_open(s: &str, n: usize) -> bool {
    match s.chars().nth(n).unwrap() {
        'b' | 'c' | 'd' | 'e' | 'f' => true,
        _ => false,
    }
}

fn bfs_step(input: &str) -> (bool, Vec<String>) {
    let (x, y) = input.chars().map(|c| match c {
        'D' => (0, 1),
        'U' => (0, -1),
        'L' => (-1, 0),
        'R' => (1, 0),
        _ => panic!("NOT A VALID DIRECTION: {}" , c),
    })
    .fold((0,0), |pos, step| (pos.0 + step.0, pos.1 + step.1));
    let mut ret = vec![];
    let is_done = (x,y) == (3,3);

    if !is_done {
        let mut sh = Md5::new();
        sh.input_str(&format!("{}{}", INPUT, input));

        let hash = sh.result_str();
        if x > 0 && is_open(&hash, 2) {
            ret.push(input.to_owned() + "L");
        }
        if x < 3 && is_open(&hash, 3) {
            ret.push(input.to_owned() + "R");
        }
        if y > 0 && is_open(&hash, 0) {
            ret.push(input.to_owned() + "U");
        }
        if y < 3 && is_open(&hash, 1) {
            ret.push(input.to_owned() + "D");
        }
    }
    (is_done, ret)
}

fn part_one() {
    let mut q = VecDeque::new();
    q.push_back("".to_owned());
    loop {
        let path = q.pop_front().unwrap();
        let (done, next) = bfs_step(&path);
        if done {
            println!("{}", path);
            return
        }
        next.into_iter().foreach(|p| q.push_back(p));
    }
}

fn part_two() {
    let mut q = VecDeque::new();
    q.push_back("".to_owned());
    let mut record = 0;
    loop {
        let path = match q.pop_front() {
            Some(p) => p,
            None => break,
        };
        let (done, next) = bfs_step(&path);
        if done {
            record = record.max(path.len());
        }
        next.into_iter().foreach(|p| q.push_back(p));
    }
    println!("{}", record);
}

fn main() {
    part_one();
    part_two();
}
