use crate::utils::*;
use itertools::*;
use std::collections::HashMap;
use rayon::prelude::*;

fn parse(s: &str) -> (&str, HashMap<(char, char), char>) {
    let mut l = s.lines();
    let poly = l.next().unwrap();
    (
        poly,
        l.skip(1)
            .flat_map(|s| s.trim().split(" -> ").collect_tuple::<(_, _)>())
            .map(|(l, r)| {
                (
                    l.chars().collect_tuple::<(_, _)>().unwrap(),
                    r.chars().next().unwrap(),
                )
            })
            .collect::<HashMap<_, _>>(),
    )
}

pub fn part1(input: String) -> String {
    let (poly, transitions) = parse(&input);
    let counts = poly
        .chars()
        .tuple_windows::<(_, _)>()
        .map(|tup| run_poly(tup, 10, &transitions, &mut dict![]))
        .fold(HashMap::new(), |m, n| add_counting_sets(m, n));
    format!(
        "{}",
        counts.values().max().unwrap() - counts.values().min().unwrap() + 1
    )
}

fn run_poly(
    (a, b): (char, char),
    n: usize,
    t: &HashMap<(char, char), char>,
    memo: &mut HashMap<(char, char, usize), HashMap<char, usize>>
) -> HashMap<char, usize> {
    if n == 0 {
        [a, b].into_iter().counting_set()
    } else if let Some(res) = memo.get(&(a, b, n)) {
        res.clone()
    } else {
        let m = t[&(a, b)];
        let left = (a, m);
        let right = (m, b);
        let mut ret = add_counting_sets(run_poly(left, n - 1, t, memo), run_poly(right, n - 1, t, memo));
        *ret.entry(m).or_insert(1) -= 1;
        memo.insert((a, b, n), ret.clone());
        ret
    }
}

pub fn part2(input: String) -> String {
    let (poly, transitions) = parse(&input);
    let counts = poly
        .chars()
        .tuple_windows::<(_, _)>()
        .map(|tup| run_poly(tup, 40, &transitions, &mut dict![]))
        .fold(HashMap::new(), |m, n| add_counting_sets(m, n));
    format!(
        "{}",
        counts.values().max().unwrap() - counts.values().min().unwrap() + 1
    )
}


#[test]
fn test() {
    let s = r"NNCB

    CH -> B
    HH -> N
    CB -> H
    NH -> C
    HB -> C
    HC -> B
    HN -> C
    NN -> C
    BH -> H
    NC -> B
    NB -> B
    BN -> B
    BB -> N
    BC -> B
    CC -> N
    CN -> C";

    assert_eq!("1588", part1(s.to_string()));
    assert_eq!("2188189693529", part2(s.to_string()));
}