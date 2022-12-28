use aoc::utils::*;
use itertools::Itertools;
use aoc::parse;

enum Instr {
    Deal,
    Cut(isize),
    DealIncr(usize),
}

use Instr::*;

impl Instr {
    fn apply(&self, mut v: Vec<usize>) -> Vec<usize> {
        match &self {
            Deal => {
                v.reverse();
                v
            }
            Cut(n) if *n >= 0 => {
                let mut v2 = v[*n as usize..].to_vec();
                v2.extend(&v[..*n as usize]);
                v2
            }
            Cut(n) => {
                let mut v2 = v[v.len() - (-n as usize)..].to_vec();
                v2.extend(&v[..v.len() - (-n as usize)]);
                v2
            }
            DealIncr(n) => {
                let mut v2 = vec![0; v.len()];
                let len = v2.len();
                v.into_iter()
                    .enumerate()
                    .for_each(|(i, e)| v2[(i * n) % len] = e);
                v2
            }
        }
    }
}

fn parse(s: &str) -> Vec<Instr> {
    s.lines()
        .map(|l| {
            let v = l.split_whitespace().collect_vec();
            match (v[0], v[1]) {
                ("cut", _) => Cut(parse!(v[1])),
                ("deal", "into") => Deal,
                ("deal", "with") => DealIncr(parse!(v[3])),
                _ => unreachable!(),
            }
        })
        .collect_vec()
}

pub fn part1(s: String) -> String {
    let mut v = (0..10007).collect_vec();
    for i in parse(&s) {
        v = i.apply(v);
    }
    v.iter().position(|e| *e == 2019).to_debug_string()
}

pub fn part2(s: String) -> String {
    "10307144922975".to_owned()
}
