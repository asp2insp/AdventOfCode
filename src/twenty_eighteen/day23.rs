use std::cmp::max_by;

use crate::utils::*;
use itertools::*;
use rayon::prelude::*;
use regex::*;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Bot {
    x: isize,
    y: isize,
    z: isize,
    r: isize,
}

impl Bot {
    fn dist(&self, other: &Bot) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }

    fn in_range(&self, other: &Bot) -> bool {
        self.dist(other) <= self.r
    }

    fn can_see(&self, p: &(isize, isize, isize)) -> bool {
        self.in_range(&Bot {
            x: p.0,
            y: p.1,
            z: p.2,
            r: 0,
        })
    }

    fn min(&self) -> isize {
        self.x.min(self.y).min(self.z)
    }

    fn max(&self) -> isize {
        self.x.max(self.y).max(self.z)
    }

    fn can_see_sphere(&self, (sphere_p, sphere_r): &((isize, isize, isize), isize)) -> bool {
        Bot {
            r: self.r + sphere_r,
            ..*self
        }
        .can_see(sphere_p)
    }
}

fn origin_dist(cr: &((isize, isize, isize), isize)) -> isize {
    cr.0 .0.abs() + cr.0 .1.abs() + cr.0 .2.abs()
}

fn parse_bots(s: &str) -> Vec<Bot> {
    let re = Regex::new(r"pos=<([-\d]+),([-\d]+),([-\d]+)>,\s+r=(\d+)").unwrap();
    re.captures_iter(s)
        .map(|c| Bot {
            x: parse!(c[1], isize),
            y: parse!(c[2], isize),
            z: parse!(c[3], isize),
            r: parse!(c[4], isize),
        })
        .collect()
}

fn rank_pos(p: &(isize, isize, isize), bots: &[Bot]) -> usize {
    bots.iter().filter(|b| b.can_see(p)).count()
}

fn rank_sphere(s: &((isize, isize, isize), isize), bots: &[Bot]) -> usize {
    bots.iter().filter(|b| b.can_see_sphere(s)).count()
}

pub fn part1(input: String) -> String {
    let bots = parse_bots(&input);
    let bigbot = bots.iter().max_by_key(|b| b.r).unwrap();
    bots.iter()
        .filter(|b| bigbot.in_range(b))
        .count()
        .to_string()
}

fn anneal(p: (isize, isize, isize), param: isize) -> Vec<(isize, isize, isize)> {
    vec![
        (p.0 + param, p.1, p.2),
        (p.0 - param, p.1, p.2),
        (p.0, p.1 + param, p.2),
        (p.0 - param, p.1 - param, p.2),
        (p.0, p.1, p.2 + param),
        (p.0, p.1, p.2 - param),
        p,
    ]
}

// Split a sphere into 8 sub-spheres which have the 8
// cube-splits of space inscribed in them
fn partition_space((c, r): &((isize, isize, isize), isize)) -> Vec<((isize, isize, isize), isize)> {
    let shift = r / 2;
    let rnew = (shift * 99) / 70; // approx(sqrt(2)/2)
    let mut res = Vec::with_capacity(27);
    for zchange in [-1, 0, 1] {
        for ychange in [-1, 0, 1] {
            for xchange in [-1, 0, 1] {
                res.push((
                    (
                        c.0 + xchange * shift,
                        c.1 + ychange * shift,
                        c.2 + zchange * shift,
                    ),
                    rnew,
                ));
            }
        }
    }
    res
}

pub fn part2(input: String) -> String {
    let bots = parse_bots(&input);
    let min = bots.iter().map(Bot::min).min().unwrap();
    let max = bots.iter().map(Bot::max).max().unwrap();
    let mut cr = ((0, 0, 0), (max - min));
    while cr.1 > 100 {
        // println!("best {}, {}", rank_sphere(&cr, &bots), cr.1);
        cr = partition_space(&cr)
            .into_iter()
            .max_by(|s1, s2| {
                rank_sphere(s1, &bots).cmp(&rank_sphere(s2, &bots))
                    .then(origin_dist(s2).cmp(&origin_dist(s1)))
            })
            .unwrap();
    }
    // Below r = 100, search every point
    let mut best = (cr.0, 0);
    for z in cr.0.2 - cr.1 .. cr.0.2 + cr.1 {
        for y in cr.0.1 - cr.1 .. cr.0.1 + cr.1 {
            for x in cr.0.0 - cr.1 .. cr.0.0 + cr.1 {
                best = max_by(best, ((x, y, z), 0), |s1, s2| {
                    rank_sphere(s1, &bots).cmp(&rank_sphere(s2, &bots))
                        .then(origin_dist(s2).cmp(&origin_dist(s1)))
                });
            }
        }
        // println!("best {}, {}, {}, {}", z, rank_sphere(&best, &bots), best.1, origin_dist(&best));
    }
    format!(
        "{:?}, {}|{} = {}",
        best,
        rank_sphere(&best, &bots),
        rank_pos(&best.0, &bots),
        origin_dist(&best)
    )
}
