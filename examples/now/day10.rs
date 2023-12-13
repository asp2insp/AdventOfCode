use std::{collections::HashSet, sync::atomic::{Ordering, AtomicUsize}};

use aoc::utils::*;
use itertools::Itertools;
use num::Integer;

fn turn(p: Point, d: Direction, g: &Grid<()>) -> Direction {
    use Direction::*;
    match (g.read_pt(&p), d) {
        ('-', x) if x == E || x == W => x,
        ('|', x) if x == N || x == S => x,
        ('L', S) => E,
        ('L', W) => N,
        ('J', E) => N,
        ('J', S) => W,
        ('7', E) => S,
        ('7', N) => W,
        ('F', N) => E,
        ('F', W) => S,
        ('S', x) => x,
        (a, _) => panic!("Invalid turn at {:?} {} {:?}", p, a, d),
    }
}

fn can_connect(dest: Point, d: Direction, g: &Grid<()>) -> bool {
    use Direction::*;
    match (g.read_pt(&dest), d) {
        ('-', x) if x == E || x == W => true,
        ('|', x) if x == N || x == S => true,
        ('L', x) if x == S || x == W => true,
        ('J', x) if x == E || x == S => true,
        ('7', x) if x == E || x == N => true,
        ('F', x) if x == N || x == W => true,
        _ => false,
    }
}

fn drive_and_turn((d, p): (Direction, Point), g: &Grid<()>) -> (Direction, Point) {
    let np = g.drive(p, d).unwrap();
    (turn(np, d, g), np)
}

pub fn part1(input: String) -> String {
    let g = Grid::new(&input, ());
    let start = g.find('S').unwrap();
    let mut count = 0;
    let (mut l, mut r) = g
        .neighbors_with_directions(start)
        .filter(|p| can_connect(p.1, p.0, &g))
        .map(|(d, p)| (turn(p, d, &g), p))
        .collect_tuple::<(_, _)>()
        .unwrap();

    let mut seen = HashSet::new();

    while !seen.contains(&l.1) && !seen.contains(&r.1) {
        count += 1;
        seen.insert(l.1);
        seen.insert(r.1);
        l = drive_and_turn(l, &g);
        r = drive_and_turn(r, &g);
    }
    count.to_string()
}

pub fn part2(input: String) -> String {
    let mut g = Grid::new(&input, ());
    let start = g.find('S').unwrap();
    let (mut l, mut r) = g
        .neighbors_with_directions(start)
        .filter(|p| can_connect(p.1, p.0, &g))
        .map(|(d, p)| (turn(p, d, &g), p))
        .collect_tuple::<(_, _)>()
        .unwrap();

    let mut seen = HashSet::new();
	seen.insert(start);
    while !seen.contains(&l.1) || !seen.contains(&r.1) {
        seen.insert(l.1);
        seen.insert(r.1);
        l = drive_and_turn(l, &g);
        r = drive_and_turn(r, &g);
    }
    let gc = g.clone();
    let mut count = AtomicUsize::new(0);
    g.for_each_mut(None, None, |p, (c, v)| {
        if seen.contains(&p) {
            return;
        }
        let winding_num = gc
            .drive_iter(p, Direction::W)
            .filter(|p| seen.contains(p))
            .map(|p| match gc.read_pt(&p) {
                '-' => 0,
                '|' => 2,
                'L' => -1,
                '7' => -1,
                'F' => 1,
                'J' => 1,
                _ => 0,
            })
            .sum::<i32>();
        if (winding_num / 2).is_odd() {
            *c = 'I';//winding_num.to_string().chars().next().unwrap();
			count.fetch_add(1, Ordering::SeqCst);
        }
    });
    println!("{}", g.to_string());
    count.to_debug_string()
}

const INPUT: &str = r"..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

const I2: &str = r".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

#[test]
fn test_part1() {
    assert_eq!(part1(INPUT.to_string()), "8");
}

const I3: &str = r"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

const I4: &str = r"..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
..........";

#[test]
fn test_part2() {
    assert_eq!(part2(I3.to_string()), "4");
    assert_eq!(part2(I4.to_string()), "4");
	assert_eq!(part2(I2.to_string()), "8");

}
