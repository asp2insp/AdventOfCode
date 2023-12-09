use aoc::utils::*;
use itertools::Itertools;

fn parse(s: &str) -> Vec<(Direction, isize)> {
    s.lines()
        .map(|l| {
            // replace with .tuple_apply
            let parts = l.split_whitespace().collect_vec();
            (
                match parts[0] {
                    "U" => Direction::N,
                    "D" => Direction::S,
                    "L" => Direction::W,
                    "R" => Direction::E,
                    _ => unimplemented!(),
                },
                parts[1].parse::<isize>().unwrap(),
            )
        })
        .collect_vec()
}

pub fn part1(input: String) -> String {
    let mut g = Grid::new_with_bounds(0, 0, 1000, 1000, |_| ('.', ()));
    let mut h = Point::new(500, 500);
    let mut t = h;
    g.set(t, '#', ());
    for (d, n) in parse(&input) {
        for _ in 0..n {
            // Move Head
            h = g.drive(h, d).unwrap();
            if t.x.abs_diff(h.x) <= 1 && t.y.abs_diff(h.y) <= 1 {
                continue;
            }
            // Move Tail
            t = t.offset(((h.x - t.x).clamp(-1, 1), (h.y - t.y).clamp(-1, 1)));
            g.set(t, '#', ());
        }
    }
    g.iter_chars().filter(|c| c.1 == '#').count().to_string()
}

pub fn part2(input: String) -> String {
    let mut g = Grid::new_with_bounds(0, 0, 1000, 1000, |_| ('.', ()));
    let mut snek = vec![Point::new(500, 500); 10];
    g.set(snek[9], '#', ());
    for (d, n) in parse(&input) {
        for _ in 0..n {
            // Move Head
            snek[0] = g.drive(snek[0], d).unwrap();
            for (h, t) in (0..10).tuple_windows() {
                if snek[t].x.abs_diff(snek[h].x) <= 1 && snek[t].y.abs_diff(snek[h].y) <= 1 {
                    continue;
                }
                // Move Tail
                snek[t] = snek[t].offset((
                    (snek[h].x - snek[t].x).clamp(-1, 1),
                    (snek[h].y - snek[t].y).clamp(-1, 1),
                ));
            }
            g.set(snek[9], '#', ());
        }
    }
    g.iter_chars().filter(|c| c.1 == '#').count().to_string()
}

#[test]
fn test_1() {
    let s = r#"R 4
	U 4
	L 3
	D 1
	R 4
	D 1
	L 5
	R 2"#
        .to_owned();
    assert_eq!("13", part1(s));
}

#[test]
fn test_2() {
    let s = r#"R 5
	U 8
	L 8
	D 3
	R 17
	D 10
	L 25
	U 20"#
        .to_owned();
    assert_eq!("36", part2(s));
}
