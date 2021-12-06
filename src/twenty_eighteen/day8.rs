use chrono::{NaiveDateTime, Timelike};
use itertools::*;
use rayon::prelude::*;
use regex::*;
use std::collections::HashMap;
use std::mem;

fn parse_lines(s: &str) -> Vec<usize> {
    s.split_whitespace().map(|l| l.parse().unwrap()).collect()
}

#[derive(Debug)]
struct Tree {
    children: Vec<Tree>,
    metadata: Vec<usize>,
}

impl Tree {
    fn meta_sum(&self) -> usize {
        self.metadata.iter().sum::<usize>()
            + self.children.iter().map(Tree::meta_sum).sum::<usize>()
    }

    fn value(&self) -> usize {
        if self.children.len() == 0 {
            self.metadata.iter().sum::<usize>()
        } else {
            self.metadata
                .iter()
                .cloned()
                .filter(|m| *m > 0 && *m <= self.children.len())
                .map(|i| self.children[i - 1].value())
                .sum::<usize>()
        }
    }
}

fn parse_tree(input: &[usize]) -> (Tree, usize) {
    let num_children = input[0];
    let num_meta = input[1];
    let mut t = Tree {
        children: Vec::with_capacity(num_children),
        metadata: Vec::with_capacity(num_meta),
    };
    let mut next = 2;
    for i in 0..num_children {
        let (c, n) = parse_tree(&input[next..]);
        t.children.push(c);
        next += n;
    }
    t.metadata.extend_from_slice(&input[next..next + num_meta]);
    next += num_meta;
    (t, next)
}

pub fn part1(input: String) -> String {
    let items = parse_lines(&input);
    let (root, _count) = parse_tree(&items);
    format!("{}", root.meta_sum())
}

pub fn part2(input: String) -> String {
    let items = parse_lines(&input);
    let (root, _count) = parse_tree(&items);
    format!("{}", root.value())
}
