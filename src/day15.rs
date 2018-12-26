use itertools::*;
use chrono::{NaiveDateTime,Timelike};
use std::collections::{HashMap, HashSet};
use time::Duration;
use rayon::prelude::*;
use regex::*;
use std::mem;
use std::cmp::{Ordering};

const SIZE: usize = 33;
const AP: isize = 3;
const START_HP: isize = 200;

#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn dist(&self, rhs: &Point) -> usize {
        (self.x.max(rhs.x) - self.x.min(rhs.x)) +
        (self.y.max(rhs.y) - self.y.min(rhs.y))
    }

    fn adjacents(&self) -> impl Iterator<Item=Point> {
        let x = self.x as isize;
        let y = self.y as isize;
        vec![(x,y-1),(x-1,y),(x+1,y),(x,y+1)].into_iter()
            .filter(|(x1,y1)| *x1 >= 0 && *y1 >= 0)
            .filter(|(x1,y1)| *x1 < SIZE as isize && *y1 < SIZE as isize)
            .map(|(x1,y1)| Point {x: x1 as usize, y: y1 as usize})
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        self.y.cmp(&other.y).then(self.x.cmp(&other.x))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Team {
    Goblin,
    Elf,
}
use self::Team::*;

#[derive(Debug, Clone)]
struct Unit {
    hp: isize,
    team: Team,
}

struct Map {
    real_size: usize,
    grid: [[bool; SIZE]; SIZE],
    units: HashMap<Point, Unit>,
}

impl Map {
    fn print_cell(&self, p: &Point) -> char {
        if self.grid[p.y][p.x] {
            '.'
        } else {
            '#'
        }
    }

    fn print(&self) -> String {
        (0..self.real_size).map(|y| {
            let mut s = (0..self.real_size)
                .map(|x| Point {x: x, y: y})
                .map(|p| self.units.get(&p)
                            .map(|u| u.print())
                            .unwrap_or(self.print_cell(&p)))
                .join("");
            s.push_str("  ");
            s.push_str(&self.units.iter()
                .filter(|(p, _)| p.y == y)
                .map(|(_, u)| u.print_hp())
                .join(" ")
            );
            s
        })
        .join("\n")
    }
}

impl Unit {
    fn print(&self) -> char {
        match self.team {
            Goblin => 'G',
            Elf => 'E',
        }
    }

    fn print_hp(&self) -> String {
        format!("{}({})", self.print(), self.hp)
    }

    fn is_foe(&self, other: &Unit) -> bool {
        self.team != other.team
    }
}

fn parse_grid(s: &str) -> Map {
    let mut r = [[false; SIZE]; SIZE];
    let mut units = HashMap::new();
    let mut len = SIZE;
    for (y, l) in s.lines().enumerate() {
        if l.len() > 0 {
            // Record the min width that is non-zero encountered
            len = len.min(l.len());
        }
        for (x, c) in l.chars().enumerate() {
            let p = Point {x: x, y: y};
            r[y][x] = match c {
                'G' => {
                    units.insert(p, Unit {hp: START_HP, team: Goblin});
                    true
                },
                'E' => {
                    units.insert(p, Unit {hp: START_HP, team: Elf});
                    true
                },
                '.' => true,
                _ => false,
            };
        }
    }
    Map {
        grid: r,
        units: units,
        real_size: len.max(s.lines().count()),
    }
}

pub fn part1(input: String) -> String {
    let map = parse_grid(&input);
    println!("{}", map.print());
    "No answer".to_owned()
}

pub fn part2(input: String) -> String {
    "No answer".to_owned()
}

const EXAMPLE: &str = r#"
#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######
"#;