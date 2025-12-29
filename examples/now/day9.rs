use std::collections::{HashMap, HashSet, VecDeque};

use aoc::utils::*;
use itertools::Itertools;

fn area(p1: Point, p2: Point) -> usize {
    (p1.x.abs_diff(p2.x) + 1) * (p1.y.abs_diff(p2.y) + 1)
}

pub fn part1(input: String) -> String {
    gimme_nums(&input)
        .into_iter()
        .map(|t| Point::new(t[0], t[1]))
        .combinations(2)
        .map(|t| area(t[0], t[1]))
        .max()
        .unwrap()
        .to_string()
}

fn is_rect_inside_bounds(
    c1: Point,
    c2: Point,
    x_bounds_for_y: &HashMap<isize, (isize, isize)>,
    y_bounds_for_x: &HashMap<isize, (isize, isize)>,
) -> bool {
    let xrange = c1.x.min(c2.x)..=c1.x.max(c2.x);
    let yrange = c1.y.min(c2.y)..=c1.y.max(c2.y);
    for x in xrange {
        let bounds = y_bounds_for_x.get(&x).unwrap();
        if c1.y > bounds.1 || c1.y < bounds.0 || c2.y > bounds.1 || c2.y < bounds.0 {
            if bounds.1 - bounds.0 > 20_000 {
                println!(
                    "Y Rejecting {:?}, {:?} since {} !< {}/{} !< {}",
                    c1, c2, bounds.0, c1.y, c2.y, bounds.1
                );
            }
            return false;
        }
    }
    for y in yrange {
        let bounds = x_bounds_for_y.get(&y).unwrap();
        if c1.x > bounds.1 || c1.x < bounds.0 || c2.x > bounds.1 || c2.x < bounds.0 {
            if bounds.1 - bounds.0 > 20_000 {
                println!(
                    "X: Rejecting {:?}, {:?} since {} !< {}/{} !< {}",
                    c1, c2, bounds.0, c1.x, c2.x, bounds.1
                );
            }
            return false;
        }
    }
    true
}

fn check_edge(pts: impl Iterator<Item = Point>, set: &HashSet<Point>) -> bool {
    for p in pts {
        if set.contains(&p) {
            return false;
        }
    }
    true
}

fn flood_fill(set: &HashSet<Point>) -> HashSet<Point> {
    let mut q: VecDeque<Point> = VecDeque::new();
    let mut result = HashSet::with_capacity(5000_000);
    let centroid = Point::new(50_000, 50_000);
    q.extend([
        Point::new(13_000, 13_000),
        Point::new(13_000, 86_000),
        Point::new(85_000, 85_000),
        Point::new(86_000, 13_000),
    ]);
    while let Some(p) = q.pop_front() {
        for np in DIRECTIONS.iter().map(|d| p.offset_dir(*d)) {
            if !set.contains(&np) && !result.contains(&np) && np.real_dist(&centroid) < 51_000f64 {
                result.insert(np);
                q.push_back(np);
            }
        }
    }
    result
}

pub fn part2(input: String) -> String {
    let mut hull = gimme_nums(&input)
        .into_iter()
        .map(|i| Point::new(i[0], i[1]))
        .collect_vec();
    let start = hull[0];
    hull.push(start);
    let mut set = HashSet::with_capacity(hull.len() * 10);
    let mut outside = HashSet::with_capacity(hull.len() * 10);
    for (a, b) in hull.iter().tuple_windows() {
        set.insert(*a);
        set.insert(*b);
        if a.x == b.x {
            let offset = if b.y > a.y {(1, 0)} else {(-1, 0)};
            for y in a.yrange(&b) {
                set.insert(Point::new(a.x, y));
                outside.insert(Point::new(a.x, y).offset(offset));
            }
        } else if a.y == b.y {
            let offset = if b.x > a.x {(0, -1)} else {(0, 1)};
            for x in a.xrange(&b) {
                set.insert(Point::new(x, a.y));
                outside.insert(Point::new(x, a.y).offset(offset));
            }
        }
    }
    // for pt in &outside {
    //     println!("Out {},{}", pt.x, pt.y);
    // }
    println!("Finished walking border");

    let a = Point::new(94901, 50265);
    let b = Point::new(94901, 48488);
    let mut max_area_a = 0;
    let mut max_area_b = 0;
    for h in hull {
        if h.y >= a.y {
            let cand = area(a, h);
            if cand <= max_area_a {
                continue;
            }
            if !check_edge(a.yrange(&h).map(|y| Point::new(a.x, y)), &outside) {
                continue;
            }
            if !check_edge(a.yrange(&h).map(|y| Point::new(h.x, y)), &outside) {
                continue;
            }
            println!("Cand {},{} {},{}", a.x, a.y, h.x, h.y);
            max_area_a = cand;
        } else if h.y <= b.y {
            let cand = area(b, h);
            if cand <= max_area_b {
                continue;
            }
            if !check_edge(b.yrange(&h).map(|y| Point::new(b.x, y)), &outside) {
                continue;
            }
            if !check_edge(b.yrange(&h).map(|y| Point::new(h.x, y)), &outside) {
                continue;
            }
            // println!("Cand {},{} {},{}", b.x, b.y, h.x, h.y);
            max_area_b = cand;
        }
    }
    max_area_b.max(max_area_a).to_string()
}

pub fn part2_arbitrary(input: String) -> String {
    // Build the full set
    let mut hull = gimme_nums(&input)
        .into_iter()
        .map(|i| Point::new(i[0], i[1]))
        .collect_vec();
    let start = hull[0];
    hull.push(start);
    let mut set = HashSet::with_capacity(hull.len() * 10);
    for (a, b) in hull.iter().tuple_windows() {
        set.insert(*a);
        set.insert(*b);
        if a.x == b.x {
            for y in a.yrange(&b) {
                set.insert(Point::new(a.x, y));
            }
        } else if a.y == b.y {
            for x in a.xrange(&b) {
                set.insert(Point::new(x, a.y));
            }
        }
    }
    let midpoint_x = hull.iter().map(|p| p.x).sum::<isize>() / hull.len() as isize;
    let midpoint_y = hull.iter().map(|p| p.y).sum::<isize>() / hull.len() as isize;
    let mut x_bounds_for_y: HashMap<isize, (isize, isize)> = HashMap::new();
    let mut y_bounds_for_x: HashMap<isize, (isize, isize)> = HashMap::new();
    for p in &set {
        // Update y bounds
        let xbfy = x_bounds_for_y.entry(p.y).or_insert((0, 100_000));
        if p.x < midpoint_x && p.x > xbfy.0 {
            xbfy.0 = p.x;
        }
        if p.x >= midpoint_x && p.x < xbfy.1 {
            xbfy.1 = p.x
        }

        // Update x bounds
        let ybfx = y_bounds_for_x.entry(p.x).or_insert((0, 100_000));
        if p.y < midpoint_y && p.y > ybfx.0 {
            ybfx.0 = p.y;
        }
        if p.y >= midpoint_y && p.y < ybfx.1 {
            ybfx.1 = p.y;
        }
    }
    let mut max_area = 0;
    for t in hull.into_iter().combinations(2).map(|t| (t[0], t[1])) {
        if t.0.x.abs_diff(t.1.x) < 20_000 || t.0.y.abs_diff(t.1.y) < 20_000 {
            // Don't look at ones that are too close together
            continue;
        }
        let a = area(t.0, t.1);
        if a <= max_area {
            continue;
        }
        if is_rect_inside_bounds(t.0, t.1, &x_bounds_for_y, &y_bounds_for_x) {
            println!("{:?} is good! New max {}", t, a);
            max_area = a;
        }
    }
    max_area.to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_area() {
        assert_eq!(6, area(Point::new(2, 3), Point::new(7, 3)));
        assert_eq!(35, area(Point::new(7, 1), Point::new(11, 7)));
        assert_eq!(24, area(Point::new(2, 5), Point::new(9, 7)));
    }
}
