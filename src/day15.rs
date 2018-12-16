use itertools::*;
use chrono::{NaiveDateTime,Timelike};
use std::collections::{HashMap, HashSet};
use time::Duration;
use rayon::prelude::*;
use regex::*;
use std::mem;
use std::cmp::{Ordering};

const SIZE: usize = 33;
// const SIZE: usize = 10;
const AP: isize = 3;
const START_HP: isize = 200;

type MAP = [[char; SIZE]; SIZE];

#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn e(&self, x: usize, y: usize) -> bool {
        self.x == x && self.y == y
    }

    fn dist(&self, rhs: &Point) -> usize {
        (self.x.max(rhs.x) - self.x.min(rhs.x)) +
        (self.y.max(rhs.y) - self.y.min(rhs.y))
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
    loc: Point,
    hp: isize,
    team: Team,
}

impl Unit {
    fn step(&self, map: &MAP, units: &[Unit]) -> (Option<Point>, Option<Point>) {
        let locs: Vec<_> = units.iter()
            .filter(|u| u.hp >= 0 && self.team != u.team)
            .flat_map(|u| adjacents(u.loc))
            .collect();

        // Calculate the attack in place if any
        if let Some(att_in_place) = adjacents(self.loc)
                .flat_map(|p| units.iter()
                    .find(|u| u.hp > 0 && u.loc == p && u.team != self.team))
                .sorted_by(|a, b| a.hp.cmp(&b.hp).then(a.loc.cmp(&b.loc)))
                .into_iter()
                .next()
                .map(|u| u.loc) {
            (None, Some(att_in_place))
        } else {
            // Try to move
            let unit_locs: HashSet<Point> = units.iter().filter(|u| u.hp > 0).map(|u| u.loc).collect();
            let moveto = locs.into_iter()
                .map(|to| a_star(self.loc, to, map, &unit_locs))
                .filter(|to| to.0 != Point {x: 0, y: 0})
                .min_by_key(|(_, d)| *d)
                .map(|(p, _)| p);

            let attack = adjacents(moveto.unwrap_or(self.loc))
                .flat_map(|p| units.iter()
                    .find(|u| u.hp > 0 && u.loc == p && u.team != self.team))
                .sorted_by(|a, b| a.hp.cmp(&b.hp).then(a.loc.cmp(&b.loc)))
                .into_iter()
                .next()
                .map(|u| u.loc);
            (moveto, attack)
        }

        
    }

    fn print(&self) -> char {
        match self.team {
            Goblin => 'G',
            Elf => 'E',
        }
    }

    fn print_hp(&self) -> String {
        format!("{}({})", self.print(), self.hp)
    }

    fn foe(&self) -> char {
        match self.team {
            Goblin => 'E',
            Elf => 'G',
        }
    }
}

fn a_star(from: Point, to: Point, map: &MAP, units: &HashSet<Point>) -> (Point, usize) {
    let mut open = vec![from];
    let mut paths = [[(Point {x: 0, y: 0}, usize::max_value()); SIZE]; SIZE];
    paths[from.y][from.x] = (Point {x: from.x, y: from.y}, 0);
    while !open.is_empty() {
        let next_i = open.iter()
            .enumerate()
            .min_by_key(|(_, p)| p.dist(&to)).map(|(i, _)| i)
            .unwrap();
        let q = open.remove(next_i);
        if q == to {
            let mut p = q;
            while paths[p.y][p.x].0 != from {
                p = paths[p.y][p.x].0;
            }
            return (p, paths[q.y][q.x].1)
        }
        let next_set: Vec<_> = adjacents(q)
            .filter(|succ| map[succ.y][succ.x] == '.')
            .filter(|succ| !units.contains(succ))
            .filter(|succ| !open.contains(succ))
            .flat_map(|succ| {
                if paths[succ.y][succ.x].1 > paths[q.y][q.x].1 {
                    paths[succ.y][succ.x] = (q, paths[q.y][q.x].1 + 1);
                    Some(succ)
                } else {
                    None
                }
            })
            .collect();
        open.extend(next_set);
    }
    // No path exists
    (Point {x: 0, y: 0}, usize::max_value())
}

fn adjacents(p: Point) -> impl Iterator<Item=Point> {
    let Point {x, y} = p;
    vec![(x,y-1),(x-1,y),(x+1,y),(x,y+1)].into_iter()
        .filter(|(x1,y1)| *x1 < SIZE && *y1 < SIZE)
        .map(|(x1,y1)| Point {x: x1, y: y1})
}

fn parse_grid(s: &str) -> (MAP, Vec<Unit>) {
    let mut r = [['#'; SIZE]; SIZE];
    let mut units = vec![];
    for (y, l) in s.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            let p = Point {x: x, y: y};
            r[y][x] = match c {
                'G' => {
                    units.push(Unit {loc: p, hp: START_HP, team: Goblin});
                    '.'
                },
                'E' => {
                    units.push(Unit {loc: p, hp: START_HP, team: Elf});
                    '.'
                },
                other => other,
            };
        }
    }
    (r, units)
}

fn step(map: &MAP, mut units: Vec<Unit>) -> Vec<Unit> {
    units.sort_by_key(|u| u.loc);
    for i in 0..units.len() {
        println!("Round X, unit {}", i);
        if units[i].hp <= 0 {
            continue
        }
        let (moveto, attack) = units[i].step(map, &units);
        if let Some(p) = moveto {
            units[i].loc = p;
        }
        if let Some(att) = attack {
            units.iter_mut().find(|u| u.loc == att).map(|u| u.hp -= AP);
        }
    }
    units.into_iter().filter(|n| n.hp > 0).collect()
}

fn print_grid(map: &MAP, units: &[Unit]) -> String {
    (0..SIZE).map(|y| {
        format!("{} {}", (0..SIZE).map(|x| {
            units.iter().filter(|u| u.loc.e(x,y)).next().map(Unit::print).unwrap_or(map[y][x])
        })
        .join(""),
        units.iter().filter(|u| u.loc.y == y).map(|u| u.print_hp()).join(" "))
    })
    .join("\n")
}

pub fn part1(input: String) -> String {
    let (map, mut units) = parse_grid(&input);
    for i in 0.. {
        let next = step(&map, units);
        units = next;
        println!("{}", print_grid(&map, &units));
        let hps = units.iter().fold((0, 0), |acc, n| match n.team {
            Goblin => (acc.0 + n.hp, acc.1),
            Elf => (acc.0, acc.1 + n.hp),
        });
        if hps.0 == 0 || hps.1 == 0 {
            let hp_remains = hps.0.max(hps.1) as usize;
            return format!("{}*{} = {}", i, hp_remains, i * hp_remains)
        }
    }
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