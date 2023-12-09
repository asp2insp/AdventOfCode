use aoc::utils::*;
use fnv::FnvHashSet;
use itertools::Itertools;

fn parse(s: &str) -> Vec<(Point, Point)> {
    gimme_nums(s)
        .into_iter()
        .map(|v| (Point::new(v[0], v[1]), Point::new(v[2], v[3])))
        .collect_vec()
}

pub fn part1(input: String) -> String {
    let s2p = parse(&input);
    let mut ret = FnvHashSet::default();
    let y = 2000000;
    for (s, b) in &s2p {
        let radius = s.dist(b);
        for x in (s.x - radius)..=(s.x + radius) {
            let test = Point::new(x, y);
            if s.dist(&test) <= radius && test != *b {
                ret.insert(test);
            }
        }
    }
    ret.len().to_string()
}

pub fn part2(input: String) -> String {
    let s2p = parse(&input);
    let mut cand = FnvHashSet::default();
    let mut seen = FnvHashSet::default();
    let limit = 4000000;
    for (s, b) in &s2p {
        let radius = s.dist(b);
        let mut t = Point::new(s.x - radius, s.y);
        // println!("Processing {:?}", s);

        // Left /^ to Top with */ on left
        while t != Point::new(s.x, s.y + radius) {
            let np = t.offset((1, 1));
            let nc = np.offset((-1, 0));
            if !seen.contains(&np) && !seen.contains(&nc) && in_limit(t, limit) {
                cand.insert(nc);
                seen.insert(nc);
            }
            cand.remove(&np);
            t = np;
        }

        // Top \v to Right with \* on top
        while t != Point::new(s.x + radius, s.y) {
            let np = t.offset((1, -1));
            let nc = np.offset((0, 1));
            if !seen.contains(&np) && !seen.contains(&nc) && in_limit(t, limit) {
                cand.insert(nc);
                seen.insert(nc);
            }
            cand.remove(&np);
            t = np;
        }

        // Right v/ to Bot with /. on right
        while t != Point::new(s.x, s.y - radius) {
            let np = t.offset((-1, -1));
            let nc = np.offset((1, 0));
            if !seen.contains(&np) && !seen.contains(&nc) && in_limit(t, limit) {
                cand.insert(nc);
                seen.insert(nc);
            }
            cand.remove(&np);
            t = np;
        }

        // Bot ^\ to Left with .\ on bot
        while t != Point::new(s.x - radius, s.y) {
            let np = t.offset((-1, 1));
            let nc = np.offset((0, -1));
            if !seen.contains(&np) && !seen.contains(&nc) && in_limit(t, limit) {
                cand.insert(nc);
                seen.insert(nc);
            }
            cand.remove(&np);
            t = np;
        }
    }

    cand.retain(|p| s2p.iter().all(|(s, b)| s.dist(b) < s.dist(p)));
    let answer = cand.into_iter().next().unwrap();
    (answer.x * limit + answer.y).to_string()
}

fn in_limit(p: Point, limit: isize) -> bool {
    p.x >= 0 && p.x <= limit && p.y >= 0 && p.y <= limit
}

#[test]
fn test1() {
    let s = r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
	Sensor at x=9, y=16: closest beacon is at x=10, y=16
	Sensor at x=13, y=2: closest beacon is at x=15, y=3
	Sensor at x=12, y=14: closest beacon is at x=10, y=16
	Sensor at x=10, y=20: closest beacon is at x=10, y=16
	Sensor at x=14, y=17: closest beacon is at x=10, y=16
	Sensor at x=8, y=7: closest beacon is at x=2, y=10
	Sensor at x=2, y=0: closest beacon is at x=2, y=10
	Sensor at x=0, y=11: closest beacon is at x=2, y=10
	Sensor at x=20, y=14: closest beacon is at x=25, y=17
	Sensor at x=17, y=20: closest beacon is at x=21, y=22
	Sensor at x=16, y=7: closest beacon is at x=15, y=3
	Sensor at x=14, y=3: closest beacon is at x=15, y=3
	Sensor at x=20, y=1: closest beacon is at x=15, y=3"#
        .to_owned();
    assert_eq!("26", part1(s));
}
