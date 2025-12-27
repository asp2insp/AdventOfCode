use std::{
    collections::{HashMap, VecDeque},
    rc::Rc,
};

use aoc::{makeset, utils::*};
use fnv::FnvHashSet;

pub fn part1(input: String) -> String {
    let g = Grid::new(&input, ()).with_wall('#');
    let start = g.find('S').unwrap();
    let end = g.find('E').unwrap();
    let mut q = VecDeque::new();
    q.push_back((start, Direction::E, 0));
    let mut visited = HashMap::new();
    let mut lowest = std::usize::MAX;
    while let Some((pos, dir, steps)) = q.pop_front() {
        if pos == end {
            lowest = lowest.min(steps);
            continue;
        }
        if let Some(amount) = visited.get(&(pos, dir)) {
            if *amount <= steps {
                continue;
            }
        }
        visited.insert((pos, dir), steps);
        if let Some(next) = g.drive(pos, dir) {
            q.push_back((next, dir, steps + 1));
        }
        q.push_back((pos, dir.turn('L'), steps + 1000));
        q.push_back((pos, dir.turn('R'), steps + 1000));
    }
    lowest.to_string()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PL {
    pos: Point,
    next: Option<Rc<PL>>,
}

impl PL {
    fn new(pos: Point) -> Self {
        Self { pos, next: None }
    }

    fn clone_with(self: &Rc<Self>, pos: Point) -> Rc<Self> {
        Rc::new(Self {
            pos,
            next: Some(self.clone()),
        })
    }
}

pub fn part2(input: String) -> String {
    let target_cost = part1(input.clone()).parse::<usize>().unwrap();
    let g = Grid::new(&input, ()).with_wall('#');
    let start = g.find('S').unwrap();
    let end = g.find('E').unwrap();
    let mut q = VecDeque::new();
    q.push_back((start, Direction::E, 0, Rc::new(PL::new(Point::new(-1, -1)))));
    let mut best_path_to_point: HashMap<(Point, Direction), (usize, Vec<Rc<PL>>)> = HashMap::new();
    while let Some((pos, dir, steps, trail)) = q.pop_front() {
        if steps > target_cost {
            continue;
        }
        if !best_path_to_point.contains_key(&(pos, dir)) {
            best_path_to_point.insert((pos, dir), (usize::MAX, Vec::<Rc<PL>>::new()));
        }
        let (point_cost, trails) = best_path_to_point.get_mut(&(pos, dir)).unwrap();
        if steps < *point_cost {
            *point_cost = steps;
            *trails = vec![trail.clone()];
        } else if steps == *point_cost {
            trails.push(trail.clone());
        } else {
            continue;
        }
        if pos == end {
            continue;
        }
        if let Some(next) = g.drive(pos, dir) {
            q.push_front((next, dir, steps + 1, trail.clone_with(pos)));
        }
        if g.drive(pos, dir.turn('L')).is_some() {
            q.push_front((pos, dir.turn('L'), steps + 1000, trail.clone()));
        }
        if g.drive(pos, dir.turn('R')).is_some() {
            q.push_front((pos, dir.turn('R'), steps + 1000, trail.clone()));
        }
    }
    (DIRECTIONS
        .clone()
        .into_iter()
        .flat_map(|d| best_path_to_point.get(&(end, d)))
        .flat_map(|(_, trails)| trails.clone())
        .flat_map(|mut pl| {
            let mut v = vec![pl.pos];
            while let Some(next) = &pl.next {
                pl = next.clone();
                v.push(pl.pos);
            }
            v
        })
        .filter(|&p| p != Point::new(-1, -1))
        .collect::<FnvHashSet<Point>>()
        .len()
        + 1)
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        let input = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;
        assert_eq!(part1(input.to_string()), "7036");
        assert_eq!(part2(input.to_string()), "45");
    }

    #[test]
    fn test_second() {
        let input = r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"#;
        assert_eq!(part1(input.to_string()), "11048");
        assert_eq!(part2(input.to_string()), "64");
    }
}
