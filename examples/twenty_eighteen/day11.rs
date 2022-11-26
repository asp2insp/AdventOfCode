use chrono::{NaiveDateTime, Timelike};
use itertools::*;
use lazy_static::lazy_static;
use rayon::prelude::*;
use regex::*;
use std::collections::HashMap;
use std::mem;

const SIZE: usize = 300;
const SERIAL: isize = 1788;

lazy_static! {
    static ref GRID: [[isize; SIZE]; SIZE] = {
        let mut grid = [[0isize; SIZE]; SIZE];
        (0..SIZE).cartesian_product(0..SIZE).for_each(|(x, y)| {
            grid[x][y] = (0..x)
                .cartesian_product(0..y)
                .map(|(xp, yp)| power_level(xp, yp))
                .sum();
        });
        grid
    };
}

fn power_level(x: usize, y: usize) -> isize {
    let rack_id = x as isize + 10;
    let n = (rack_id * y as isize + SERIAL) * rack_id;
    ((n / 100) % 10) - 5
}

fn g(x: isize, y: isize) -> isize {
    if x < 0 || y < 0 || x >= SIZE as isize || y >= SIZE as isize {
        0
    } else {
        GRID[x as usize][y as usize]
    }
}

// 1 2 3 4 5
// 5 6 7 8 9
// A B C D E
// F G H I J
// Power(D, 2)  =      J      -     9      -     H      +      7
// Power(x,y,s) = P(x+s, y+s) - P(x+s,y-1) - P(x-1,y+s) + P(x-1,y-1)
fn convo(x: usize, y: usize, s: usize) -> isize {
    if x + s >= SIZE || y + s >= SIZE {
        return 0;
    }
    let (x, y, offset) = (x as isize, y as isize, s as isize - 1);
    g(x + offset, y + offset) - g(x + offset, y - 1) - g(x - 1, y + offset) + g(x - 1, y - 1)
}

#[derive(Debug)]
pub struct Res {
    power: isize,
    x: usize,
    y: usize,
    size: usize,
}

pub fn grid_power(size: usize) -> Res {
    let m = (1..SIZE)
        .cartesian_product((1..SIZE))
        .map(|xy| (convo(xy.0, xy.1, size), xy.0, xy.1))
        .max_by_key(|pxy| pxy.0)
        .unwrap();
    Res {
        power: m.0,
        x: m.1 - 1,
        y: m.2 - 1,
        size: size,
    }
}

pub fn part1(input: String) -> String {
    format!("{:?}", grid_power(3))
}

pub fn part2(input: String) -> String {
    let max = (1usize..301)
        .map(grid_power)
        .max_by_key(|res| res.power)
        .unwrap();
    format!("{:?}", max)
}
