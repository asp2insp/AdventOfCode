use crate::utils::*;
use itertools::Itertools;
use std::collections::HashMap;

pub fn part1(input: String) -> String {
    let g = Grid::new(&input, ());
    let (l, b, r, t) = g.get_bounds();
    format!(
        "{:?}",
        g.bfs_generic(
            makeset!(Point::new(l, t)),
            Some(&|p| g
                .neighbors(p)
                .map(|n| {
                    (n, g.get(n).unwrap().0.to_digit(10).unwrap() as isize)
                })
				.collect_vec()),
            None,
        )[&Point::new(r,b)].0
    )
}

fn counting_kinda_mod_add(a: u32, b: u32, n: u32) -> u32 {
    let mut res = a;
    for i in 0..b {
        res += 1;
        if res == n {
            res = 1;
        }
    }
    res
}

pub fn part2(input: String) -> String {
    let mut g = Grid::new(&input, 0);
    g.for_each_mut(None, None, |(c, i)| {
        *i = c.to_digit(10).unwrap();
    });

    let cp = g.clone();
    for coord in 1..5 {
        let mut cp2 = cp.clone();
        cp2.for_each_mut(None, None, |(c, i)| {
            *i = counting_kinda_mod_add(*i, coord, 10);
            *c = char::from_digit(*i, 10).unwrap();
        });
        g.add_other(&cp2, Direction::E);
    }
    let cp = g.clone();
    for coord in 1..5 {
        let mut cp2 = cp.clone();
        cp2.for_each_mut(None, None, |(c, i)| {
            *i = counting_kinda_mod_add(*i, coord, 10);
            *c = char::from_digit(*i, 10).unwrap();
        });
        g.add_other(&cp2, Direction::S);
    }

    let (l, b, r, t) = g.get_bounds();
    // println!("{:?}", (l,b,r,t));
    // println!("{}", g.to_string());
    format!(
        "{:?}",
        g.bfs_generic(
            makeset!(Point::new(l, t)),
            Some(&|p| g
                .neighbors(p)
                .map(|n| {
                    (n, g.get(n).unwrap().0.to_digit(10).unwrap() as isize)
                })
				.collect_vec()),
            None
        )[&Point::new(r,b)].0
    )
}

#[test]
fn test() {
    let s = r"1163751742
	1381373672
	2136511328
	3694931569
	7463417111
	1319128137
	1359912421
	3125421639
	1293138521
	2311944581";

    assert_eq!("40", part1(s.to_string()));
    assert_eq!("315", part2(s.to_string()));
}

#[test]
fn test2() {
    let s = r"19999
    19111
    11191";

    assert_eq!("8", part1(s.to_string()));
}