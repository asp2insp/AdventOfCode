use crate::utils::*;
use itertools::*;
use std::collections::HashMap;

fn parse(s: &str) -> (&str, HashMap<(char, char), char>) {
    let mut l = s.lines();
    let poly = l.next().unwrap();
    (
        poly,
        l.skip(1)
            .flat_map(|s| s.split(" -> ").collect_tuple::<(_, _)>())
            .map(|(l, r)| {
                (
                    l.chars().collect_tuple::<(_, _)>().unwrap(),
                    r.chars().next().unwrap(),
                )
            })
            .collect::<HashMap<_, _>>(),
    )
}

fn run_step(s: String, t: &HashMap<(char, char), char>) -> String {
    s.chars()
        .tuple_windows::<(_, _)>()
        .flat_map(|(a, b)| {
            let c = t[&(a, b)];
            [a, c].into_iter()
        })
        .chain(s.chars().last().into_iter())
        .join("")
}

pub fn part1(input: String) -> String {
    let (poly, transitions) = parse(&input);
    let mut p = poly.to_string();
    for _ in 0..10 {
        // println!("{}", p);
        p = run_step(p, &transitions);
    }
    let counts = p.chars().counting_set();
    format!(
        "{}",
        counts.values().max().unwrap() - counts.values().min().unwrap()
    )
}

fn run_poly(
    (a, b, c): (char, char, char),
    n: usize,
    t: &HashMap<(char, char), char>,
) -> HashMap<char, usize> {
    if n == 0 {
        [a, b, c].into_iter().counting_set()
    } else {
        let left = (a, t[&(a, b)], b);
        let right = (b, t[&(b, c)], c);
        let mut ret = add_counting_sets(run_poly(left, n - 1, t), run_poly(right, n - 1, t));
        *ret.entry(b).or_insert(1) -= 1;
        ret
    }
}

pub fn part2(input: String) -> String {
    let (poly, transitions) = parse(&input);
    let counts = poly
        .chars()
        .tuple_windows::<(_, _, _)>()
        .map(|tup| run_poly(tup, 40, &transitions))
        .fold(HashMap::new(), |m, n| add_counting_sets(m, n));
    format!(
        "{}",
        counts.values().max().unwrap() - counts.values().min().unwrap()
    )
}
