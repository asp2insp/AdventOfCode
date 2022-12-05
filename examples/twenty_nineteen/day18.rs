use aoc::make_ord;
use aoc::utils::*;
use fnv::{FnvHashMap, FnvHashSet};
use itertools::Itertools;
use std::collections::{BTreeSet, BinaryHeap};
use rayon::prelude::*;

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
struct State {
    cost: isize,
    loc: Point,
    keys: BTreeSet<char>,
}

make_ord!(State, |this, other| this
    .keys
    .len()
    .cmp(&other.keys.len())
    .then(other.cost.cmp(&this.cost)));

impl State {
    fn new(start_loc: Point) -> Self {
        State {
            cost: 0,
            loc: start_loc,
            keys: BTreeSet::new(),
        }
    }

    fn key(&self) -> (Vec<char>, Point) {
        (self.keys.iter().cloned().collect(), self.loc)
    }
}

pub fn part1(input: String) -> String {
    let (grid, start_loc, keys) = parse(&input);
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
        states.extend(reachable(&grid, &keys, s));
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

fn parse(s: &str) -> (Grid<()>, Point, FnvHashMap<char, Point>) {
    let g = Grid::new(s, ()).with_wall('#');
    let start_loc = g.find('@').unwrap();
    let keys = g
        .iter_chars()
        .filter(|(_, c)| *c <= 'z' && *c >= 'a')
        .map(flip)
        .collect();
    (g, start_loc, keys)
}

fn reachable(g: &Grid<()>, keys: &FnvHashMap<char, Point>, start: State) -> Vec<State> {
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
        keys: clone_with(&start.keys, g.get(to).unwrap().0),
    })
    .collect()
}

fn clone_with(s: &BTreeSet<char>, c: char) -> BTreeSet<char> {
    let mut s2 = s.clone();
    s2.insert(c);
    s2
}

#[derive(Eq, PartialEq, Clone, Debug, Hash)]

struct State2 {
    keys: BTreeSet<char>,
    positions: [Point; 4],
    cost: isize,
}

impl State2 {
    fn key(&self) -> (Vec<char>, [Point; 4]) {
        (self.keys.iter().cloned().collect_vec(), self.positions)
    }
}

make_ord!(State2, |this, other| this
    .keys
    .len()
    .cmp(&other.keys.len())
    .then(other.cost.cmp(&this.cost)));

pub fn part2(input: String) -> String {
    let (mut grid, start_loc, keys) = parse(&input);
    // Split into 4 subgrids
    grid.set(start_loc, '#', ());
    for n in grid.neighbors(start_loc) {
        grid.set(n, '#', ());
    }
    grid = grid.with_wall('#');

    let start = State2 {
        keys: BTreeSet::new(),
        positions: [
            start_loc.offset((-1, -1)),
            start_loc.offset((-1, 1)),
            start_loc.offset((1, -1)),
            start_loc.offset((1, 1)),
        ],
        cost: 0,
    };

    let mut states = BinaryHeap::new();
    states.push(start);
    let mut gen = 0;
    let mut best = isize::MAX;
    let mut seen = FnvHashMap::default();
    while !states.is_empty() {
        let working_group = (0..20)
            .filter_map(|_| states.pop())
            .filter(|s| {
                if s.cost >= best || s.cost >= *seen.get(&s.key()).unwrap_or(&best) {
                    false
                } else if s.keys.len() == keys.len() {
                    println!("Possible Soln {:?} with cost {}", s.keys, s.cost);
                    if s.cost < best {
                        best = s.cost;
                    }
                    false
                } else {
                    seen.insert(s.key(), s.cost);
                    gen += 1;
                    true
                }
            })
            .collect_vec();
        let extend = working_group.into_par_iter()
            .map(|s| {
                let mut ret = vec![];
                for i in 0..s.positions.len() {
                    for r in reachable(
                        &grid,
                        &keys,
                        State {
                            keys: s.keys.iter().cloned().collect(),
                            cost: 0,
                            loc: s.positions[i],
                        },
                    )
                    .into_iter()
                    {
                        let mut next = s.clone();
                        next.keys = r.keys;
                        next.positions[i] = r.loc;
                        next.cost += r.cost;
                        ret.push(next);
                    }
                }
                ret
            }).collect::<Vec<_>>();
        states.extend(extend.into_iter().flat_map(IntoIterator::into_iter));
            
        if gen % 10000 == 0 {
            println!(
                "Gen {} with {} current states. Current best cost of {}",
                gen,
                states.len(),
                best,
            );
        }
    }
    return best.to_string();

    // println!("{}", grid.to_string());

    // a_star(
    //     start,
    //     |s| (keys.len() - s.keys.len()) as isize,
    //     |s| {
    //         let mut ret = vec![];
    //         for i in 0..s.positions.len() {
    //             for r in reachable(&grid, &keys, State {
    //                 keys: s.keys.iter().cloned().collect(),
    //                 cost: 0,
    //                 loc: s.positions[i],
    //             }).into_iter() {
    //                 let mut next = s.clone();
    //                 next.keys = r.keys;
    //                 next.positions[i] = r.loc;
    //                 ret.push((next, r.cost));
    //             }
    //         }
    //         ret
    //     },
    //     |s| {
    //         // println!("Considering {:?}", s.keys);
    //         s.keys.len() == keys.len()
    //     },
    // )
    // .unwrap()
    // .to_string()
}

#[test]
fn test_simple() {
    let s = r#"#########
#b.A.@.a#
#########
"#;
    let (grid, start_loc, keys) = parse(s);
    // assert_eq!(
    //     "[State { cost: 2, loc: Point { x: 7, y: 1 }, keys: BTreeSet(['a']) }]",
    //     format!("{:?}", reachable(&grid, &keys, State::new(start_loc)))
    // );
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

#[test]
fn test_part_2_1() {
    let s = r#"#############
    #DcBa.#.GhKl#
    #.###.#.#I###
    #e#d##@##j#k#
    ###C#.#.###J#
    #fEbA.#.FgHi#
    #############"#
        .to_owned();
    assert_eq!("32", part2(s));

    let s2 = r#"#############
    #g#f.D#..h#l#
    #F###e#E###.#
    #dCba.#.BcIJ#
    ######@######
    #nK.L.#.G...#
    #M###N#H###.#
    #o#m..#i#jk.#
    #############"#
        .to_owned();
    assert_eq!("72", part2(s2));
}
