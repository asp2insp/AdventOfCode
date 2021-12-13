use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

use crate::utils::IterUtils;

fn is_large(s: &str) -> bool {
    s.chars().next().unwrap().is_uppercase()
}

fn neighbors<'a>(
    node: &'a str,
    adj_list: &Vec<(&'a str, &'a str)>,
    seen: &Vec<&'a str>,
) -> Vec<&'a str> {
    let fwd = adj_list
        .iter()
        .filter(|n| n.0 == node && (is_large(n.1) || !seen.contains(&n.1)))
        .map(|n| n.1);
    let bkw = adj_list
        .iter()
        .filter(|n| n.1 == node && (is_large(n.0) || !seen.contains(&n.0)))
        .map(|n| n.0);
    fwd.chain(bkw).sorted().dedup().collect_vec()
}

pub fn part1(input: String) -> String {
    let adj_list = input
        .lines()
        .map(|l| {
            let mut parts = l.trim().split("-");
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .collect_vec();
    let mut paths: HashSet<Vec<&str>> = HashSet::new();
    let mut q: VecDeque<Vec<&str>> = VecDeque::new();
    q.push_back(vec!["start"]);

    while let Some(mut path) = q.pop_front() {
        let node: &str = path.last().unwrap();
        if node == "end" {
            path.push(node);
            paths.insert(path);
            continue;
        }
        for n in neighbors(node, &adj_list, &path) {
            let mut pnew = path.clone();
            pnew.push(n);
            q.push_back(pnew);
        }
    }

    paths.len().to_string()
}

fn neighbors2<'a>(
    node: &'a str,
    adj_list: &Vec<(&'a str, &'a str)>,
    seen: &Vec<&'a str>,
) -> Vec<&'a str> {
    let seen = seen.iter().filter(|n| !is_large(n)).counting_set();
    let has_double = seen.values().any(|c| *c > 1);
    let fwd = adj_list
        .iter()
        .filter(|n| n.0 == node && (is_large(n.1) || !seen.contains_key(&n.1) || !has_double))
        .map(|n| n.1);
    let bkw = adj_list
        .iter()
        .filter(|n| n.1 == node && (is_large(n.0) || !seen.contains_key(&n.0) || !has_double))
        .map(|n| n.0);
    fwd.chain(bkw)
        .filter(|n| n != &"start")
        .sorted()
        .dedup()
        .collect_vec()
}

pub fn part2(input: String) -> String {
    let adj_list = input
        .lines()
        .map(|l| {
            let mut parts = l.trim().split("-");
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .collect_vec();
    let mut paths: HashSet<Vec<&str>> = HashSet::new();
    let mut q: VecDeque<Vec<&str>> = VecDeque::new();
    q.push_back(vec!["start"]);

    while let Some(mut path) = q.pop_front() {
        let node: &str = path.last().unwrap();
        if node == "end" {
            path.push(node);
            paths.insert(path);
            continue;
        }
        for n in neighbors2(node, &adj_list, &path) {
            let mut pnew = path.clone();
            pnew.push(n);
            q.push_back(pnew);
        }
    }
    // println!("{:#?}", paths.iter().map(|p| p.len()).max().unwrap());
    paths.len().to_string()
}

#[test]
fn test() {
    let s = "fs-end
	he-DX
	fs-he
	start-DX
	pj-DX
	end-zg
	zg-sl
	zg-pj
	pj-he
	RW-he
	fs-DX
	pj-RW
	zg-RW
	start-pj
	he-WI
	zg-he
	pj-fs
	start-RW";

    assert_eq!("226", part1(s.to_string()));
    assert_eq!("3509", part2(s.to_string()));
}

#[test]
fn test_small() {
    let s = "start-A
	start-b
	A-c
	A-b
	b-d
	A-end
	b-end";
    assert_eq!("10", part1(s.to_string()));
    assert_eq!("36", part2(s.to_string()));
}
