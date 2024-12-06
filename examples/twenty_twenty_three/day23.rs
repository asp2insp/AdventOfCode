use std::collections::VecDeque;

use aoc::dict;
use aoc::makeset;
use aoc::utils::AdjacencyList;
use aoc::utils::{Direction, Grid, Point};
use fnv::FnvHashSet;
use itertools::Itertools;

fn pair_dists(g: &Grid<()>, allow_uphill: bool) -> (AdjacencyList, Point, Point) {
    let (l, bt, r, top) = g.get_bounds();
    let start = g.find_in_range('.', l..=r, top..=top).unwrap();
    let end = g.find_in_range('.', l..=r, bt..=bt).unwrap();
    let mut junctions = g
        .iter_chars()
        .filter(|(_, c)| "<>v^".contains(*c)) // find slopes
        .flat_map(|(p, c)| g.neighbors(p).map(move |n| (n, g.read_pt(&n)))) // Find the char next to slope
        .filter(|(p, _c)| g.neighbors(*p).all(|n| g.read_pt(&n) != '.')) // that is surrounded by slope
        .collect_vec();
    // println!("{:?}", junctions);
    junctions.push((start, '.'));
    junctions.push((end, '.'));
    let pts = junctions;
    let mut ret = dict!();
    let gates = pts.iter().map(|(p, _)| *p).collect::<FnvHashSet<_>>();
    for (gate_p, _gate_c) in pts {
        let starts = makeset!(gate_p);
        let reachable = g.bfs_generic(
            starts,
            Some(&|pt| {
                if pt != gate_p && gates.contains(&pt) {
                    vec![]
                } else {
                    g.neighbors_with_directions(pt)
                        .filter(|(d, p)| match (d, g.read_pt(p)) {
                            (Direction::E, '<') => allow_uphill,
                            (Direction::W, '>') => allow_uphill,
                            (Direction::N, 'v') => allow_uphill,
                            (Direction::S, '^') => allow_uphill,
                            _ => true,
                        })
                        .map(|n| (n.1, 1))
                        .collect_vec()
                }
            }),
            None,
        );
        for (p, dist) in reachable {
            if gates.contains(&p) && p != gate_p {
                ret.entry(gate_p).or_insert(dict!()).insert(p, dist.0);
            }
        }
    }
    (AdjacencyList::new(ret), start, end)
}

pub fn part1(input: String) -> String {
    let mut g = Grid::new(&input, ());
    g.wall_char = '#';
    let (adj_list, start, end) = pair_dists(&g, false);
    println!("{:?}", adj_list);
    let mut max_dist = 0;
    let mut q = VecDeque::new();
    q.push_back((start, 0, makeset!(start)));
    while let Some((p, dist, seen)) = q.pop_front() {
        if p == end && dist > max_dist {
            max_dist = dist;
        }
        adj_list
            .bfs_step((p, dist, seen), None)
            .into_iter()
            .for_each(|x| q.push_front(x));
    }
    max_dist.to_string()
}

pub fn part2(input: String) -> String {
    let mut g = Grid::new(&input, ());
    g.wall_char = '#';
    let (mut adj_list, start, end) = pair_dists(&g, true);
    adj_list.add_back_edges();
    // println!("{:?}", adj_list);
    let mut max_dist = 0;
    let mut q = VecDeque::new();
    q.push_back((start, 0, makeset!(start)));
    // q.pop();
    while let Some((p, dist, seen)) = q.pop_front() {
        if p == end && dist > max_dist {
            // println!("Found new max dist: {}", dist);
            max_dist = dist;
        }
        adj_list
            .bfs_step((p, dist, seen), None)
            .into_iter()
            .for_each(|x| q.push_front(x));
    }
    max_dist.to_string()
}

#[test]
fn test_mini() {
    let input = r#"#.#######
		#..>.>..#
		####v####
		####....#
		#######.#"#;
    assert_eq!(part1(input.to_string()), "10");
}

#[test]
fn test_example() {
    assert_eq!("94", part1(EXAMPLE.to_string()));
    assert_eq!("154", part2(EXAMPLE.to_string()));
}

const EXAMPLE: &str = r#"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"#;
