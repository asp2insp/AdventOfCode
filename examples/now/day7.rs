use std::collections::VecDeque;

use aoc::dict;
use fnv::FnvHashMap;
use itertools::Itertools;

const THRESHOLD: usize = 100_000;

enum Line {
    Cd(String),
    Ls,
    Dir(String),
    File(usize, String),
}

struct Dir {
    files: Vec<(usize, String)>,
    children: Vec<String>,
}

impl Dir {
    fn new() -> Self {
        Dir {
            files: vec![],
            children: vec![],
        }
    }
}

use Line::*;

fn parse(s: &str) -> Vec<Line> {
    s.lines()
        .map(|l| {
            if l.starts_with("$ cd") {
                Cd(l.strip_prefix("$ cd ").unwrap().to_owned())
            } else if l.starts_with("dir ") {
                Dir(l.strip_prefix("dir ").unwrap().to_owned())
            } else if l.starts_with("$ ls") {
                Ls
            } else {
                let mut parts = l.split_whitespace();
                File(
                    parts.next().unwrap().parse::<usize>().unwrap(),
                    parts.next().unwrap().to_owned(),
                )
            }
        })
        .collect_vec()
}

pub fn find_sizes(s: &str) -> FnvHashMap<String, usize> {
    let mut dirs = dict! {"".to_owned() => Dir::new()};
    let mut stack = vec!["".to_owned()];
    for line in parse(s) {
        match line {
            Cd(dst) => match dst.as_str() {
                "/" => {
                    stack.resize(1, String::new());
                }
                ".." => {
                    stack.pop();
                }
                dst => {
                    stack.push(dst.to_owned());
                }
            },
            Dir(name) => {
                let curr = dirs.entry(stack.join("/")).or_insert(Dir::new());
                curr.children.push(format!("{}/{}", stack.join("/"), name));
            }
            Ls => {}
            File(size, name) => {
                let curr = dirs.entry(stack.join("/")).or_insert(Dir::new());
                curr.files.push((size, name));
            }
        }
    }
    let mut cache = dict! {};
    let mut q = VecDeque::new();
    q.push_back("".to_owned());
    let mut nq = vec![];
    while let Some(next) = q.pop_front() {
        nq.push(next.clone());
        dirs.get(&next).unwrap().children.iter().for_each(|c| {
            q.push_back(c.clone());
        });
    }

    while let Some(next) = nq.pop() {
        let d = dirs.get(&next).unwrap();
        let value = d.files.iter().map(|&(n, _)| n).sum::<usize>()
            + d.children
                .iter()
                .map(|c| cache.get(c).unwrap())
                .sum::<usize>();
        cache.insert(next, value);
    }
    cache
}

pub fn part1(input: String) -> String {
    find_sizes(&input)
        .values()
        .filter(|n| **n <= THRESHOLD)
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: String) -> String {
    let sizes = find_sizes(&input);
    let total_size = sizes.get("").unwrap();
    let needed = 30000000 - (70000000 - total_size);
    sizes
        .values()
        .filter(|n| **n > needed)
        .min()
        .unwrap()
        .to_string()
}
