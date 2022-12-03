use aoc::utils::*;
use fnv::{FnvHashMap, FnvHashSet};
use itertools::Itertools;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
struct State {
    cost: isize,
    loc: Point,
    keys: VecSet<char>,
}

impl std::cmp::PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.keys
            .len()
            .cmp(&other.keys.len())
            .then(other.cost.cmp(&self.cost))
        // other.cost
        //     .cmp(&self.cost)
        //     .then(self.keys.len().cmp(&other.keys.len()))
    }
}

impl State {
    fn new(start_loc: Point) -> Self {
        State {
            cost: 0,
            loc: start_loc,
            keys: VecSet::new(),
        }
    }

    fn key(&self) -> (Vec<char>, Point) {
        (self.keys.key(), self.loc)
    }
}

pub fn part1(input: String) -> String {
    let (grid, start_loc, keys, distances) = parse(&input);
    let mut states = BinaryHeap::new();
    states.push(State::new(start_loc));
    let mut gen = 0;
    let mut best = isize::MAX;
    let mut seen = FnvHashMap::default();
    while let Some(s) = states.pop() {
        if s.cost >= best || s.cost >= *seen.get(&s.key()).unwrap_or(&best) {
            continue;
        }
        seen.insert(s.key(), s.cost);
        let curr = (s.cost, s.keys.clone());
        if s.keys.len() == keys.len() {
            println!("Possible Soln {:?} with cost {}", s.keys, s.cost);
            if s.cost < best {
                best = s.cost;
            }
            continue;
        }
        gen += 1;
        states.extend(reachable(&grid, &keys, &distances, s));
        if gen % 10000 == 0 {
            println!(
                "Gen {} with {} current states. Current is {:?} keys at a cost of {}",
                gen,
                states.len(),
                curr.1,
                curr.0,
            );
        }
    }
    return best.to_string();
}

fn parse(
    s: &str,
) -> (
    Grid<()>,
    Point,
    FnvHashMap<char, Point>,
    FnvHashMap<(Point, Point), isize>,
) {
    let g = Grid::new(s, ()).with_wall('#');
    let start_loc = g.find('@').unwrap();
    let keys = g
        .iter_chars()
        .filter(|(_, c)| *c <= 'z' && *c >= 'a')
        .map(flip)
        .collect();
    let dist = find_distances(&g, &keys, start_loc);
    (g, start_loc, keys, dist)
}

fn find_distances(
    g: &Grid<()>,
    keys: &FnvHashMap<char, Point>,
    start_loc: Point,
) -> FnvHashMap<(Point, Point), isize> {
    // For each key and start loc, find distances to all other keys
    let mut ret = FnvHashMap::with_capacity_and_hasher((keys.len() + 1).pow(2), Default::default());
    ret.extend(
        g.dfs_path_bulk(start_loc, keys.values().cloned().collect(), Some(|_| 1))
            .into_iter()
            .map(|(p, (c, _))| ((start_loc, p), c)),
    );
    for from in keys.values() {
        ret.extend(
            g.dfs_path_bulk(*from, keys.values().cloned().collect(), Some(|_| 1))
                .into_iter()
                .map(|(p, (c, _))| ((*from, p), c)),
        );
    }
    ret
}

fn reachable(
    g: &Grid<()>,
    keys: &FnvHashMap<char, Point>,
    distances: &FnvHashMap<(Point, Point), isize>,
    start: State,
) -> Vec<State> {
    g.dfs_path_bulk(
        start.loc,
        keys.iter()
            .filter_map(|(c, &p)| {
                if start.keys.contains(c) {
                    None
                } else {
                    Some(p)
                }
            })
            .collect(),
        Some(|np| {
            let nc = g.read_pt(&np);
            if nc.is_uppercase()
                && nc.is_alphabetic()
                && !start.keys.contains(&nc.to_ascii_lowercase())
            {
                isize::MAX
            } else {
                1
            }
        }),
    )
    .into_iter()
    .map(|(to, (ncost, _))| State {
        loc: to,
        cost: start.cost.saturating_add(ncost),
        keys: start.keys.clone_with(g.get(to).unwrap().0),
    })
    .collect()
}

pub fn part2(input: String) -> String {
    let (mut grid, start_loc, keys, distances) = parse(&input);
    // Split into 4 subgrids
    grid.set(start_loc, '#', ());
    for n in grid.neighbors(start_loc) {
        grid.set(n, '#', ());
    }
    grid = grid.with_wall('#');

    let states = [
        State::new(start_loc.offset((-1, -1))),
        State::new(start_loc.offset((-1, 1))),
        State::new(start_loc.offset((1, -1))),
        State::new(start_loc.offset((1, 1))),
    ];

    let mut states = BinaryHeap::new();
    states.push(State::new(start_loc));
    "part2".to_string()
}

#[test]
fn test_simple() {
    let s = r#"#########
#b.A.@.a#
#########
"#;
    let (grid, start_loc, keys, distances) = parse(s);
    assert_eq!(
        "[State { cost: 2, loc: Point { x: 7, y: 1 }, keys: VecSet(['a']) }]",
        format!(
            "{:?}",
            reachable(&grid, &keys, &distances, State::new(start_loc))
        )
    );
    assert_eq!("8", part1(s.to_owned()));
}

#[test]
fn test_larger() {
    let s = r#"########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################
"#;
    assert_eq!("86", part1(s.to_owned()));
}

#[test]
fn test_2() {
    let s2 = r#"########################
    #...............b.C.D.f#
    #.######################
    #.....@.a.B.c.d.A.e.F.g#
    ########################"#;
    assert_eq!("132", part1(s2.to_owned()));
}

#[test]
fn test_3() {
    let s3 = r#"#################
    #i.G..c...e..H.p#
    ########.########
    #j.A..b...f..D.o#
    ########@########
    #k.E..a...g..B.n#
    ########.########
    #l.F..d...h..C.m#
    #################"#;
    assert_eq!("136", part1(s3.to_owned()));
    // assert!(false)
}

#[test]
fn test_4() {
    let s4 = r#"########################
    #@..............ac.GI.b#
    ###d#e#f################
    ###A#B#C################
    ###g#h#i################
    ########################"#;
    assert_eq!("81", part1(s4.to_owned()));

    // assert!(false);
}
