//! ```cargo
//! [dependencies]
//! itertools = "0.7"
//! rust-crypto = "0.2"
//! chomp = "0.3.1"
//! regex = "0.2.0"
//! rayon = "0.9.0"
//! ```
#![allow(unused)]


extern crate regex;
extern crate itertools;
#[macro_use]
extern crate chomp;
extern crate rayon;

use rayon::prelude::*;
use regex::Regex;
use std::iter;
use itertools::*;
use std::collections::{VecDeque,HashSet,HashMap};
use std::{mem,fmt,thread};
use chomp::prelude::{U8Input, SimpleResult, parse_only, many1, any, string, token, take_while};
use chomp::ascii::{decimal, skip_whitespace, is_whitespace, is_alphanumeric, is_alpha, signed};
use chomp::types::Buffer;
use std::fs::File;
use std::io::prelude::*;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::time::Duration;

struct Particle {
    id: usize,
    pos: (isize, isize, isize),
    vel: (isize, isize, isize),
    acc: (isize, isize, isize),
}

fn get_input() -> Vec<Particle> {
    let pat: Regex = Regex::new(
        r"p=<(?P<px>[-\d]+),(?P<py>[-\d]+),(?P<pz>[-\d]+)>, v=<(?P<vx>[-\d]+),(?P<vy>[-\d]+),(?P<vz>[-\d]+)>, a=<(?P<ax>[-\d]+),(?P<ay>[-\d]+),(?P<az>[-\d]+)>"
    ).unwrap();
    let mut s = String::new();
    File::open("input20.txt").expect("!!").read_to_string(&mut s);
    s.lines()
        .enumerate()
        .map(|(id, l)| {
            let data = pat.captures(l).unwrap();
            Particle {
                id: id,
                pos: (
                    data["px"].parse().unwrap(),
                    data["py"].parse().unwrap(),
                    data["pz"].parse().unwrap(),
                ),
                vel: (
                    data["vx"].parse().unwrap(),
                    data["vy"].parse().unwrap(),
                    data["vz"].parse().unwrap(),
                ),
                acc: (
                    data["ax"].parse().unwrap(),
                    data["ay"].parse().unwrap(),
                    data["az"].parse().unwrap(),
                ),
            }
        })
        .collect()
}

fn simulate(v: &mut Vec<Particle>) {
    v.par_iter_mut()
        .for_each(|p| {
            p.vel.0 += p.acc.0;
            p.vel.1 += p.acc.1;
            p.vel.2 += p.acc.2;

            p.pos.0 += p.vel.0;
            p.pos.1 += p.vel.1;
            p.pos.2 += p.vel.2;
        });
}

fn simulate_with_collision(v: Vec<Particle>) -> Vec<Particle> {
    let mut v = v;
    simulate(&mut v);
    let mut counts = HashMap::new();
    for p in &v {
        *counts.entry(p.pos).or_insert(0) += 1;
    }
    counts.retain(|_, count| *count > 1);
    v.into_iter()
        .filter(|p| !counts.contains_key(&p.pos))
        .collect()
}

fn part_one() -> usize {
    let mut p = get_input();
    for _ in 0..1000 {
        simulate(&mut p);
    }
    p.iter().min_by(|p1, p2|
        (p1.pos.0.abs() + p1.pos.1.abs() + p1.pos.2.abs())
        .cmp(&(p2.pos.0.abs() + p2.pos.1.abs() + p2.pos.2.abs()))
    )
    .unwrap()
    .id
}

fn part_two() -> usize {
    let mut p = get_input();
    for _ in 0..1000 {
        p = simulate_with_collision(p);
    }
    p.len()
}

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}
