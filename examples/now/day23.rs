use aoc::{dict, makeset, utils::*};
use itertools::Itertools;
use std::cmp::{max, min};
use Direction::*;

fn run_round(g: &mut Grid<()>, start: usize) {
    let positions = g
        .iter_chars()
        .filter_map(|(p, c)| if c == '#' { Some(p) } else { None })
        .collect_vec();
    let mut proposals = dict!();
    let mut stays = makeset!();
    for pos in positions.clone() {
        if g.neighbors_with_diagonals(pos).count() == 8 {
            stays.insert(pos);
            continue;
        }
        let mut proposed_next = pos;
        for j in start..(start + 4) {
            let attempt = match j % 4 {
                0 => (g.drive2(pos, N, E), g.drive(pos, N), g.drive2(pos, N, W)),
                1 => (g.drive2(pos, S, E), g.drive(pos, S), g.drive2(pos, S, W)),
                2 => (g.drive2(pos, W, N), g.drive(pos, W), g.drive2(pos, W, S)),
                3 => (g.drive2(pos, E, S), g.drive(pos, E), g.drive2(pos, E, N)),
                _ => unreachable!(),
            };
            if let (Some(_), Some(p2), Some(_)) = attempt {
                proposed_next = p2;
                break;
            }
        }
        if proposed_next != pos {
            if !proposals.contains_key(&proposed_next) {
                proposals.insert(proposed_next, pos);
            } else {
                stays.insert(pos);
                stays.insert(proposals.remove(&proposed_next).unwrap());
            }
        } else {
            stays.insert(pos);
        }
    }
    for p in positions {
        g.set(p, '.', ());
    }
    for p in proposals.keys() {
        g.set(*p, '#', ());
    }
    for p in stays {
        g.set(p, '#', ());
    }
}

pub fn part1(input: String) -> String {
    let mut g = Grid::new(&input, ());
    g.clear_bounds();
    g.wall_char = '#';
    let rounds = 10;
    let mut start = 0;
    for _ in 0..rounds {
        run_round(&mut g, start);
        start += 1;
    }
    let mut left = isize::MAX;
    let mut bottom = isize::MAX;
    let mut right = isize::MIN;
    let mut top = isize::MIN;

    for (Point { x, y }, c) in g.iter_chars() {
        if c != '#' {
            continue;
        }
        left = min(left, x);
        right = max(right, x);
        bottom = min(bottom, y);
        top = max(top, y);
    }
    ((top - bottom + 1) * (right - left + 1)
        - g.iter_range(Some(left..=right), Some(bottom..=top))
            .filter(|(_, c, _)| *c == '#')
            .count() as isize)
        .to_string()
}

pub fn part2(input: String) -> String {
    let mut g = Grid::new(&input, ());
    g.clear_bounds();
    g.wall_char = '#';
    let mut start = 0;
    let mut prev = makeset!();
    loop {
        run_round(&mut g, start);
        let ident = g
            .iter_chars()
            .filter_map(|(p, c)| if c == '#' { Some(p) } else { None })
            .collect::<_>();
        start += 1;
        if ident == prev {
            return start.to_string();
        }
        prev = ident;
    }
}

#[test]
fn test1() {
    let s = r#"....#..
	..###.#
	#...#.#
	.#...##
	#.###..
	##.#.##
	.#..#..
	"#
    .to_owned();
    assert_eq!("110", part1(s.clone()));
    assert_eq!("20", part2(s));
}
