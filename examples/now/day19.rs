use cached::proc_macro::cached;
use itertools::Itertools;
use cached::UnboundCache;

fn possible(patterns: &[&str], desired: &str) -> bool {
    if desired.is_empty() {
        return true;
    }
    for p in patterns {
        if desired.starts_with(p) && possible(patterns, &desired[p.len()..]) {
            return true;
        }
    }
    return false;
}

pub fn part1(input: String) -> String {
    let mut l = input.lines();
    let patterns = l.next().unwrap().split(", ").collect_vec();
    l.next().unwrap(); //skip blank line
    l.filter(|line| possible(&patterns, line))
        .count()
        .to_string()
}

#[cached(
    ty = "UnboundCache<String,usize>",
    create = "{UnboundCache::new()}",
    convert = r#"{desired.to_string()}"#
)]
fn possible_ways(patterns: &[&str], desired: &str) -> usize {
    if desired.is_empty() {
        return 1;
    }
    let mut acc = 0;
    for p in patterns {
        if desired.starts_with(p) {
            acc += possible_ways(patterns, &desired[p.len()..]);
        }
    }
    return acc;
}

pub fn part2(input: String) -> String {
    let mut l = input.lines();
    let patterns = l.next().unwrap().split(", ").collect_vec();
    l.next().unwrap(); //skip blank line
    l.map(|line| possible_ways(&patterns, line))
        .sum::<usize>()
        .to_string()
}
