#![allow(unused)]
//! ```cargo
//! [dependencies]
//! itertools = "0.7"
//! ```


extern crate itertools;

use std::iter;
use itertools::*;
use std::collections::VecDeque;
use std::mem;

fn do_the_thing(length: usize) {
    
}

fn part_one() {
    let mut instrs = INPUT.lines().map(|s| s.split_whitespace().collect::<Vec<&'static str>>()).collect::<Vec<_>>();
    let mut pass: VecDeque<u8> = "abcdefgh".as_bytes().to_owned().into();
    for i in instrs.into_iter() {
        match (i[0], i[1]) {
            ("swap", "letter") => {
                let target = i[2].as_bytes()[0];
                let repl = i[5].as_bytes()[0];
                pass = pass.into_iter().map(|b| if b == target { repl } else if b == repl { target } else { b }).collect();
            },
            ("swap", "position") => {
                let x = i[2].parse::<usize>().unwrap();
                let y = i[5].parse::<usize>().unwrap();
                let tmp = pass[x];
                pass[x] = pass[y];
                pass[y] = tmp;
            },
            ("rotate", "left") => {
                let steps = i[2].parse::<usize>().unwrap();
                for _ in 0..steps {
                    let c = pass.pop_front().unwrap();
                    pass.push_back(c);
                }
            },
            ("rotate", "right") => {
                let steps = i[2].parse::<usize>().unwrap();
                for _ in 0..steps {
                    let c = pass.pop_back().unwrap();
                    pass.push_front(c);
                }
            },
            ("reverse", "positions") => {
                let x = i[2].parse::<usize>().unwrap();
                let y = i[4].parse::<usize>().unwrap();
                let mut tmp = Vec::from(pass);
                tmp[x..y+1].reverse();
                pass = tmp.into();
            },
            ("move", "position") => {
                let x = i[2].parse::<usize>().unwrap();
                let y = i[5].parse::<usize>().unwrap();
                let c = pass.remove(x).unwrap();
                pass.insert(y, c);
            },
            ("rotate", "based") => {
                let c = i[6].as_bytes()[0];
                let i = pass.iter().enumerate().find(|&(_, s)| *s == c).unwrap().0;
                let num_rots = i+1+ if i >= 4 { 1 } else { 0 };
                for _ in 0..num_rots {
                    let c = pass.pop_back().unwrap();
                    pass.push_front(c);
                }
                // println!("{}, {}", i, num_rots);
            },
            _ => unreachable!(),
        };
    }
    println!("{}", String::from_utf8(pass.into()).unwrap());
}

fn part_two() {
    let mut instrs = INPUT.lines().rev().map(|s| s.split_whitespace().collect::<Vec<&'static str>>()).collect::<Vec<_>>();
    let mut pass: VecDeque<u8> = "fbgdceah".as_bytes().to_owned().into();
    for i in instrs.into_iter() {
        match (i[0], i[1]) {
            ("swap", "letter") => {
                let target = i[2].as_bytes()[0];
                let repl = i[5].as_bytes()[0];
                pass = pass.into_iter().map(|b| if b == target { repl } else if b == repl { target } else { b }).collect();
            },
            ("swap", "position") => {
                let x = i[2].parse::<usize>().unwrap();
                let y = i[5].parse::<usize>().unwrap();
                let tmp = pass[x];
                pass[x] = pass[y];
                pass[y] = tmp;
            },
            ("rotate", "left") => {
                let steps = i[2].parse::<usize>().unwrap();
                for _ in 0..steps {
                    let c = pass.pop_back().unwrap();
                    pass.push_front(c);
                }
            },
            ("rotate", "right") => {
                let steps = i[2].parse::<usize>().unwrap();
                for _ in 0..steps {
                    let c = pass.pop_front().unwrap();
                    pass.push_back(c);
                }
            },
            ("reverse", "positions") => {
                let x = i[2].parse::<usize>().unwrap();
                let y = i[4].parse::<usize>().unwrap();
                let mut tmp = Vec::from(pass);
                tmp[x..y+1].reverse();
                pass = tmp.into();
            },
            ("move", "position") => {
                let x = i[2].parse::<usize>().unwrap();
                let y = i[5].parse::<usize>().unwrap();
                let c = pass.remove(y).unwrap();
                pass.insert(x, c);
            },
            ("rotate", "based") => {
                let c = i[6].as_bytes()[0];
                let i = pass.iter().enumerate().find(|&(_, s)| *s == c).unwrap().0;
                let mut num_rots = 0;
                for j in 0..pass.len()+2 {
                    let n = j+1+ if j >= 4 { 1 } else { 0 };
                    if (j + n + pass.len()) % pass.len() == i {
                        num_rots = n;
                        break;
                    }
                };
                for _ in 0..num_rots {
                    let c = pass.pop_front().unwrap();
                    pass.push_back(c);
                }
                // println!("{}, {}", i, num_rots);
            },
            _ => unreachable!(),
        };
    }
    println!("{}", String::from_utf8(pass.into()).unwrap());
}

fn main() {
    // part_one();
    part_two();
}

const I2: &'static str = "swap position 4 with position 0
swap letter d with letter b
reverse positions 0 through 4
rotate left 1 step
move position 1 to position 4
move position 3 to position 0
rotate based on position of letter b
rotate based on position of letter d";

const INPUT: &'static str = "rotate based on position of letter a
swap letter g with letter d
move position 1 to position 5
reverse positions 6 through 7
move position 5 to position 4
rotate based on position of letter b
reverse positions 6 through 7
swap letter h with letter f
swap letter e with letter c
reverse positions 0 through 7
swap position 6 with position 4
rotate based on position of letter e
move position 2 to position 7
swap position 6 with position 4
rotate based on position of letter e
reverse positions 2 through 3
rotate right 2 steps
swap position 7 with position 1
move position 1 to position 2
move position 4 to position 7
move position 5 to position 0
swap letter e with letter f
move position 4 to position 7
reverse positions 1 through 7
rotate based on position of letter g
move position 7 to position 4
rotate right 6 steps
rotate based on position of letter g
reverse positions 0 through 5
reverse positions 0 through 7
swap letter c with letter f
swap letter h with letter f
rotate right 7 steps
rotate based on position of letter g
rotate based on position of letter c
swap position 1 with position 4
move position 7 to position 3
reverse positions 2 through 6
move position 7 to position 0
move position 7 to position 1
move position 6 to position 7
rotate right 5 steps
reverse positions 0 through 6
move position 1 to position 4
rotate left 3 steps
swap letter d with letter c
move position 4 to position 5
rotate based on position of letter f
rotate right 1 step
move position 7 to position 6
swap position 6 with position 0
move position 6 to position 2
rotate right 1 step
swap position 1 with position 6
move position 2 to position 6
swap position 2 with position 1
reverse positions 1 through 7
move position 4 to position 1
move position 7 to position 0
swap position 6 with position 7
rotate left 1 step
reverse positions 0 through 4
rotate based on position of letter c
rotate based on position of letter b
move position 2 to position 1
rotate right 0 steps
swap letter b with letter d
swap letter f with letter c
swap letter d with letter a
swap position 7 with position 6
rotate right 0 steps
swap position 0 with position 3
swap position 2 with position 5
swap letter h with letter f
reverse positions 2 through 3
rotate based on position of letter c
rotate left 2 steps
move position 0 to position 5
swap position 2 with position 3
rotate right 1 step
rotate left 2 steps
move position 0 to position 4
rotate based on position of letter c
rotate based on position of letter g
swap position 3 with position 0
rotate right 3 steps
reverse positions 0 through 2
move position 1 to position 2
swap letter e with letter c
rotate right 7 steps
move position 0 to position 7
rotate left 2 steps
reverse positions 0 through 4
swap letter e with letter b
reverse positions 2 through 7
rotate right 5 steps
swap position 2 with position 4
swap letter d with letter g
reverse positions 3 through 4
reverse positions 4 through 5";
