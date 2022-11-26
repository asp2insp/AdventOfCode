use itertools::Itertools;
use crate::utils::*;

#[derive(Debug)]
struct Target {
    minx: isize,
    maxx: isize,
    miny: isize,
    maxy: isize,
}

fn hits((mut vx, mut vy): (isize, isize), t: &Target) -> Option<isize> {
    let mut x = 0;
    let mut y = 0;
    let mut style = 0;
    while x < t.maxx && y > t.miny {
        x += vx;
        y += vy;
        if vx > 0 {
            vx -= 1;
        } else if vx < 0 {
            vx += 1;
        } else {
            vx = 0;
        }
        vy -= 1;
        style = style.max(y);

        if x >= t.minx && x <= t.maxx && y >= t.miny && y <= t.maxy {
            return Some(style);
        }
    }
    None
}

pub fn part1(input: String) -> String {
    let target = Target {
        minx: 257,
        maxx: 286,
        miny: -101,
        maxy: -57,
    };
    (-10..target.maxx)
        .cartesian_product(0..1200)
        .filter_map(|t| hits(t, &target))
        .max()
        .unwrap()
        .to_string()
}

pub fn part2(input: String) -> String {
    let target = Target {
        minx: 257,
        maxx: 286,
        miny: -101,
        maxy: -57,
    };
    (-1..target.maxx + 1)
        .cartesian_product(-200..1500)
        .filter_map(|t| hits(t, &target))
        .counting_set()
		.values()
		.sum::<usize>()
        .to_string()
}

#[test]
fn test() {
    let t = Target {
        minx: 20,
        maxx: 30,
        miny: -10,
        maxy: -5,
    };
    assert_eq!(Some(45), hits((6, 9), &t));
    assert_eq!(None, hits((17, -4), &t));
    assert_eq!(Some(0), hits((9, 0), &t));
}
