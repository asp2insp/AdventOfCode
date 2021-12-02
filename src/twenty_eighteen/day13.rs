use itertools::*;
use chrono::{NaiveDateTime,Timelike};
use std::collections::{HashMap, HashSet};
use rayon::prelude::*;
use regex::*;
use std::mem;

const SIZE: usize = 150;

type MAP = [[char; SIZE+2]; SIZE+2];

#[derive(Debug, Clone)]
struct Cart {
    x: usize,
    y: usize,
    d: char,
    i: usize,
}

impl Cart {
    fn step(&mut self, map: &MAP) {
        let c = self;
        let road = just_ahead(&map, c);
        let road = if road == '+' {
            let r = match (c.i % 3, c.d) {
                (0, '^') | (0, 'v') => '\\',
                (0, '<') | (0, '>') => '/',

                (1, 'v') | (1, '^') => '|',
                (1, '<') | (1, '>') => '-',

                (2, '^') | (2, 'v') => '/',
                (2, '<') | (2, '>') => '\\',

                (n, d) => unreachable!("Bad intersection: {}, {}", n, d),
            };
            c.i += 1;
            r
        } else {
            road
        };
        match (c.d, road) {
            ('>', '-') => c.x += 1,
            ('<', '-') => c.x -= 1,
            ('^', '|') => c.y -= 1,
            ('v', '|') => c.y += 1,

            ('v', '\\') => {c.y += 1; c.d = '>';},
            ('v', '/') => {c.y += 1; c.d = '<';},
            ('>', '\\') => {c.x += 1; c.d = 'v';},
            ('>', '/') => {c.x += 1; c.d = '^';},
            ('<', '\\') => {c.x -= 1; c.d = '^';},
            ('<', '/') => {c.x -= 1; c.d = 'v';},
            ('^', '\\') => {c.y -= 1; c.d = '<';},
            ('^', '/') => {c.y -= 1; c.d = '>';},

            (d, r) => {
                unreachable!("Bad road: {} {} (orig {}) at {},{}:\n{}", d, r, just_ahead(&map, c), c.x,c.y, print_grid(map, &[c.clone()]))
            },
        };
    }
}


fn parse_grid(s: &str) -> (MAP, Vec<Cart>) {
    let mut r = [[' '; SIZE+2]; SIZE+2];
    let mut carts = vec![];
    for (yn, l) in s.lines().enumerate() {
        for (xn, c) in l.chars().enumerate() {
            let (x,y) = (xn+1, yn+1);
            r[y][x] = match c {
                '>' => {
                    carts.push(Cart {x: x, y: y, d: '>', i: 0});
                    '-'
                },
                '<' => {
                    carts.push(Cart {x: x, y: y, d: '<', i: 0});
                    '-'
                },
                '^' => {
                    carts.push(Cart {x: x, y: y, d: '^', i: 0});
                    '|'
                },
                'v' => {
                    carts.push(Cart {x: x, y: y, d: 'v', i: 0});
                    '|'
                },
                other => other,
            };
        }
    }
    (r, carts)
}

fn just_ahead(map: &MAP, c: &Cart) -> char {
    match c.d {
        '^' => map[c.y-1][c.x],
        'v' => map[c.y+1][c.x],
        '>' => map[c.y][c.x+1],
        '<' => map[c.y][c.x-1],
        bad => unreachable!("Bad direction: {}", bad),
    }
}

fn step(map: &MAP, mut carts: Vec<Cart>) -> (Option<(usize, usize)>, Vec<Cart>) {
    carts.sort_by(|a, b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)));
    for i in 0..carts.len() {
        carts[i].step(map);
        if let Some(loc) = find_collision(&carts) {
            return (Some(loc), carts);
        }
    }
    
    (None, carts)
}

fn step_with_removal(map: &MAP, mut carts: Vec<Cart>) -> Vec<Cart> {
    carts.sort_by(|a, b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)));
    let mut to_remove = vec![];
    for i in 0..carts.len() {
        carts[i].step(map);
        if let Some(loc) = find_collision(&carts) {
            to_remove.extend(carts.iter().enumerate().filter(|(i, c)| (0..carts.len()).map(|j| *i !=j && c.x == carts[j].x && c.y == carts[j].y).any(|b| b)).map(|(i, _)| i));
        }
    }
    to_remove.sort();
    for i in to_remove.into_iter().rev().dedup() {
        carts.remove(i);
    }
    carts
}

fn find_collision(carts: &[Cart]) -> Option<(usize, usize)> {
    let mut seen = HashSet::new();
    for c in carts {
        if seen.contains(&(c.x, c.y)) {
            return Some((c.x-1, c.y-1))
        }
        seen.insert((c.x, c.y));
    }
    None
}

fn print_grid(map: &MAP, carts: &[Cart]) -> String {
    // Find a bounding box:
    let (mx, my) = carts.iter().map(|c| (c.x, c.y)).fold((0,0), |(mx, my), (x, y)| (mx.max(x), my.max(y)));
    (0..=my+1).map(|y| {
        (0..=mx+1).map(|x| {
            carts.iter().filter(|c| c.x == x && c.y == y).next().map(|c| c.d).unwrap_or(map[y][x])
        })
        .join("")
    })
    .join("\n")
}

pub fn part1(input: String) -> String {
    let (map, mut carts) = parse_grid(&input);
    for i in 0.. {
        let (collided, next) = step(&map, carts);
        if let Some(loc) = collided {
            return format!("Crashed after {} steps at {},{}", i, loc.0, loc.1)
        }
        carts = next;
    }
    unreachable!("FOUND INF")
}

pub fn part2(input: String) -> String {
    let (map, mut carts) = parse_grid(&input);
    for i in 0.. {
        carts = step_with_removal(&map, carts);
        // println!("{}", print_grid(&map, &carts));
        if carts.len() < 2 {
            return format!("Last remaining: {},{}", carts[0].x-1, carts[0].y-1)
        }
    }
    unreachable!("FOUND INF")
}

const EXAMPLE: &str = r#"
/>-<\  
|   |  
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/
"#;