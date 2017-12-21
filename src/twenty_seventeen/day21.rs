//! ```cargo
//! [dependencies]
//! itertools = "0.7"
//! rust-crypto = "0.2"
//! chomp = "0.3.1"
//! regex = "0.2.0"
//! rayon = "0.9.0"
//! ```
#![allow(unused)]


extern crate regex;
extern crate itertools;
#[macro_use]
extern crate chomp;
extern crate rayon;

use rayon::prelude::*;
use regex::Regex;
use std::{iter,str};
use itertools::*;
use std::collections::{VecDeque,HashSet,HashMap};
use std::{mem,fmt,thread};
use chomp::prelude::{U8Input, SimpleResult, parse_only, many1, any, string, token, take_while};
use chomp::ascii::{decimal, skip_whitespace, is_whitespace, is_alphanumeric, is_alpha, signed};
use chomp::types::Buffer;
use std::fs::File;
use std::io::prelude::*;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::time::Duration;

#[derive(Eq, PartialEq, Clone, Hash)]
struct Square {
    data: Vec<Vec<u8>>,
}

impl fmt::Debug for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Square {{\n")?;
        for r in 0..self.size() {
            write!(f, "  {}\n", str::from_utf8(&self.data[r][..]).unwrap())?;
        }
        write!(f, "}} ({})\n", self.serialize())
    }
}

impl <'a> From<&'a str> for Square {
    fn from(s: &'a str) -> Square {
        Square {
            data: s.split_terminator('/').map(|l| l.bytes().collect()).collect(),
        }
    }
}

impl From<Vec<Vec<Square>>> for Square {
    fn from(parts: Vec<Vec<Square>>) -> Square {
        let part_size = parts[0][0].size();
        let final_size = part_size * parts.len();
        let mut rows = Vec::with_capacity(final_size);
        for r in 0..final_size {
            let mut col: Vec<u8> = Vec::with_capacity(final_size);
            for c in 0..parts.len() {
                col.extend_from_slice(&parts[r/part_size][c].data[r%part_size][..]);
            }
            rows.push(col);
        }
        Square { data: rows }
    }
}

impl Square {
    fn size(&self) -> usize { self.data.len() }

    fn serialize(&self) -> String {
        self.data.iter()
            .flat_map(|row| String::from_utf8(row.clone()))
            .join("/")
    }

    fn split(&self) -> Vec<Vec<Square>> {
        if self.size() % 2 == 0 {
            split_into_n(&self, self.size()/2)
        } else if self.size() % 3 == 0 {
            split_into_n(&self, self.size()/3)
        } else {
            unreachable!()
        }
    }

    fn rotate(&self) -> Square {
        let mut res = vec![vec![b' '; self.size()]; self.size()];
        for r in 0..self.size() {
            for c in 0..self.size() {
                res[r][c] = self.data[c][r];
            }
        }
        res[..].reverse();
        Square { data: res }
    }

    fn flip_x(&self) -> Square {
        let mut res = vec![vec![b' '; self.size()]; self.size()];
        for r in 0..self.size() {
            for c in 0..self.size() {
                res[r][c] = self.data[r][self.size()-1-c];
            }
        }
        Square { data: res }
    }

    fn flip_y(&self) -> Square {
        let mut res = vec![vec![b' '; self.size()]; self.size()];
        for r in 0..self.size() {
            for c in 0..self.size() {
                res[r][c] = self.data[self.size()-1-r][c];
            }
        }
        Square { data: res }
    }

    fn get_ts(&self) -> Vec<Square> {
        let mut v = self.get_ts_rot();
        v.push(self.clone());
        v.append(&mut self.flip_x().get_ts_rot());
        v.append(&mut self.flip_y().get_ts_rot());
        v.into_iter().unique().collect()
    }

    fn get_ts_rot(&self) -> Vec<Square> {
        vec![self.rotate(), self.rotate().rotate(), self.rotate().rotate().rotate()]
    }
}

fn split_into_n(sq: &Square, n: usize) -> Vec<Vec<Square>> {
    let block_size = sq.size()/n;
    let mut rows = Vec::with_capacity(n);
    for i in 0..n {
        let mut col = Vec::with_capacity(n);
        for j in 0..n {
            let mut s = Square { data: Vec::with_capacity(block_size) };
            for x in 0..block_size {
                let r_i = i*block_size + x;
                let c_i = j*block_size;
                s.data.push(sq.data[r_i][c_i..c_i+block_size].to_owned());
            }
            col.push(s);
        }
        rows.push(col);
    }
    rows
}

struct Rule {
    from: Vec<Square>,
    to: Square,
}

fn get_rules() -> Vec<Rule> {
    RULES.lines().map(|l| {
        let mut lr = l.split_terminator(" => ").map(|s| Square::from(s));
        Rule {
            from: lr.next().unwrap().get_ts(),
            to: lr.next().unwrap(),
        }
    })
    .collect()
}

fn do_the_thing(n: usize) -> usize {
    let mut current: Square = START.into();
    let rules = get_rules();
    for _ in 0..n {
        current = Square::from(current.split()
            .into_iter()
            .map(|row| row.into_iter().map(|sq| {
                    rules.iter()
                        .find(|r| r.from.iter().any(|f| f == &sq))
                        .expect("NO RULE MATCHED")
                        .to
                        .clone()
                })
                .collect::<Vec<_>>()
            )
            .collect::<Vec<_>>());
    }
    current.serialize().chars().filter(|c| *c == '#').count()
}


fn part_one() -> usize {
    do_the_thing(5)
}
fn part_two() -> usize {
    do_the_thing(18)
}

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}

const START: &str = ".#./..#/###";


const RULES: &str =
"../.. => ..#/#../.#.
#./.. => #../#../...
##/.. => ###/#.#/#..
.#/#. => ###/##./.#.
##/#. => .../.#./..#
##/## => ##./#.#/###
.../.../... => ##../.#../#.#./....
#../.../... => ..../##.#/...#/##.#
.#./.../... => ###./####/#.../#..#
##./.../... => ###./.##./...#/..##
#.#/.../... => .###/.##./#.../#.##
###/.../... => ##.#/#..#/#.#./#.##
.#./#../... => #.#./.###/#.../#.##
##./#../... => #.../####/#.##/....
..#/#../... => #.##/..#./...#/...#
#.#/#../... => #.##/####/.#.#/#.#.
.##/#../... => #.../##../##.#/.##.
###/#../... => ..../#.#./.###/#...
.../.#./... => .#.#/#..#/##../#.##
#../.#./... => ###./.###/.#.#/..#.
.#./.#./... => ..##/.##./..##/.#.#
##./.#./... => ..#./##../###./...#
#.#/.#./... => ..##/.##./.###/###.
###/.#./... => ..#./.###/###./#.##
.#./##./... => ###./..../.#../#...
##./##./... => .#.#/##../##.#/...#
..#/##./... => ##.#/.##./.###/..##
#.#/##./... => .###/..#./#.##/####
.##/##./... => ##.#/..#./..##/###.
###/##./... => ..../.#.#/.#../#...
.../#.#/... => ###./.#.#/.#../#.##
#../#.#/... => ####/#..#/..../....
.#./#.#/... => #.../..##/#.##/#.#.
##./#.#/... => #.#./###./##../#.#.
#.#/#.#/... => ...#/.##./.##./.#..
###/#.#/... => ..../.##./####/#.#.
.../###/... => .###/.#../.###/#.##
#../###/... => ..##/..##/.##./##..
.#./###/... => .#.#/..#./..##/##.#
##./###/... => ...#/#.##/#.#./##.#
#.#/###/... => #.##/.##./...#/###.
###/###/... => ##../...#/..##/####
..#/.../#.. => #.##/#.../.#../#.#.
#.#/.../#.. => .##./.##./.#.#/.##.
.##/.../#.. => .#.#/#.##/...#/##.#
###/.../#.. => ##../..#./...#/##..
.##/#../#.. => ##../..##/#..#/#..#
###/#../#.. => ##../..#./#.#./....
..#/.#./#.. => .##./##.#/##../####
#.#/.#./#.. => ####/...#/.#.#/..#.
.##/.#./#.. => .#.#/..#./##.#/.#..
###/.#./#.. => #.../#.##/..../##.#
.##/##./#.. => #.#./#.#./#.##/#.#.
###/##./#.. => ...#/###./.##./.#.#
#../..#/#.. => ####/####/..../.##.
.#./..#/#.. => #.##/...#/..#./####
##./..#/#.. => ..#./#.../..##/####
#.#/..#/#.. => #.../#.##/#.##/..##
.##/..#/#.. => ####/..../##../####
###/..#/#.. => ..../##.#/.##./####
#../#.#/#.. => ...#/..##/###./#..#
.#./#.#/#.. => #..#/..#./.###/##.#
##./#.#/#.. => ###./####/#.##/..#.
..#/#.#/#.. => ##../##.#/..##/.##.
#.#/#.#/#.. => .#.#/.##./#.../##.#
.##/#.#/#.. => .#.#/#..#/.##./..#.
###/#.#/#.. => ...#/.#../.##./##.#
#../.##/#.. => ###./##../#.#./####
.#./.##/#.. => .#../##../#.#./.#.#
##./.##/#.. => ##.#/.#../.#.#/####
#.#/.##/#.. => ####/.#.#/..../....
.##/.##/#.. => ####/##../#..#/####
###/.##/#.. => .###/##.#/.#../#.##
#../###/#.. => #..#/###./####/.#.#
.#./###/#.. => ..##/##../##.#/.#.#
##./###/#.. => #..#/.#../####/...#
..#/###/#.. => ##../##.#/...#/#..#
#.#/###/#.. => ..#./.##./#..#/....
.##/###/#.. => #..#/#.../..../.#..
###/###/#.. => ..#./#.##/.##./#...
.#./#.#/.#. => .#.#/.##./##.#/.##.
##./#.#/.#. => #..#/.###/.#.#/.##.
#.#/#.#/.#. => #.../##../#.../.###
###/#.#/.#. => ###./.###/###./....
.#./###/.#. => .#../####/...#/##..
##./###/.#. => ####/###./..../....
#.#/###/.#. => ...#/.###/..../####
###/###/.#. => ..../#.../..#./.###
#.#/..#/##. => #.#./#.../####/#.##
###/..#/##. => .#.#/#..#/.###/#...
.##/#.#/##. => ..##/..#./..../##..
###/#.#/##. => #.#./##.#/####/#..#
#.#/.##/##. => ..../.#../#.#./##.#
###/.##/##. => ..../..../.#../##.#
.##/###/##. => #.#./.###/#.#./#.##
###/###/##. => ##.#/##.#/.###/..#.
#.#/.../#.# => #..#/.#../#.../...#
###/.../#.# => ##../.#../##.#/..#.
###/#../#.# => ..##/#.#./####/.#..
#.#/.#./#.# => ...#/...#/#..#/#.#.
###/.#./#.# => ..../####/.##./.#.#
###/##./#.# => #..#/.#.#/..##/####
#.#/#.#/#.# => #.#./..#./...#/.#..
###/#.#/#.# => ...#/##.#/.###/.#..
#.#/###/#.# => .#.#/###./.#../.##.
###/###/#.# => ...#/.###/.#.#/###.
###/#.#/### => #.##/.#.#/...#/.#..
###/###/### => ..##/.#../#.#./.#..";
