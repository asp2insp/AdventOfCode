use std::collections::VecDeque;

use aoc::{dict, makeset, utils::*};
use fnv::{FnvHashMap, FnvHashSet};

pub fn part1(input: String) -> String {
    let g = Grid::new(&input, ()).with_wall('#');
    let end = g.find('E').unwrap();
    let path = g.bfs_generic(makeset!(end), None, None);
    let mut saves = vec![];
    for (point, (cost, _)) in path.iter() {
        for d in DIRECTIONS {
            let p2 = point.offset_dir(d).offset_dir(d);
            if let Some(dest) = path.get(&p2) {
                if cost.saturating_sub(dest.0) >= 2 {
                    saves.push(cost.saturating_sub(dest.0) - 2);
                }
            }
        }
    }
    // println!("{:?}", saves.into_iter().counting_set());
    let cutoff = if g.width() < 20 { 1 } else { 100 };
    saves
        .into_iter()
        .filter(|&x| x >= cutoff)
        .count()
        .to_string()
}

pub fn part2(input: String) -> String {
    let g = Grid::new(&input, ()).with_wall('#');
    let end = g.find('E').unwrap();
    let path = g.bfs_generic(makeset!(end), None, None);
    let mut saves = dict!();
    for (point, (cost, _)) in path.iter() {
        for (p2, rem) in bfs_reachable(&path, *point, 20) {
            if let Some((dest_cost, _)) = path.get(&p2) {
                if cost.saturating_sub(*dest_cost) + rem as isize > 20 {
                    let entry = saves.entry((*point, p2)).or_insert(0);
                    let max = (*entry).max(cost.saturating_sub(*dest_cost) + rem as isize - 20);
                    *entry = max;
                }
            }
        }
    }
    let cutoff = if g.width() < 20 { 50 } else { 100 };
    // println!("{:?}", saves.values().filter(|&&x| x >= cutoff).counting_set());
    saves
        .into_values()
        .filter(|&x| x >= cutoff)
        .count()
        .to_string()
}

fn bfs_reachable(
    path: &FnvHashMap<Point, (isize, Point)>,
    start: Point,
    steps_remaining: usize,
) -> FnvHashSet<(Point, usize)> {
    let mut reachable = makeset!();
    let mut visited = makeset!();
    let mut queue = VecDeque::new();
    queue.push_back((start, steps_remaining));
    while let Some((point, steps_remaining)) = queue.pop_front() {
        if visited.contains(&point) {
            continue;
        }
        visited.insert(point);
        if steps_remaining == 0 {
            continue;
        }
        for d in DIRECTIONS {
            let p2 = point.offset_dir(d);
            if path.contains_key(&p2) {
                reachable.insert((p2, steps_remaining - 1));
            }
            queue.push_back((p2, steps_remaining - 1));
        }
    }
    reachable
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"#;
        assert_eq!(part1(input.to_string()), "44");
        assert_eq!(part2(input.to_string()), "285");
    }
}
