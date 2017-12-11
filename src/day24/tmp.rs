//! ```cargo
//! [dependencies]
//! itertools = "0.7"
//! rust-crypto = "0.2"
//! chomp = "0.3.1"
//! regex = "0.2.0"
//! permutohedron = "0.2"
//! ```
#![allow(unused)]

extern crate permutohedron;
extern crate regex;
extern crate itertools;
#[macro_use]
extern crate chomp;

use permutohedron::*;
use regex::Regex;
use std::iter;
use itertools::*;
use std::collections::{VecDeque,HashSet,HashMap,BinaryHeap};
use std::mem;
use std::fmt;
use chomp::prelude::{U8Input, SimpleResult, parse_only, many1, any, string, token, take_while};
use chomp::ascii::{decimal, skip_whitespace, is_whitespace, is_alphanumeric, is_alpha, signed};
use chomp::types::Buffer;
use std::fs::File;
use std::io::prelude::*;
use std::cmp::Ordering;
use self::Tile::*;

fn get_input() -> String {
    let mut s = String::new();
    File::open("input.txt").expect("!!").read_to_string(&mut s);
    s
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Soln {
    goal: (u16, u16),
    steps: u32,
    x: u16,
    y: u16,
}

impl Soln {
    fn score(&self) -> u32 {
        let (x, y) = self.goal;
        let manhat_dist = (self.x as isize - x as isize).abs() + (self.y as isize - y as isize).abs();
        1000_000 - self.steps - manhat_dist as u32
    }

    fn step_to(&self, x: usize, y: usize) -> Soln {
        Soln {
            x: x as u16,
            y: y as u16,
            steps: self.steps + 1,
            goal: self.goal,
        }
    }

    fn with_goal(&self, x: usize, y: usize) -> Soln {
        Soln {
            goal: (x as u16, y as u16),
            ..self.clone()
        }
    }
}

impl PartialOrd for Soln {
    fn partial_cmp(&self, other: &Soln) -> Option<Ordering> {
        Some(self.score().cmp(&other.score()))
    }
}

impl Ord for Soln {
    fn cmp(&self, other: &Soln) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

enum Tile {
    Wall,
    Open,
    Goal(u8),
}

impl Tile {
    fn can_pass(&self) -> bool {
        match self {
            &Wall => false,
            _ => true,
        }
    }

    fn get_goal(&self) -> Option<u8> {
        match self {
            &Goal(g) => Some(g),
            _ => None
        }
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Tile {
        match c {
            '#' => Wall,
            '.' => Open,
            _ => Goal(c as u8),
        }
    }
}

fn find_goal(g: u8, map: &Vec<Vec<Tile>>) -> (usize, usize) {
    for (y, v) in map.iter().enumerate() {
        for (x, t) in v.iter().enumerate() {
            if let &Goal(g2) = t {
                if g2 == g {
                    return (x,y)
                }
            }
        }
    }
    panic!("Not found");
}

fn get_move_if_valid(dx: isize, dy: isize, soln: &Soln, map: &Vec<Vec<Tile>>, visited: &mut HashSet<(usize, usize)>) -> Option<Soln> {
    let x = (soln.x as isize + dx) as usize;
    let y = (soln.y as isize + dy) as usize;
    let next = soln.step_to(x, y);
    if map[y][x].can_pass() && !visited.contains(&(x,y)){
        Some(next)
    } else {
        None
    }
}

const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

pub fn part_one() -> String {
    let map: Vec<Vec<Tile>> = get_input().lines()
        .map(|l| l.chars().map(|c| c.into()).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut all_goals = map.iter()
        .flat_map(|l| l.iter())
        .flat_map(|t| match t { &Goal(g) => Some(g), _ => None})
        .filter(|g| *g != b'0')
        .collect::<Vec<_>>();
    let num_goals = (all_goals.len() + 1) as u32;
    let (mut x, mut y) = find_goal(b'0', &map);
    let mut data = vec![1, 2, 3];
    let mut permutations = Heap::new(&mut all_goals);
    let mut q: BinaryHeap<Soln> = BinaryHeap::new();
    let mut results = vec![];
    for order in permutations {
        q.clear();
        q.push(Soln{x: x as u16, y: y as u16, steps: 0, goal: (0,0)});
        for target in order.iter().chain(iter::once(&b'0')) {
            let (gx, gy) = find_goal(*target, &map);
            let starting_point = q.pop().unwrap().with_goal(gx, gy);
            q.push(starting_point);
            let mut visited = HashSet::new();
            loop {
                let next = match q.pop() {
                    Some(n) => n,
                    _ => break,
                };
                if let Some(g) = map[next.y as usize][next.x as usize].get_goal() {
                    if g == *target {
                        q.clear();
                        q.push(next);
                        break
                    }
                }
                visited.insert((next.x as usize, next.y as usize));
                DIRS.iter().flat_map(|&(dx,dy)| get_move_if_valid(dx, dy, &next, &map, &mut visited)).foreach(|s| q.push(s));
            }
        }
        results.push(q.pop().map(|soln| soln.steps as isize).unwrap_or(-1));
    }
    format!("{}", results.iter().min().unwrap())
}


pub fn part_two() -> String {

    format!("{}", "Damn mazes")
}

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}
