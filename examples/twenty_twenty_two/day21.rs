use std::iter::repeat;

use aoc::{dict, parse, utils::ToDebugString};
use fnv::FnvHashMap;
use itertools::Itertools;
use regex::Regex;

enum Monkey {
    Humn(f64),
    Imm(f64),
    Op(String, char, String),
}
use Monkey::*;

impl Monkey {
    fn eval(&self, all: &FnvHashMap<String, Monkey>) -> f64 {
        match &self {
            &Humn(v) => *v,
            &Imm(v) => *v,
            &Op(a, o, b) => {
                let l = all.get(a).map(|m| m.eval(all)).unwrap();
                let r = all.get(b).map(|m| m.eval(all)).unwrap();
                match &o {
                    '+' => l + r,
                    '*' => l * r,
                    '-' => l - r,
                    '/' => l / r,
                    _ => unimplemented!("Unknown op"),
                }
            }
        }
    }

    fn eval_alg(&self, all: &FnvHashMap<String, Monkey>) -> (f64, f64) {
        match &self {
            &Humn(_) => (0., 1.),
            &Imm(v) => (*v, 0.),
            &Op(a, o, b) => {
                let l = all.get(a).map(|m| m.eval_alg(all)).unwrap();
                let r = all.get(b).map(|m| m.eval_alg(all)).unwrap();
                match &o {
                    '+' => (l.0 + r.0, l.1 + r.1),
                    '*' => (l.0 * r.0, l.1 * r.0 + l.0 * r.1),
                    '-' => (l.0 - r.0, l.1 - r.1),
                    '/' => {
						if l.1 == 0. && r.1 == 0. {
							(l.0 / r.0, 0.)
						} else if r.1 == 0. {
							(l.0 / r.0, l.1 / r.0)
						} else {
							unimplemented!()
						}
					},
                    '=' => if r.1 == 0.0 {
						(1., (r.0 - l.0) / l.1)
					} else {
						(1., (l.0 - r.0) / r.1)
					},
                    _ => unimplemented!("Unknown op"),
                }
            }
        }
    }
}

fn parse(s: &str) -> FnvHashMap<String, Monkey> {
    let imm = Regex::new(r"^(\w+): (\d+)$").unwrap();
    let op = Regex::new(r"(\w+): (\w+) (.) (\w+)").unwrap();
    let mut ret = dict!();
    for l in s.lines() {
        if let Some(caps) = imm.captures(l) {
            if &caps[1] == "humn" {
                ret.insert(caps[1].to_owned(), Humn(parse!(caps[2])));
            } else {
                ret.insert(caps[1].to_owned(), Imm(parse!(caps[2])));
            }
        } else if let Some(caps) = op.captures(l) {
            ret.insert(
                caps[1].to_owned(),
                Op(
                    caps[2].to_owned(),
                    caps[3].chars().next().unwrap(),
                    caps[4].to_owned(),
                ),
            );
        } else {
            unreachable!("Not matched")
        }
    }
    ret
}

pub fn part1(input: String) -> String {
    let monkeys = parse(&input);
    monkeys.get("root").unwrap().eval(&monkeys).to_string()
}

pub fn part2(input: String) -> String {
    let mut monkeys = parse(&input);
    if let Some(Op(l, o, r)) = monkeys.get("root") {
        return Monkey::Op(l.clone(), '=', r.clone())
            .eval_alg(&monkeys)
            .to_debug_string();
    }
    "part2".to_string()
}
