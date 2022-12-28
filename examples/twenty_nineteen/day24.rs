
use aoc::utils::*;
use aoc::*;
use itertools::Itertools;

pub fn part1(s: String) -> String {
    let mut g = Grid::new(&s, ());
    let mut seen = makeset!();
    loop {
        if seen.contains(&g.to_string()) {
            return g
                .iter_chars()
                .filter(|(p, c)| *c == '#')
                .map(|(p, _c)| {
                    let score = 2_usize.pow(((4 - p.y) * 5 + p.x) as u32);
                    // println!("{:?} scores {}", p, score);
                    score
                })
                .sum::<usize>()
                .to_string();
        }
        seen.insert(g.to_string());
        let mut g2 = g.clone();
        for pt in g.all_pts() {
            let adj_count = g.neighbors(pt).filter(|p2| g.read_pt(&p2) == '#').count();
            if g.read_pt(&pt) == '#' && adj_count != 1 {
                g2.set(pt, '.', ());
            } else if g.read_pt(&pt) == '.' && (adj_count == 1 || adj_count == 2) {
                g2.set(pt, '#', ());
            }
        }
        g = g2;
    }
}

fn adj(l: isize, p: Point) -> Vec<(isize, Point)> {
    let n = match (p.x, p.y) {
        // Outer corners
        (0, 0) => vec![(l, 1, 0), (l, 0, 1), (l-1, 1, 2), (l-1, 2, 1)],
        (0, 4) => vec![(l, 0, 3), (l, 1, 4), (l-1, 1, 2), (l-1, 2, 3)],
        (4, 0) => vec![(l, 3, 0), (l, 4, 1), (l-1, 2, 1), (l-1, 3, 2)],
        (4, 4) => vec![(l, 3, 4), (l, 4, 3), (l-1, 2, 3), (l-1, 3, 2)],

        // Bottom Middle
        (1, 0) => vec![(l, 0, 0), (l, 1, 1), (l, 2, 0), (l-1, 2, 1)],
        (2, 0) => vec![(l, 1, 0), (l, 2, 1), (l, 3, 0), (l-1, 2, 1)],
        (3, 0) => vec![(l, 2, 0), (l, 3, 1), (l, 4, 0), (l-1, 2, 1)],

        // Left Middle
        (0, 1) => vec![(l, 0, 0), (l, 0, 2), (l, 1, 1), (l-1, 1, 2)],
        (0, 2) => vec![(l, 0, 1), (l, 0, 3), (l, 1, 2), (l-1, 1, 2)],
        (0, 3) => vec![(l, 0, 2), (l, 0, 4), (l, 1, 3), (l-1, 1, 2)],

        // Right Middle
        (4, 1) => vec![(l, 4, 0), (l, 4, 2), (l, 3, 1), (l-1, 3, 2)],
        (4, 2) => vec![(l, 4, 1), (l, 4, 3), (l, 3, 2), (l-1, 3, 2)],
        (4, 3) => vec![(l, 4, 2), (l, 4, 4), (l, 3, 3), (l-1, 3, 2)],

        // Top Middle
        (1, 4) => vec![(l, 0, 4), (l, 2, 4), (l, 1, 3), (l-1, 2, 3)],
        (2, 4) => vec![(l, 1, 4), (l, 3, 4), (l, 2, 3), (l-1, 2, 3)],
        (3, 4) => vec![(l, 2, 4), (l, 4, 4), (l, 3, 3), (l-1, 2, 3)],

        // Interior Corners
        (1, 1) => vec![(l, 0, 1), (l, 1, 0), (l, 1, 2), (l, 2, 1)],
        (1, 3) => vec![(l, 0, 3), (l, 1, 2), (l, 1, 4), (l, 2, 3)],
        (3, 1) => vec![(l, 3, 0), (l, 3, 2), (l, 2, 1), (l, 4, 1)],
        (3, 3) => vec![(l, 2, 3), (l, 4, 3), (l, 3, 2), (l, 3, 4)],

        // Inner adjacents
        (2, 1) => concat(vec![(l, 1, 1), (l, 2, 0), (l, 3, 1)], (0..5).map(|x| (l+1, x, 0))),
        (2, 3) => concat(vec![(l, 1, 3), (l, 2, 4), (l, 3, 3)], (0..5).map(|x| (l+1, x, 4))),
        (1, 2) => concat(vec![(l, 1, 1), (l, 0, 2), (l, 1, 3)], (0..5).map(|y| (l+1, 0, y))),
        (3, 2) => concat(vec![(l, 3, 1), (l, 3, 3), (l, 4, 2)], (0..5).map(|y| (l+1, 4, y))),

        _ => unreachable!(),
    };
    n.into_iter().map(|(l, x, y)| (l, Point::new(x, y))).collect_vec()
}

pub fn part2(s: String) -> String {
    let g = Grid::new(&s, ());
    let mut blank = g.clone();
    blank.for_each_mut(None, None, |(c, _)| *c = '.');
    let mut worlds = dict!(0isize => g);
    for _ in 0..200 {
        let (mut min, mut max) = (*worlds.keys().min().unwrap(), *worlds.keys().max().unwrap());
        if worlds.get(&min).unwrap().iter_chars().any(|(_, c)| c == '#') {
            min -= 1;
            worlds.insert(min, blank.clone());
        }
        if worlds.get(&max).unwrap().iter_chars().any(|(_, c)| c == '#') {
            max += 1;
            worlds.insert(max, blank.clone());
        }
        let mut worlds2 = worlds.clone();
        for i in min..=max {
            let g = worlds.get(&i).unwrap();
            for pt in g.all_pts() {
                if pt == Point::new(2, 2) {
                    continue;
                }
                let adj_count = adj(i, pt).into_iter().filter(|(l, p2)| worlds.get(l).map(|g| g.read_pt(&p2) == '#').unwrap_or(false)).count();
                if g.read_pt(&pt) == '#' && adj_count != 1 {
                    worlds2.get_mut(&i).unwrap().set(pt, '.', ());
                } else if g.read_pt(&pt) == '.' && (adj_count == 1 || adj_count == 2) {
                    worlds2.get_mut(&i).unwrap().set(pt, '#', ());
                }
            }
        }
        worlds = worlds2;
    }
    // for (k,v) in &worlds {
    //     println!("{}\n{}\n", k, v.to_string());
    // }
    worlds.values().flat_map(|g| g.iter_chars()).filter(|(p, c)| *c == '#').count().to_string()
}

#[test]
fn test() {
    let s =
 r#"....#
    #..#.
    #..##
    ..#..
    #...."#
        .to_owned();
    assert_eq!("2129920", part1(s.clone()));
    assert_eq!("99", part2(s));
}
