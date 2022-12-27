use crate::intcode::*;
use aoc::{utils::*, *};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn parse(s: &str) -> (Grid<Option<Point>>, Point, Point) {
    let mut g = Grid::new_preserving_whitespace(s, |_| None::<Point>).with_wall('#');
    let portals: HashMap<Point, String> = g
        .iter_chars()
        .filter_map(|(p, c)| {
            if c.is_ascii_alphabetic() && c.is_ascii_uppercase() {
                g.neighbors(p)
                    .find(|px| g.get(*px).map(|(c, _)| *c == '.').unwrap_or(false))
                    .and_then(|p2| {
                        g.drive(p, p.dir_rel_to(&p2)).map(|px| {
                            let c1 = g.read_pt(&px);
                            let c2 = g.read_pt(&p);
                            if px.x < p.x || p.y < px.y {
                                (p2, format!("{}{}", c1, c2))
                            } else {
                                (p2, format!("{}{}", c2, c1))
                            }
                        })
                    })
            } else {
                None
            }
        })
        .collect();
    let mut start = None;
    let mut end = None;
    for (p, s) in &portals {
        for (p2, s2) in &portals {
            if s == s2 && p != p2 {
                g.set(*p, '.', Some(*p2));
                g.set(*p2, '.', Some(*p));
            }
        }
        if s == "AA" {
            start = Some(*p);
        } else if s == "ZZ" {
            end = Some(*p);
        }
    }
    let start = start.unwrap();
    let end = end.unwrap();
    (g, start, end)
}

pub fn part1(s: String) -> String {
    let (g, start, end) = parse(&s);

    let ret = g.bfs_generic(
        makeset!(start),
        Some(&|p| {
            let mut v = g.neighbors(p).collect_vec();
            if let Some((_, Some(warp))) = g.get(p) {
                v.push(*warp);
            };
            v.into_iter()
                .filter(|d| g.read_pt(d) == '.')
                .map(|d| (d, 1))
                .collect_vec()
        }),
        Some(&|f| f.contains_key(&end)),
    );
    ret.get(&end).unwrap().0.to_string()
}

pub fn part2(s: String) -> String {
    let (g, start, end) = parse(&s);
    let mut q = vec![(start, 0, 0)];
    let mut best = usize::MAX;
    let outers = g
        .iter()
        .filter_map(|(p, _, w)| {
            if w.is_some()
                && (p.x.abs_diff(g.left_bound) <= 3
                    || p.x.abs_diff(g.right_bound) <= 3
                    || p.y.abs_diff(g.top_bound) <= 3
                    || p.y.abs_diff(g.bottom_bound) <= 3)
            {
                Some(p)
            } else {
                None
            }
        })
        .collect::<HashSet<Point>>();
    let mut seen = dict!();
    while let Some((np, nl, c)) = q.pop() {
        if nl < 0 {
            continue
        }
        if np == end && nl == 0 {
            best = best.min(c);
            continue
        }
        if c >= best {
            continue
        }
        if let Some(&c2) = seen.get(&(np, nl)) {
            if c >= c2 {
                continue
            }
        }
        seen.insert((np, nl), c);

        if c > *seen.entry((np, nl)).or_insert(c) {
            continue
        }
        // println!("{:?}", (np, nl, c));
        g.neighbors(np).filter(|d| g.read_pt(d) == '.').for_each(|d| q.push((d, nl, c+1)));
        if let Some((_, Some(warp))) = g.get(np) {
            q.push((*warp, if outers.contains(&np) {nl-1} else {nl+1}, c+1));
        }
    }
    best.to_string()
}
