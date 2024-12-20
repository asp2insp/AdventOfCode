#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(dead_code)]
use aoc::{dict, makeset};
use fnv::FnvHashMap;
use itertools::*;
use std::collections::VecDeque;
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
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

#[cfg(test)]
#[test]
fn test_invert_rot() {
    assert_eq!(S, E.invert_rot_flip(&RotFlip::new(270, false, true)));
    assert_eq!(N, E.invert_rot_flip(&RotFlip::new(270, false, false)));
}

impl Tile {
    fn edge(&self, s: Side, rf: &RotFlip) -> Vec<char> {
        let side = s.rot(rf.rot);
        let mut res = self.edge_simple(side);
        if rf.flip_y && (s == E || s == W) {
            res.reverse();
        } else if rf.flip_x && (s == N || s == S) {
            res.reverse();
        }
        res
    }

    fn edge_simple(&self, s: Side) -> Vec<char> {
        match s {
            N => self.data[0].iter().cloned().collect(),
            S => self.data[9].iter().cloned().collect(),
            E => (0..10).map(|i| self.data[i][9]).collect(),
            W => (0..10).map(|i| self.data[i][0]).collect(),
        }
    }

    fn all_edges(&self) -> impl Iterator<Item = (Side, Vec<char>)> + '_ {
        [N, S, E, W].iter().map(move |s| (*s, self.edge_simple(*s)))
    }

    fn outer_sides(&self, others: &FnvHashMap<usize, Tile>) -> Vec<Side> {
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
        for y in 0..5 {
            self.data.swap(y, 9 - y);
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

    fn orient_up(&mut self, others: &FnvHashMap<usize, Tile>) {
        let outer_side = self.outer_sides(others)[0];
        let outer_side = self.edge_simple(outer_side);
        let rf = find_rotflip_for_constraints(self, &[], &outer_side).unwrap();
        self.apply_rotflip(&rf);
    }
}

fn parse_tiles(s: &str) -> FnvHashMap<usize, Tile> {
    let mut res = dict!();
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
        assert!(t
            .data
            .iter()
            .all(|r| r.iter().all(|&c| c == '.' || c == '#')));
        res.insert(t.id, t);
    }
    res
}

fn find_matching_side(
    id: usize,
    side: &[char],
    all: &FnvHashMap<usize, Tile>,
) -> Option<(usize, Side, bool)> {
    for t in all.values() {
        if t.id == id {
            continue;
        }
        for (s, e) in t.all_edges() {
            if e == side {
                return Some((t.id, s, false));
            } else if e.iter().eq(side.iter().rev()) {
                return Some((t.id, s, true));
            }
        }
    }
    None
}

fn find_corners(all: &FnvHashMap<usize, Tile>) -> Vec<&Tile> {
    all.values()
        .filter(|t| t.outer_sides(all).len() == 2)
        .collect()
}

fn find_edges(all: &FnvHashMap<usize, Tile>) -> Vec<&Tile> {
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

impl RotFlip {
    fn new(rot: usize, flip_x: bool, flip_y: bool) -> Self {
        RotFlip {
            rot: rot,
            flip_x: flip_x,
            flip_y: flip_y,
        }
    }
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

pub fn _part2(input: String) -> String {
    let tiles = parse_tiles(&input);
    let mut corners = find_corners(&tiles);
    assert_eq!(4, corners.len());
    let mut board = [[NULL_TILE; 12]; 12];
    let mut edges = find_edges(&tiles).into_iter().cloned().collect_vec();
    assert_eq!(40, edges.len());
    edges.iter_mut().for_each(|t| t.orient_up(&tiles));
    let mut placed_edges = vec![
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
    ];
    for edge_idx in 0..4 {
        placed_edges[edge_idx].push_back(edges.pop().unwrap());
        for l in 1..10 {
            println!(
                "{:?}",
                placed_edges[edge_idx].iter().map(|v| v.id).collect_vec()
            );
            let possible_w = placed_edges[edge_idx].back().unwrap().edge_simple(E);
            let possible_e = placed_edges[edge_idx].front().unwrap().edge_simple(W);
            for i in 0..edges.len() {
                if placed_edges[edge_idx].iter().any(|pt| pt.id == edges[i].id) {
                    continue;
                }
                if edges[i].edge_simple(W) == possible_w {
                    placed_edges[edge_idx].push_back(edges[i].clone());
                    break;
                } else if edges[i].edge_simple(E) == possible_w {
                    let mut t = edges[i].clone();
                    t.flip_x();
                    placed_edges[edge_idx].push_back(t);
                } else if edges[i].edge_simple(E) == possible_e {
                    placed_edges[edge_idx].push_front(edges[i].clone());
                    break;
                } else if edges[i].edge_simple(W) == possible_e {
                    let mut t = edges[i].clone();
                    t.flip_x();
                    placed_edges[edge_idx].push_front(t);
                }
            }
            assert!(
                placed_edges[edge_idx].len() == l + 1,
                "Expected to add edge {} to {} but didn't.\n({:?})",
                l + 1,
                edge_idx,
                placed_edges.iter().map(|v| v.len()).collect_vec()
            );
        }
        edges.retain(|t| !placed_edges[edge_idx].iter().any(|pt| pt.id == t.id));
        assert_eq!(edges.len(), 40 - (edge_idx + 1) * 10);
    }

    "hi".to_owned()
}

pub fn part2(input: String) -> String {
    let tiles = parse_tiles(&input);
    let mut map = dict!();
    for (id, t) in tiles.iter() {
        for (s, e) in t.all_edges() {
            if let Some((other_id, other_side, reversed)) = find_matching_side(*id, &e, &tiles) {
                map.insert((*id, s), (other_id, other_side, reversed));
            }
        }
    }
    // Do some sanity checks:
    // print_stats(&tiles, &map);
    // Find the upper left tile to start
    let start = tiles
        .keys()
        .find(|&tid| {
            map.contains_key(&(*tid, S))
                && map.contains_key(&(*tid, E))
                && !map.contains_key(&(*tid, N))
                && !map.contains_key(&(*tid, W))
        })
        .copied()
        .unwrap();
    // Now fill in the grid
    let mut used = makeset!(start);
    let mut grid = vec![vec![NULL_TILE; 12]; 12];
    grid[0][0] = tiles[&start].clone();
    let mut pred = start;
    for it in 1..12 {
        let (_, target_side, _) = find_matching_side(
            0,
            &grid[0][it - 1].edge_simple(E),
            &dict!(pred => tiles[&pred].clone()),
        )
        .unwrap();
        // println!("{}->{:?}, looking at [{:?}] {:?}", pred, map[&(pred, target_side)], target_side, grid[0][it-1].edge_simple(target_side));
        let (next_tile, next_side, reversed) = map[&(pred, target_side)];
        assert!(!used.contains(&next_tile));
        used.insert(next_tile);
        grid[0][it] = tiles[&next_tile].clone();
        // Check Side and Reversed?
        let mut rf = RotFlip::default();
        match next_side {
            W => {}
            E => rf.flip_x = true,
            N => rf.rot = 270,
            S => rf.rot = 90,
        };
        grid[0][it].apply_rotflip(&rf);
        if grid[0][it].edge_simple(W) != grid[0][it - 1].edge_simple(E) {
            grid[0][it].flip_y();
        }
        pred = next_tile;
    }
    for col in 0..12 {
        pred = grid[0][col].id;
        for it in 1..12 {
            let (_, target_side, _) = find_matching_side(
                0,
                &grid[it - 1][col].edge_simple(S),
                &dict!(pred => tiles[&pred].clone()),
            )
            .unwrap();
            if !map.contains_key(&(pred, target_side)) {
                println!("No match for {} {:?}", pred, target_side);
                break;
            }
            // println!("{}->{:?}, looking at [{:?}] {:?}", pred, map[&(pred, target_side)], target_side, tiles[&pred].edge_simple(target_side));
            let (next_tile, next_side, reversed) = map[&(pred, target_side)];
            if used.contains(&next_tile) {
                println!("Already used {}", next_tile);
                break;
            }
            used.insert(next_tile);
            grid[it][col] = tiles[&next_tile].clone();
            // Check Side and Reversed?
            let mut rf = RotFlip::default();
            match next_side {
                N => {}
                S => rf.flip_y = true,
                E => rf.rot = 270,
                W => rf.rot = 90,
            };
            grid[it][col].apply_rotflip(&rf);
            if grid[it][col].edge_simple(N) != grid[it - 1][col].edge_simple(S) {
                grid[it][col].flip_x();
            }
            pred = next_tile;
        }
    }
    // Print puzzle borders
    // println!("{}", grid[0].iter().map(|t| format!("   {}   ", t.id)).join(" "));
    // for i in 0..120 {
    //     println!(
    //         "{}",
    //         grid[i/10].iter().map(|t| t.data[i%10].iter().enumerate().map(|idx| match (i%10, idx.0) { (9,_)|(0, _)|(_, 0|9) => idx.1, _ => &' ',}).collect::<String>()).join(" ")
    //     );
    // }

    // Condense into a single string
    let condensed = grid
        .iter()
        .flat_map(|row| {
            (1..9).map(move |i| {
                row.iter()
                    .map(|t| t.data[i][1..9].iter().collect::<String>())
                    .join("")
            })
        })
        .join("\n");
    // println!("{}", condensed);
    let condensed_arr = condensed
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    println!("{} x {} ", condensed_arr.len(), condensed_arr[0].len());
    all_orientations().map(|rf| {
        let mut arr = condensed_arr.clone();
        for _ in 0..(rf.rot / 90) {
            arr = arr.iter().enumerate().map(|(y, row)| row.iter().enumerate().map(|(x, &c)| arr[x][arr.len() -1  - y]).collect()).collect();
        }
        if rf.flip_x {
            arr.iter_mut().for_each(|row| row.reverse());
        }
        if rf.flip_y {
            arr.reverse();
        }
        let count = find_sea_monsters(&mut arr);
        let roughness = arr.iter().map(|row| row.iter().filter(|&&c| c == '#').count()).sum::<usize>();
        println!("Found {} sea monsters sea is {} ({})", count, roughness, arr.iter().map(|r| r.iter().filter(|&&c| c == 'O').count()).sum::<usize>());
        // if count > 0 {
        //     println!(
        //         "{}",
        //         arr
        //             .iter()
        //             .map(|row| row.iter().collect::<String>())
        //             .join("\n")
        //     );
        // }
        roughness
    }).min().unwrap().to_string()
}

fn find_sea_monsters(condensed_arr: &mut Vec<Vec<char>>) -> usize {
    let mut count = 0;
    // Find some sea monsters
    let sea_monster = vec![
        "                  # ",
        "#    ##    ##    ###",
        " #  #  #  #  #  #   ",
    ];
    for middle_row in 1..(condensed_arr.len() - 1) {
        for tail_col in 0..(condensed_arr[0].len() - 20) {
            if condensed_arr[middle_row][tail_col..(tail_col + 20)]
                .iter()
                .zip(sea_monster[1].chars())
                .all(|(a, b)| a == &'#' || b == ' ')
                && condensed_arr[middle_row + 1][tail_col..(tail_col + 20)]
                    .iter()
                    .zip(sea_monster[2].chars())
                    .all(|(a, b)| a == &'#' || b == ' ')
                && condensed_arr[middle_row - 1][tail_col..(tail_col + 20)]
                    .iter()
                    .zip(sea_monster[0].chars())
                    .all(|(a, b)| a == &'#' || b == ' ')
            {
                // println!("Found sea monster at {},{}", middle_row, tail_col);
                count += 1;
                for (r, row) in sea_monster.iter().enumerate() {
                    for (c, ch) in row.chars().enumerate() {
                        if ch == '#' {
                            assert!(condensed_arr[middle_row + r - 1 ][tail_col + c] == '#');
                            condensed_arr[middle_row + r - 1][tail_col + c] = 'O';
                        }
                    }
                }
            }
        }
    }
    count
}

fn print_stats(
    tiles: &FnvHashMap<usize, Tile>,
    map: &FnvHashMap<(usize, Side), (usize, Side, bool)>,
) {
    let mut corner_count = 0;
    let mut edge_count = 0;
    let mut inner_count: i32 = 0;
    for tid in tiles.keys() {
        let mut matched_edge_count = 0;
        for s in [N, S, E, W].iter() {
            if map.contains_key(&(*tid, *s)) {
                matched_edge_count += 1;
            }
        }
        match matched_edge_count {
            2 => corner_count += 1,
            3 => edge_count += 1,
            4 => inner_count += 1,
            _ => println!("Unexpected outer count: {} on {}", matched_edge_count, tid),
        }
    }
    println!(
        "Corner: {}, Edge: {}, Inner: {}",
        corner_count, edge_count, inner_count
    );
}
