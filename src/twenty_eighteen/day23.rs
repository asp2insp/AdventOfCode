use regex::*;
use rayon::prelude::*;
use itertools::*;
use crate::utils::*;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Bot {
    x: isize,
    y: isize,
    z: isize,
    r: isize,
}

impl Bot {
    fn dist(&self, other: &Bot) -> isize {
        (self.x-other.x).abs() +
        (self.y-other.y).abs() +
        (self.z-other.z).abs()
    }

    fn in_range(&self, other: &Bot) -> bool {
        self.dist(other) < self.r
    }

    fn can_see(&self, p: (isize, isize, isize)) -> bool {
        self.in_range(&Bot { x: p.0, y: p.1, z: p.2, r: 0 })
    }

    fn min(&self) -> isize {
        self.x.min(self.y).min(self.z)
    }

    fn max(&self) -> isize {
        self.x.max(self.y).max(self.z)
    }

    fn pts(&self) -> Vec<(isize, isize, isize)> {
        let mut res = vec![];
        for z in self.z-self.r ..= self.z+self.r {
            for y in self.y-self.r ..= self.y+self.r {
                for x in self.x-self.r ..= self.x+self.r {
                    let p = (x, y, z);
                    if self.can_see(p) {
                        res.push((x, y, z));
                    }
                }
            }
        }
        res
    }
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

fn rank_pos(p: (isize, isize, isize), bots: &[Bot]) -> usize {
    bots.iter().filter(|b| b.can_see(p)).count()
}

pub fn part1(input: String) -> String {
    let bots = parse_bots(&input);
    let bigbot = bots.iter().max_by_key(|b| b.r).unwrap();
    bots.iter().filter(|b| bigbot.in_range(b)).count().to_string()
}

fn anneal(p: (isize, isize, isize), param: isize) -> Vec<(isize, isize, isize)> {
    vec![
        (p.0 + param, p.1, p.2), (p.0 - param, p.1, p.2),
        (p.0, p.1 + param, p.2), (p.0 - param, p.1 - param, p.2),
        (p.0, p.1, p.2 + param), (p.0, p.1, p.2 - param),
        p
    ]
}

pub fn part2(input: String) -> String {
    let bots = parse_bots(&input);
    let min = bots.iter().map(Bot::min).min().unwrap();
    let max = bots.iter().map(Bot::max).max().unwrap();
    let mut vec = Vec::new();
    let sample_dist = (max - min) / 100;
    for z in (min..max).step_by(sample_dist as usize) {
        for y in (min..max).step_by(sample_dist as usize) {
            for x in (min..max).step_by(sample_dist as usize) {
                vec.push(((x, y, z), 0));
            }
        }
    }
    for b in &bots {
        vec.push(((b.x, b.y, b.z), 1));
    }

    let gen_size = vec.len();
    let mut cap = (max - min) / 100;
    while cap > 0 {
        // Run 10 generations at each step size
        for _ in 0..10 {
            // In each generation, anneal every point, sort by cost,
            // then cull back to gensize
            vec = vec.into_par_iter()
                .flat_map(|p| anneal(p.0, cap).into_par_iter())
                .map(|p| (p, rank_pos(p, &bots)))
                .collect();
            vec = vec.into_iter().sorted_by_key(|p| usize::MAX - p.1).dedup().take(gen_size).collect();
            println!("gen");
        }
        // In step sizes of 10
        if cap < 10 {
            cap -= 1;
        } else {
            cap /= 10;
        }
        println!("..{} ({})", cap, vec[0].1);
    }
    let results = vec[0..100].iter().sorted_by_key(|p| p.0.0.abs() + p.0.1.abs() + p.0.2.abs()).collect::<Vec<_>>();
    format!("{:?}", &results[0..5])
}