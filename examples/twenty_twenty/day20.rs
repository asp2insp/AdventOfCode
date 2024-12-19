use itertools::*;
use std::collections::{HashMap, HashSet, VecDeque};
use Side::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Tile {
    data: [[char; 10]; 10],
    id: usize,
}

const NULL_TILE: Tile = Tile {
    data: [['X'; 10]; 10],
    id: 0,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Side {
    N,
    S,
    E,
    W,
}

impl Side {
    fn rot(&self, angle: usize) -> Side {
        let mut res = *self;
        for _ in 0..(angle / 90) {
            res = res.rot90();
        }
        res
    }

    fn rot90(&self) -> Side {
        match self {
            N => E,
            E => S,
            S => W,
            W => N,
        }
    }
}

impl Tile {
    fn edge(&self, s: Side, rf: &RotFlip) -> Vec<char> {
        let side = s.rot(rf.rot);
        let mut res: Vec<char> = match side {
            N => self.data[0].iter().cloned().collect(),
            S => self.data[9].iter().cloned().collect(),
            E => (0..10).map(|i| self.data[i][9]).collect(),
            W => (0..10).map(|i| self.data[i][0]).collect(),
        };
        if rf.flip_y && (s == E || s == W) {
            res.reverse();
        } else if rf.flip_x && (s == N || s == S) {
            res.reverse();
        }
        res
    }

    fn all_edges(&self) -> impl Iterator<Item = (Side, Vec<char>)> + '_ {
        [N, S, E, W]
            .iter()
            .map(move |s| (*s, self.edge(*s, &RotFlip::default())))
    }

    fn outer_sides(&self, others: &HashMap<usize, Tile>) -> Vec<Side> {
        self.all_edges()
            .filter_map(|(side, s)| {
                if let Some(_) = find_matching_side(self.id, &s, others) {
                    None
                } else {
                    Some(side)
                }
            })
            .collect()
    }

    fn rot90(&mut self) {
        let mut new_data = [[' '; 10]; 10];
        for y in 0..10 {
            for x in 0..10 {
                new_data[x][9 - y] = self.data[y][x];
            }
        }
        self.data = new_data;
    }

    fn flip_x(&mut self) {
        for y in 0..10 {
            self.data[y].reverse();
        }
    }

    fn flip_y(&mut self) {
        for y in 0..10 {
            for x in 0..5 {
                self.data[y].swap(x, 9 - x);
            }
        }
    }

    fn apply_rotflip(&mut self, rf: &RotFlip) {
        for _ in 0..(rf.rot / 90) {
            self.rot90();
        }
        if rf.flip_x {
            self.flip_x();
        }
        if rf.flip_y {
            self.flip_y();
        }
    }

    fn orient_up(&mut self, others: &HashMap<usize, Tile>) {
        let outer_side = self.outer_sides(others)[0];
        let outer_side = self.edge(outer_side, &RotFlip::default());
        let rf = find_rotflip_for_constraints(self, &[], &outer_side).unwrap();
        self.apply_rotflip(&rf);
    }
}

fn parse_tiles(s: &str) -> HashMap<usize, Tile> {
    let mut res = HashMap::new();
    let mut lines = s.lines();
    while let Some(l) = lines.next() {
        if l.is_empty() {
            continue;
        }
        let id = l
            .split_whitespace()
            .nth(1)
            .unwrap()
            .trim_matches(':')
            .parse::<usize>()
            .unwrap();
        let mut t = Tile {
            id: id,
            data: [[' '; 10]; 10],
        };
        for y in 0..10 {
            let mut c = lines.next().unwrap().chars();
            for x in 0..10 {
                t.data[y][x] = c.next().unwrap();
            }
        }
        res.insert(t.id, t);
    }
    res
}

fn find_matching_side(
    id: usize,
    side: &[char],
    all: &HashMap<usize, Tile>,
) -> Option<(usize, Side)> {
    for t in all.values() {
        if t.id == id {
            continue;
        }
        for (s, e) in t.all_edges() {
            if e == side || e.iter().eq(side.iter().rev()) {
                // println!("{}-{:?} matches {}/{:?}-{:?}", id, side, t.id, s, side);
                return Some((t.id, s));
            }
        }
    }
    None
}

fn find_corners(all: &HashMap<usize, Tile>) -> Vec<&Tile> {
    all.values()
        .filter(|t| t.outer_sides(all).len() == 2)
        .collect()
}

fn find_edges(all: &HashMap<usize, Tile>) -> Vec<&Tile> {
    all.values()
        .filter(|t| t.outer_sides(all).len() == 1)
        .collect()
}

#[derive(Default, Copy, Clone, Debug)]
struct RotFlip {
    rot: usize,
    flip_x: bool,
    flip_y: bool,
}

fn all_orientations() -> impl Iterator<Item = RotFlip> {
    [0, 90, 180, 270]
        .iter()
        .cartesian_product([true, false].iter())
        .cartesian_product([true, false].iter())
        .map(move |((r, x), y)| RotFlip {
            rot: *r,
            flip_x: *x,
            flip_y: *y,
        })
}

fn find_rotflip_for_constraints(t: &Tile, left: &[char], top: &[char]) -> Option<RotFlip> {
    all_orientations()
        .filter(|rf| {
            let left_edge = t.edge(W, rf);
            let top_edge = t.edge(N, rf);
            if !left.is_empty() && left != left_edge {
                false
            } else if !top.is_empty() && top != top_edge {
                false
            } else {
                true
            }
        })
        .next()
}

pub fn part1(input: String) -> String {
    let tiles = parse_tiles(&input);
    find_corners(&tiles)
        .into_iter()
        .map(|t| t.id)
        .product::<usize>()
        .to_string()
}

fn find_next<'a>(
    is: impl Iterator<Item = &'a Tile>,
    left: &[char],
    top: &[char],
) -> (&'a Tile, RotFlip) {
    is.filter_map(|t| find_rotflip_for_constraints(t, left, top).map(|rf| (t, rf)))
        .next()
        .unwrap()
}

pub fn part2(input: String) -> String {
    let tiles = parse_tiles(&input);
    let mut corners = find_corners(&tiles);
    let mut board = [[NULL_TILE; 12]; 12];
    let mut edges = find_edges(&tiles).into_iter().cloned().collect_vec();
    edges.iter_mut().for_each(|t| t.orient_up(&tiles));
    edges = edges
        .into_iter()
        .flat_map(|e| {
            let mut erev = e.clone();
            erev.flip_x();
            [e, erev]
        })
        .collect();
    let mut placed_edges = vec![
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
    ];
    for edge_idx in 0..4 {
        placed_edges[edge_idx].push_back(edges.pop().unwrap());
        for l in 1..10 {
            let possible_w = placed_edges[edge_idx]
                .back()
                .unwrap()
                .edge(E, &RotFlip::default());
            let possible_e = placed_edges[edge_idx]
                .front()
                .unwrap()
                .edge(W, &RotFlip::default());
            for i in 0..edges.len() {
                if edges[i].edge(W, &RotFlip::default()) == possible_w {
                    placed_edges[edge_idx].push_back(edges.remove(i));
                    break;
                } else if edges[i].edge(E, &RotFlip::default()) == possible_e {
                    placed_edges[edge_idx].push_front(edges.remove(i));
                    break;
                }
            }
            assert!(placed_edges[edge_idx].len() == l + 1);
        }
    }

    "hi".to_owned()
}
