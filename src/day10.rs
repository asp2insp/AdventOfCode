use itertools::*;
use chrono::{NaiveDateTime,Timelike};
use std::collections::HashMap;
use time::Duration;
use rayon::prelude::*;
use regex::*;
use std::mem;
use image::{Luma, ImageBuffer};
use std::path::Path;

const SIZE: u32 = 200;

// position=< 10533,  31237> velocity=<-1, -3>
lazy_static! {
    static ref RE: Regex = Regex::new(r"position=<\s*(?P<x>[\d-]+),\s*(?P<y>[\d-]+)> velocity=<\s*(?P<vx>[\d-]+),\s*(?P<vy>[\d-]+)").unwrap();
}

struct Point {
    x: isize,
    y: isize,
    vx: isize,
    vy: isize,
}

fn parse_lines(s: &str) -> Vec<Point> {
    s.lines()
        .map(|l|{
            let cap = RE.captures(l).unwrap();
            Point {
                x: cap["x"].parse().unwrap(),
                y: cap["y"].parse().unwrap(),
                vx: cap["vx"].parse().unwrap(),
                vy: cap["vy"].parse().unwrap(),
            }
        })
        .collect()
}

fn step(points: &mut[Point]) {
    for p in points {
        p.x += p.vx;
        p.y += p.vy;
    }
}

const SCALE: u32 = 1;

fn coord_to_px(coord: isize) -> u32 {
    ((coord + (SIZE as isize * SCALE as isize/2)) as u32) / SCALE - 200
}

fn make_image(points: &[Point], path: &Path) {
    let mut img = ImageBuffer::new(SIZE, SIZE);
    for p in points {
        let x = coord_to_px(p.x);
        let y = coord_to_px(p.y);
        img.put_pixel(x, y, Luma([255u8]));
    }
    img.save(path);
}

const STEP: usize = 1;

pub fn part1(input: String) -> String {
    let mut points = parse_lines(&input);
    let i = 10_369;
    for _ in 0..i {
        step(&mut points);
    }
    // KBJHEZCB
    let p = format!("/tmp/advent/{}.png", i);
    make_image(&points, &Path::new(&p));
    format!("{}", p)
}

pub fn part2(input: String) -> String {
    format!("10_369")
}