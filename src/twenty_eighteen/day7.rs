use itertools::*;
use chrono::{NaiveDateTime,Timelike};
use std::collections::HashMap;
use rayon::prelude::*;
use regex::*;
use std::mem;
use lazy_static::lazy_static;

// Step L must be finished before step N can begin.
lazy_static! {
    static ref RE: Regex = Regex::new(r"Step (?P<dep>[A-Z]) must be finished before step (?P<item>[A-Z]) can begin.").unwrap();
}

fn parse_lines(s: &str) -> HashMap<char, Vec<char>> {
    s.lines()
        .map(|l|{
            let cap = RE.captures(l).unwrap();
            (
                cap["item"].chars().next().unwrap(),
                cap["dep"].chars().next().unwrap(),
            )
        })
        .fold(HashMap::new(), |mut map, (item, dep)| {
            map.entry(dep).or_insert(Vec::new());
            map.entry(item).or_insert(Vec::new()).push(dep);
            map
        })
}


pub fn part1(input: String) -> String {
    let mut deps = parse_lines(&input);
    let mut order = Vec::new();
    while deps.len() > 0 {
        let mut ready = None;
        for (item, deps) in &deps {
            if deps.iter().all(|d| order.contains(d)) {
                ready = match (ready, *item) {
                    (None, i) => Some(i),
                    (Some(j), i) if i < j => Some(i),
                    (Some(j), _) => Some(j),
                };
            }
        }
        // println!("Finished {:?}", ready);
        if let Some(done) = ready {
            deps.remove(&done);
            order.push(done);
        }
    }
    format!("{}", order.into_iter().join(""))
}

#[derive(Copy, Clone, Debug)]
struct Worker {
    task: char,
    time_left: usize,
}

impl Worker {
    fn tick_by(&mut self, count: usize) -> Option<char> {
        if self.is_idle() {
            None
        } else if self.time_left < count {
            panic!("Tried to tick by {}, but only {} remaining", count, self.time_left);
        } else if self.time_left == count {
            self.time_left = 0;
            let c = self.task;
            self.task = IDLE;
            Some(c)
        } else {
            self.time_left -= count;
            None
        }
    }

    fn remaining(&self) -> Option<usize> {
        if self.is_idle() {
            None
        } else {
            Some(self.time_left)
        }
    }

    fn is_idle(&self) -> bool {
        self.task == IDLE
    }
}

const IDLE: char = '#';

fn time_taken(task: char) -> usize {
    60 + task as u8 as usize - 64
}

pub fn part2(input: String) -> String {
    let mut deps = parse_lines(&input);
    let mut workers = [Worker {task: IDLE, time_left: 0}; 5];
    let mut clock = 0;
    let mut done = Vec::new();
    let target = deps.len();

    while done.len() < target {
        let mut ready = Vec::new();
        for (item, deps) in &deps {
            if deps.iter().all(|d| done.contains(d)) {
                ready.push(*item);
            }
        }
        ready.sort();
        for item in ready {
            for w in &mut workers {
                if w.task == IDLE {
                    w.task = item;
                    w.time_left = time_taken(item);
                    deps.remove(&item);
                    break
                }
            }
        }
        // println!("State {}) {:?}", clock, workers);
        let tb = workers.iter().flat_map(|w| w.remaining().into_iter()).min().unwrap();
        let finished: Vec<char> = workers.iter_mut().flat_map(|mut w| w.tick_by(tb).into_iter()).collect();
        clock += tb;
        // println!("{}) {:?}", clock, finished);
        done.extend(finished);
    }
    format!("{}", clock)
}