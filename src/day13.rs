use itertools::*;
use chrono::{NaiveDateTime,Timelike};
use std::collections::HashMap;
use time::Duration;
use rayon::prelude::*;
use regex::*;
use std::mem;

const SIZE: usize = 150;

enum Cell {
    Horizontal,
    Vertical,
    Intersection,
    

}

fn parse_grid(s: &str) -> []

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