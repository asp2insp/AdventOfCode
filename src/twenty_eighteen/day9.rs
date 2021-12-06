use chrono::{NaiveDateTime, Timelike};
use itertools::*;
use rayon::prelude::*;
use regex::*;
use std::collections::{HashMap, VecDeque};
use std::mem;

// const P: usize = 10;
// const M: usize = 1618;
const P: usize = 423;
const M: usize = 71944;

#[derive(Debug)]
struct Circle {
    left_list: VecDeque<usize>,
    right_list: VecDeque<usize>,
}

impl Circle {
    fn new() -> Circle {
        Circle {
            left_list: VecDeque::new(),
            right_list: VecDeque::new(),
        }
    }

    fn left(&mut self) -> &mut Circle {
        if self.left_list.len() > 0 {
            let n = self.left_list.pop_back().unwrap();
            self.right_list.push_front(n);
        } else {
            for _ in 0..self.right_list.len() {
                self.right();
            }
            self.left();
        }
        self
    }

    fn right(&mut self) -> &mut Circle {
        if self.right_list.len() > 0 {
            let n = self.right_list.pop_front().unwrap();
            self.left_list.push_back(n);
        } else {
            for _ in 0..self.left_list.len() {
                self.left();
            }
            self.right();
        }
        self
    }

    fn insert(&mut self, elem: usize) -> &mut Circle {
        self.left_list.push_back(elem);
        self
    }

    fn remove(&mut self) -> usize {
        self.left_list.pop_back().unwrap()
    }
}

pub fn part1(input: String) -> String {
    let mut marbles = Circle::new();
    marbles.insert(0);
    let mut scores = [0usize; P];
    for m in 1..=M {
        let player = (m - 1) % P;
        if m % 23 == 0 {
            scores[player] += m;
            scores[player] += marbles
                .left()
                .left()
                .left()
                .left()
                .left()
                .left()
                .left()
                .remove();
            marbles.right();
        } else {
            marbles.right().insert(m);
        }
        // println!("[{}] {:?}", player + 1, marbles);
    }
    format!("{}", scores.iter().max().unwrap())
}

pub fn part2(input: String) -> String {
    let mut marbles = Circle::new();
    marbles.insert(0);
    let mut scores = [0usize; P];
    for m in 1..=(M * 100) {
        let player = (m - 1) % P;
        if m % 23 == 0 {
            scores[player] += m;
            scores[player] += marbles
                .left()
                .left()
                .left()
                .left()
                .left()
                .left()
                .left()
                .remove();
            marbles.right();
        } else {
            marbles.right().insert(m);
        }
        // println!("[{}] {:?}", player + 1, marbles);
    }
    format!("{}", scores.iter().max().unwrap())
}
