use std::collections::{HashMap, HashSet};

use aoc::utils::*;
use itertools::Itertools;

fn drive_beam(pt: Point, d: Direction, g: &Grid<()>) -> Vec<(Point, Direction)> {
    use Direction::*;
    if let Some((np, c)) = g.drive(pt, d).and_then(|np| g.get(np).map(|v| (np, v.0))) {
        return match (d, c) {
            (N, '-') | (S, '-') => vec![(np, W), (np, E)],
            (E, '|') | (W, '|') => vec![(np, N), (np, S)],
            (N, '/') => vec![(np, E)],
            (N, '\\') => vec![(np, W)],
            (S, '/') => vec![(np, W)],
            (S, '\\') => vec![(np, E)],
            (E, '/') => vec![(np, N)],
            (E, '\\') => vec![(np, S)],
            (W, '/') => vec![(np, S)],
            (W, '\\') => vec![(np, N)],
            _ => vec![(np, d)],
        };
    }
    vec![]
}

fn get_score(init: Vec<(Point, Direction)>, g: &Grid<()>) -> usize {
    let mut states = HashSet::new();
    let mut pts = init;
    while pts.len() > 0 {
        // println!("{:?}", pts);
        pts = pts
            .into_iter()
            .flat_map(|(pt, d)| drive_beam(pt, d, &g))
            .filter(|s| !states.contains(s))
            .collect();
        pts.iter().for_each(|s: &(Point, Direction)| {
            states.insert(*s);
        });

        // let mut g2 = g.clone();
        // pts.iter().for_each(|pt| { g2.set(pt.0, '#', ()); });
        // println!("{}\n", g2.to_string());
    }
    states.into_iter().map(|(pt, _)| pt).unique().count()
}

pub fn part1(input: String) -> String {
    let g = Grid::new(&input, ());
    get_score(vec![(Point::new(-1, g.top_bound), Direction::E)], &g).to_string()
}

pub fn part2(input: String) -> String {
    let g = Grid::new(&input, ());
    (g.left_bound..=g.right_bound)
        .map(|x| (Point::new(x, g.top_bound + 1), Direction::S))
        .chain(
            (g.left_bound..=g.right_bound)
                .map(|x| (Point::new(x, g.bottom_bound - 1), Direction::N)),
        )
        .chain(
            (g.bottom_bound..=g.top_bound).map(|y| (Point::new(g.left_bound - 1, y), Direction::E)),
        )
        .chain(
            (g.bottom_bound..=g.top_bound)
                .map(|y| (Point::new(g.right_bound + 1, y), Direction::W)),
        )
        .map(|s| get_score(vec![s], &g))
        .max()
        .unwrap()
        .to_string()
}

#[test]
fn test() {
    let i = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
    assert_eq!(part1(i.to_string()), "46");
}
