use itertools::*;
use chrono::{NaiveDateTime,Timelike};
use std::collections::HashMap;
use time::Duration;
use rayon::prelude::*;
use regex::*;
use std::mem;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(?P<pat>[#.]+) => (?P<res>[#.])").unwrap();
}

const INIT: &str = "#..#####.#.#.##....####..##.#.#.##.##.#####..####.#.##.....#..#.#.#...###..#..###.##.#..##.#.#.....#";
// const INIT: &str = "#..#.#..##......###...###";

#[derive(Debug)]
struct Rule {
    pat: Vec<bool>,
    res: bool,
}

impl Rule {
    fn matches(&self, window: &[bool]) -> bool {
        &self.pat[..] == window
    }
}

fn format_state(v: &Vec<bool>) -> String {
    v.iter()
        .map(|c| if *c {'#'} else {'.'})
        .collect()
}

fn to_bool_vec(s: &str) -> Vec<bool> {
    s.chars()
        .map(|c| c == '#')
        .collect()
}

fn parse_lines(s: &str) -> Vec<Rule> {
    s.lines()
        .map(|l|{
            let cap = RE.captures(l).unwrap();
            Rule {
                pat: to_bool_vec(&cap["pat"]),
                res: &cap["res"] == "#",
            }
        })
        .collect()
}

fn step(v: Vec<bool>, rules: &[Rule]) -> Vec<bool> {
    let mut r = vec![false; v.len()];
    for i in 2..v.len()-2 {
        let window = &v[i-2..i+3];
        for rule in rules {
            if rule.matches(window) {
                r[i] = rule.res;
                break
            }
        }
    }
    while r[r.len()-5..].iter().any(|b| *b) {
        r.push(false);
    }
    r
}

pub fn part1(input: String) -> String {
    let mut rules = parse_lines(&input);
    let mut state = vec![false; INIT.len()];
    state.extend_from_slice(&to_bool_vec(INIT));
    state.extend_from_slice(&vec![false; INIT.len()]);
    for _ in 0..20 {
        state = step(state, &rules);
        // println!("{}", format_state(&state))
    }
    format!("{}", state.into_iter().enumerate().map(|(i, c)| (i as isize - INIT.len() as isize, c)).filter(|(_, c)| *c).map(|(i, _)| i).sum::<isize>())
}


pub fn part2(input: String) -> String {
    let mut rules = parse_lines(&input);
    let mut state = vec![false; 5];
    state.extend_from_slice(&to_bool_vec(INIT));
    state.extend_from_slice(&vec![false; 5]);
    let first = 100;
    for _ in 0..first {
        state = step(state, &rules);
        // println!("{}", format_state(&state))
    }
    let steady_state: Vec<_> = state.into_iter().enumerate().map(|(i, c)| (i as isize - 5, c)).filter(|(_, c)| *c).map(|(i, _)| i as usize).collect();
    let rest = 50000000000 - first;
    let final_state: Vec<_> = steady_state.into_iter().map(|p| p+rest).collect();
    format!("{}", final_state.into_iter().sum::<usize>())
}