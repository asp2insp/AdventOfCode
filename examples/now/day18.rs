use std::collections::VecDeque;

use aoc::{
    makeset,
    utils::{gimme_usizes_once, P3},
};
use fnv::FnvHashSet;
use itertools::{Itertools, MinMaxResult};

fn parse(s: &str) -> Vec<P3> {
    s.lines()
        .map(|l| {
            let v = gimme_usizes_once(l);
            P3::new(v[0] as isize, v[1] as isize, v[2] as isize)
        })
        .collect_vec()
}

pub fn part1(input: String) -> String {
    let pts: FnvHashSet<_> = parse(&input).into_iter().collect();
    pts.iter()
        .map(|p| {
            p.neighbors()
                .into_iter()
                .filter(|p2| !pts.contains(p2))
                .count()
        })
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: String) -> String {
    let pts: FnvHashSet<_> = parse(&input).into_iter().collect();
    let mut steam = makeset!();
    if let (
        MinMaxResult::MinMax(xl, xr),
        MinMaxResult::MinMax(yl, yr),
        MinMaxResult::MinMax(zl, zr),
    ) = (
        pts.iter().map(|p| p.x).minmax(),
        pts.iter().map(|p| p.y).minmax(),
        pts.iter().map(|p| p.z).minmax(),
    ) {
        let bounds = ((xl - 1)..=xr + 1, (yl - 1)..=yr + 1, (zl - 1)..=zr + 1);
        let start = P3::new(xr + 1, yr + 1, zr + 1); // Start outside the region
        let mut q = VecDeque::new();
        q.push_back(start);
        while let Some(n) = q.pop_front() {
            // Discard out of bounds
            if !bounds.0.contains(&n.x) || !bounds.1.contains(&n.y) || !bounds.2.contains(&n.z) {
                continue;
            }
            // Discard already steam
            if steam.contains(&n) {
                continue;
            }
            // Discard lava
            if pts.contains(&n) {
                continue;
            }
            // Add and expand
            q.extend(n.neighbors());
            steam.insert(n);
        }
    }
    pts.iter()
        .map(|p| {
            p.neighbors()
                .into_iter()
                .filter(|p2| steam.contains(p2))
                .count()
        })
        .sum::<usize>()
        .to_string()
}
