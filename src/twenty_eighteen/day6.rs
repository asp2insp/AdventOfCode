use itertools::*;
use chrono::{NaiveDateTime,Timelike};
use std::collections::HashMap;
use time::Duration;
use rayon::prelude::*;
use regex::*;
use std::mem;

const GS: usize = 500;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(?P<x>\d+), (?P<y>\d+)").unwrap();
}

struct Point {
    x: isize,
    y: isize,
}

#[derive(Copy, Clone, Debug)]
struct Dist {
    id: usize,
    dist: usize,
}

fn parse_lines(s: &str) -> HashMap<usize, Point> {
    s.lines()
        .map(|l|{
            let cap = RE.captures(l).unwrap();
            Point {
                x: cap["x"].parse().unwrap(),
                y: cap["y"].parse().unwrap(),
            }
        })
        .enumerate()
        .collect()
}


fn dist(x1: isize, y1: isize, x2: isize, y2: isize) -> usize {
    ((x1 - x2).abs() + (y1 - y2).abs() ) as usize
}

fn print_grid(grid: &[[Dist; GS]; GS]) {
    let mut r = Vec::with_capacity(GS);
    for x in 0..GS {
        let mut c = String::new();
        for y in 0..GS {
            if grid[x][y].id == usize::max_value() {
                c.push_str("**");
            } else {
                c.push_str(&format!("{:_>width$}", grid[x][y].id, width=2));
            }
            c.push_str(",");
        }
        r.push(c);
    }
    println!("{:#?}", r);
}

pub fn part1(input: String) -> String {
    let points = parse_lines(&input);
    let mut grid = [[Dist {id: usize::max_value(), dist: usize::max_value()}; GS]; GS];
    for x in 0..GS {
        for y in 0..GS {
            for (id, loc) in &points {
                let candidate_dist = dist(x as isize, y as isize, loc.x, loc.y);
                if candidate_dist < grid[x][y].dist {
                    grid[x][y] = Dist {id: *id, dist: candidate_dist};
                } else if candidate_dist == grid[x][y].dist && candidate_dist != 0 {
                    grid[x][y] = Dist {id: usize::max_value(), dist: candidate_dist};
                }
            }
        }
    }
    let mut m = 0;
    'outer: for id in points.keys() {
        // Discard any which touch the edge
        for x in 0..GS {
            if grid[x][0].id == *id || grid[x][GS-1].id == *id {
                continue 'outer;
            }
        }
        for y in 0..GS {
            if grid[0][y].id == *id || grid[GS-1][y].id == *id {
                continue 'outer;
            }
        }
        let score = (0..GS).cartesian_product((0..GS))
            .filter(|(x, y)| grid[*x][*y].id == *id)
            .count();
        // println!("{}: {}", id, score);
        m = m.max(score);
    }
    // print_grid(&grid);
    format!("{}", m)
}

pub fn part2(input: String) -> String {
    let points = parse_lines(&input);
    let mut size = 0;
    let min = (GS as isize) * -1;
    for x in min..GS as isize {
        for y in min..GS as isize {
            if points.values().map(|loc| dist(x, y, loc.x, loc.y)).sum::<usize>() < 10_000 {
                size += 1;
            }
        }
    }
    format!("{}", size)
}