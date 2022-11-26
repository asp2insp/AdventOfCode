//! ```cargo
//! [dependencies]
//! itertools = "0.7"
//! rust-crypto = "0.2"
//! chomp = "0.3.1"
//! regex = "0.2.0"
//! rayon = "0.9.0"
//! ```
#![allow(unused)]


extern crate regex;
extern crate itertools;
#[macro_use]
extern crate chomp;
extern crate rayon;

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

fn get_init_set() -> HashSet<(isize, isize)> {
    INPUT.lines()
        .enumerate()
        .flat_map(|(r, l)| l.chars().enumerate().map(move |(c, i)| {
            ((r as isize, c as isize), i == '#')
        }))
        .filter(|&(_, b)| b)
        .map(|(a,_)| a)
        .collect()
}

fn get_init_map() -> HashMap<(isize,isize), State> {
    INPUT.lines()
        .enumerate()
        .flat_map(|(r, l)| l.chars().enumerate().map(move |(c, i)| {
            (
                (r as isize, c as isize),
                match i {
                    '.' => State::Clean,
                    '#' => State::Infected,
                    _ => panic!(),
                }
            )
        }))
        .collect()
}

#[derive(Clone)]
enum State {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

impl State {
    fn next(&self) -> State {
        match self {
            &State::Clean => State::Weakened,
            &State::Weakened => State::Infected,
            &State::Infected => State::Flagged,
            &State::Flagged => State::Clean,
        }
    }
}

fn part_two() -> usize {
    let mut map = get_init_map();
    let mut count = 0;
    let mut r = (INPUT.lines().count() / 2) as isize;
    let mut c = (INPUT.lines().next().unwrap().len() / 2) as isize;
    let mut dir = (-1isize, 0isize);
    for _ in 0..10000000 {
        let curr = map.get(&(r,c)).unwrap_or(&State::Clean).clone();
        match curr {
            State::Clean => {
                dir = match dir {
                    (-1,0) => (0,-1),
                    (0,1) => (-1,0),
                    (1,0) => (0,1),
                    (0,-1) => (1,0),
                    _ => panic!(),
                };
            },
            State::Weakened => {
                count += 1;
            },
            State::Infected => {
                dir = match dir {
                    (-1,0) => (0,1),
                    (0,1) => (1,0),
                    (1,0) => (0,-1),
                    (0,-1) => (-1,0),
                    _ => panic!(),
                };
            },
            State::Flagged => {
                dir = match dir {
                    (-1,0) => (1,0),
                    (0,1) => (0,-1),
                    (1,0) => (-1,0),
                    (0,-1) => (0,1),
                    _ => panic!(),
                };
            },
        };
        map.insert((r,c), curr.next());
        r += dir.0;
        c += dir.1;
    }
    count
}

fn part_one() -> usize {
    let mut map = get_init_set();
    let mut count = 0;
    let mut r = (INPUT.lines().count() / 2) as isize;
    let mut c = (INPUT.lines().next().unwrap().len() / 2) as isize;
    let mut dir = (-1isize, 0isize);
    for _ in 0..10000 {
        if map.contains(&(r, c)) {
            // println!("on {},{}, turning right, cleaning", r,c);
            dir = match dir {
                (-1,0) => (0,1),
                (0,1) => (1,0),
                (1,0) => (0,-1),
                (0,-1) => (-1,0),
                _ => panic!(),
            };
            map.remove(&(r,c));
        } else {
            // println!("on {},{}, turning left, infecting", r,c);
            dir = match dir {
                (-1,0) => (0,-1),
                (0,1) => (-1,0),
                (1,0) => (0,1),
                (0,-1) => (1,0),
                _ => panic!(),
            };
            count += 1;
            map.insert((r,c));
        }
        // println!("{:?}", map);
        r += dir.0;
        c += dir.1;
    }
    // println!("FINAL DIR/POS {:?}/{},{}", dir, r,c);
    count
}

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}

const TEST: &str =
"..#
#..
...";

const INPUT: &str =
"...###.#.#.##...##.#..##.
.#...#..##.#.#..##.#.####
#..#.#...######.....#####
.###.#####.#...#.##.##...
.#.#.##......#....#.#.#..
....##.##.#..##.#...#....
#...###...#.###.#.#......
..#..#.....##..####..##.#
#...#..####.#####...#.##.
###.#.#..#..#...##.#..#..
.....##..###.##.#.....#..
#.....#...#.###.##.##...#
.#.##.##.##.#.#####.##...
##.#.###..#.####....#.#..
#.##.#...#.###.#.####..##
#.##..#..##..#.##.####.##
#.##.#....###.#.#......#.
.##..#.##..###.#..#...###
#..#.#.#####.....#.#.#...
.#####..###.#.#.##..#....
###..#..#..##...#.#.##...
..##....##.####.....#.#.#
..###.##...#..#.#####.###
####.########.#.#..##.#.#
#####.#..##...####.#..#..";
