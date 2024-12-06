use std::collections::HashMap;

use aoc::utils::ToDebugString;
use num::integer::lcm;
use regex::Regex;

fn parse<'a>(l: impl Iterator<Item = &'a str>) -> HashMap<String, Vec<String>> {
    let re = Regex::new(r"^(\w+) = \((\w+), (\w+)\)$").unwrap();
    l.map(|l| {
        let caps = re.captures(l).unwrap();
        (
            caps.get(1).unwrap().as_str().to_string(),
            vec![
                caps.get(2).unwrap().as_str().to_string(),
                caps.get(3).unwrap().as_str().to_string(),
            ],
        )
    })
    .collect()
}

pub fn part1(input: String) -> String {
    let mut l = input.lines();
    let instrs = l
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => panic!(),
        })
        .collect::<Vec<_>>();
    let map = parse(l.skip(1));

    let mut count = 0;
    let mut pos = "AAA".to_owned();
    while pos != "ZZZ" {
        let branches = map.get(&pos).unwrap();
        pos = branches[instrs[count % instrs.len()]].clone();
        count += 1;
    }
    count.to_string()
}

fn cycle_size(start: &String, map: &HashMap<String, Vec<String>>, instrs: &Vec<usize>) -> usize {
    let mut count = 0;
    let mut pos = start.clone();
    while !pos.ends_with("Z") {
        pos = map.get(&pos).unwrap()[instrs[count % instrs.len()]].clone();
        count += 1;
    }
    count
}

pub fn part2(input: String) -> String {
    let mut l = input.lines();
    let instrs = l
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => panic!(),
        })
        .collect::<Vec<usize>>();
    let map = parse(l.skip(1));

    map.keys()
        .filter(|k| k.ends_with("A"))
        .cloned()
        .map(|k| cycle_size(&k, &map, &instrs))
        .reduce(|acc, n| lcm(acc, n))
        .to_debug_string()
}
