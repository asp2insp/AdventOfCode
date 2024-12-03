use aoc::{dict, parse, utils::*};
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<HashMap<String, usize>>,
}

fn parse_game(s: &str) -> Game {
    let mut parts = s.split(':');
    let gid = gimme_usizes_once(parts.next().unwrap())[0];
    let setss = parts.next().unwrap().split(';');
    let mut g = Game {
        id: gid,
        sets: vec![],
    };

    for s in setss {
        g.sets.push(
            s.split(',')
                .map(|c| c.trim().split(' ').collect_tuple::<(&str, &str)>().unwrap())
                .map(|(v, k)| (k.to_owned(), parse!(v, usize)))
                .collect::<HashMap<String, usize>>(),
        );
    }
    g
}

pub fn part1(input: String) -> String {
    let limits = dict!(
        "red".to_owned() => 12usize,
        "green".to_owned() => 13,
        "blue".to_owned() => 14,
    );
    input
        .lines()
        .map(parse_game)
        .filter(|g| {
            !g.sets
                .iter()
                .flat_map(|s| s.iter())
                .any(|(k, v)| limits.get(k).unwrap_or(&100) < v)
        })
        .map(|g| g.id)
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: String) -> String {
    input
        .lines()
        .map(parse_game)
        .map(|g| {
            g.sets.iter().fold(
                dict!(
                    "red".to_owned() => 0usize,
                    "green".to_owned() => 0,
                    "blue".to_owned() => 0,
                ),
                |mut acc, n| {
                    for (k, v) in n {
                        acc.insert(k.clone(), *v.max(&acc[k]));
                    }
                    acc
                },
            )
        })
        .map(|gvals| gvals.values().product::<usize>())
        .sum::<usize>()
        .to_string()
}
