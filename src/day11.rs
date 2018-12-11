use itertools::*;
use chrono::{NaiveDateTime,Timelike};
use std::collections::HashMap;
use time::Duration;
use rayon::prelude::*;
use regex::*;
use std::mem;

const SIZE: usize = 300;
const SERIAL: isize = 1788;

fn power_level(x: usize, y: usize) -> isize {
    let rack_id = x as isize + 10;
    let n = (rack_id * y as isize + SERIAL) * rack_id;
    ((n / 100) % 10) - 5
}

fn vcell_power_level((x,y): (usize, usize), m: usize) -> isize {
    if x + m + 1 > SIZE || y + m + 1 > SIZE {
        0
    } else {
        (0..m).cartesian_product(0..m)
            .map(|(xp, yp)| power_level(x+xp, y+yp))
            .sum()
    }
}

#[derive(Debug)]
pub struct Res {
    power: isize,
    x: usize,
    y: usize,
    size: usize,
}

pub fn grid_power(size: usize) -> Res {
    let m = (1..SIZE).cartesian_product((1..SIZE))
        .map(|xy| (vcell_power_level(xy, size), xy.0, xy.1))
        .max_by_key(|pxy| pxy.0)
        .unwrap();
    Res {
        power: m.0,
        x: m.1,
        y: m.2,
        size: size,
    }
}

pub fn part1(input: String) -> String {
    format!("{:?}", grid_power(3))
}

pub fn part2(input: String) -> String {
    let max = (1usize..301)
        .into_par_iter()
        .map(grid_power)
        .max_by_key(|res| res.power)
        .unwrap();
    format!("{:?}", max)
}