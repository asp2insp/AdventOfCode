use std::{
    collections::BinaryHeap,
    ops::{Add, Mul, Sub},
};

use aoc::{make_ord, makeset, utils::gimme_usizes_once};
use itertools::Itertools;
use rand::Rng;
use rayon::prelude::*;

#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Res {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geodes: usize,
}

impl Res {
    fn can_afford(&self, rhs: &Res) -> bool {
        self.ore >= rhs.ore
            && self.clay >= rhs.clay
            && self.obsidian >= rhs.obsidian
            && self.geodes >= rhs.geodes
    }

    fn geode_unit() -> Self {
        Res {
            geodes: 1,
            ..Default::default()
        }
    }

    fn obsidian_unit() -> Self {
        Res {
            obsidian: 1,
            ..Default::default()
        }
    }

    fn clay_unit() -> Self {
        Res {
            clay: 1,
            ..Default::default()
        }
    }

    fn ore_unit() -> Self {
        Res {
            ore: 1,
            ..Default::default()
        }
    }
}

impl Add<Res> for Res {
    type Output = Res;

    fn add(self, rhs: Res) -> Self::Output {
        Res {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geodes: self.geodes + rhs.geodes,
        }
    }
}

impl Sub<Res> for Res {
    type Output = Res;

    fn sub(self, rhs: Res) -> Self::Output {
        Res {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geodes: self.geodes - rhs.geodes,
        }
    }
}

impl Mul<usize> for Res {
    type Output = Res;

    fn mul(self, rhs: usize) -> Self::Output {
        Res {
            ore: self.ore * rhs,
            clay: self.clay * rhs,
            obsidian: self.obsidian * rhs,
            geodes: self.geodes * rhs,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Blueprint {
    id: usize,
    orebot: Res,
    claybot: Res,
    obsbot: Res,
    geodebot: Res,
}

impl Blueprint {
    fn cost_of(&self, order: Res) -> Res {
        // Convert an order of bots expressed in Res into a cost expressed in Res
        self.orebot * order.ore
            + self.claybot * order.clay
            + self.obsbot * order.obsidian
            + self.geodebot * order.geodes
    }

    fn max_ore(&self) -> usize {
		[&self.orebot, &self.claybot, &self.obsbot, &self.geodebot].into_iter().map(|r| r.ore).max().unwrap()
    }

    fn max_clay(&self) -> usize {
        self.obsbot.clay
    }
}

fn parse(s: &str) -> Blueprint {
    let v = gimme_usizes_once(s);
    Blueprint {
        id: v[0],
        orebot: Res {
            ore: v[1],
            ..Default::default()
        },
        claybot: Res {
            ore: v[2],
            ..Default::default()
        },
        obsbot: Res {
            ore: v[3],
            clay: v[4],
            ..Default::default()
        },
        geodebot: Res {
            ore: v[5],
            obsidian: v[6],
            ..Default::default()
        },
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct World<'a> {
    bots: Res,
    stock: Res,
    blueprint: &'a Blueprint,
    time: usize,
}

impl<'a> PartialOrd for World<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for World<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.stock
            .geodes
            .cmp(&other.stock.geodes)
            .then(self.bots.geodes.cmp(&other.bots.geodes))
            .then(self.stock.obsidian.cmp(&other.stock.obsidian))
            .then(self.bots.obsidian.cmp(&other.bots.obsidian))
            .then(other.time.cmp(&self.time))
    }
}

impl<'a> World<'a> {
    fn new(bp: &'a Blueprint) -> Self {
        World {
            bots: Res::ore_unit(),
            stock: Res::default(),
            blueprint: bp,
            time: 0,
        }
    }

    fn tick(&mut self, strategy: impl Fn(&World) -> Res) {
        let botbuild = strategy(&self);
        self.stock = self.stock + self.bots;
        // Make a bot?
        self.bots = self.bots + botbuild;
        self.stock = self.stock - self.blueprint.cost_of(botbuild);
        self.time += 1;
    }

    fn run(bp: Blueprint, strat: impl Fn(&World) -> Res) -> usize {
        let mut w = World::new(&bp);
        for _ in 0..24 {
            w.tick(&strat);
        }
        w.stock.geodes
    }
}

// fn greedy_strat(w: &World) -> Res {
//     let mut botbuild = Res::default();
//     // Determine which bot to make
//     if w.stock.can_afford(&w.blueprint.geodebot) {
//         botbuild.geodes += 1;
//     } else if w.stock.can_afford(&w.blueprint.obsbot) {
//         botbuild.obsidian += 1;
//     } else if w.stock.can_afford(&w.blueprint.claybot) && w.bots.clay < w.blueprint.max_clay() {
//         botbuild.clay += 1;
//     } else if w.stock.can_afford(&w.blueprint.orebot) && w.bots.ore < w.blueprint.max_ore() {
//         botbuild.ore += 1;
//     }
//     botbuild
// }

// fn vecstrat<'a>(moves: &'a Vec<usize>) -> impl Fn(&World) -> Res + 'a {
//     move |w| {
//         let desired = match moves[w.time] {
//             0 => Res::ore_unit(),
//             1 => Res::clay_unit(),
//             2 => Res::obsidian_unit(),
//             3 => Res::geode_unit(),
//             _ => unreachable!(),
//         };
//         if w.stock.can_afford(&w.blueprint.cost_of(desired)) {
//             desired
//         } else {
//             Res::default()
//         }
//     }
// }

// fn mutate(v: &[usize]) -> Vec<usize> {
//     let mut rng = rand::thread_rng();
//     v.iter()
//         .map(|e| {
//             match rng.gen_range(0..3) {
//                 0 => *e,                   // Leave it alone
//                 1 => (*e + 1).clamp(0, 3), // Incr
//                 2 => e.saturating_sub(1),  // Decr
//                 _ => unreachable!(),
//             }
//         })
//         .collect_vec()
// }

// fn anneal(bp: Blueprint, pop_size: usize, rounds: usize, retention: usize) -> usize {
//     let mut rng = rand::thread_rng();
//     let mut population = (0..pop_size)
//         .map(|_| {
//             let mut v = vec![0; 5];
//             v.extend(vec![1; 5]);
//             v.extend((0..14).map(|_| rng.gen_range(0..4)));
//             v
//         })
//         .collect_vec();
//     population.sort_by_key(|v| usize::MAX - World::run(bp, vecstrat(v)));
//     for _ in 0..rounds {
//         population.truncate(retention);
//         let mut nv = Vec::with_capacity(pop_size);
//         for i in 0..pop_size {
//             nv.push(mutate(&population[i % retention]))
//         }
//         population = nv;
//         // TODO: crossover

//         population.sort_by_key(|v| usize::MAX - World::run(bp, vecstrat(v)));
//     }
//     World::run(bp, vecstrat(&population[0]))
// }

fn nullstrat(_: &World) -> Res {
    Res::default()
}

fn best_bfs(bp: &Blueprint, rounds: usize) -> usize {
    let mut q = BinaryHeap::new();
    let mut seen = makeset!();
    q.push(World::new(&bp));
    let mut best = 0;
    while let Some(mut w) = q.pop() {
        if w.time >= rounds {
            if w.stock.geodes > best {
                best = w.stock.geodes;
                println!(
                    "B{} Found new max cand with bots {:?} stock {:?}",
                    w.blueprint.id, w.bots, w.stock
                );
            }
            continue;
        }
        if seen.contains(&w) {
            continue;
        }
        seen.insert(w.clone());
		let mut built_bot = false;
        if w.stock.can_afford(&w.blueprint.geodebot) {
            let mut w2 = w.clone();
            w2.tick(|_| Res::geode_unit());
            q.push(w2);
			built_bot = true;
        }
        if w.stock.can_afford(&w.blueprint.obsbot) {
            let mut w2 = w.clone();
            w2.tick(|_| Res::obsidian_unit());
            q.push(w2);
			built_bot = true;
        }
        if w.stock.can_afford(&w.blueprint.claybot) && w.bots.clay < w.blueprint.max_clay() {
            let mut w2 = w.clone();
            w2.tick(|_| Res::clay_unit());
            q.push(w2);
        }
        if w.stock.can_afford(&w.blueprint.orebot) && w.bots.ore < w.blueprint.max_ore() {
            let mut w2 = w.clone();
            w2.tick(|_| Res::ore_unit());
            q.push(w2);
        }
		if !built_bot {
			w.tick(&nullstrat);
			q.push(w);
		}

        if q.len() > 3000 {
            q = q.into_iter().take(2000).collect();
        }
    }
    best
}

pub fn part1(input: String) -> String {
    let prints = input.lines().map(parse).collect_vec();
    prints
        .into_par_iter()
        .map(|bp| {
            let id = bp.id;
            let best = best_bfs(&bp, 24);
            println!("B{} ended up with score {}", id, best);
            best * id
        })
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: String) -> String {
	let prints = input.lines().take(3).map(parse).collect_vec();
    prints
        .into_par_iter()
        .map(|bp| {
            let id = bp.id;
            let best = best_bfs(&bp, 32);
            println!("B{} ended up with score {}", id, best);
            best
        })
        .product::<usize>()
        .to_string()
}

#[test]
fn test() {
    let s = r#"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."#.to_owned();
    assert_eq!("33", part1(s.clone()));
	assert_eq!("3472", part2(s));
}
