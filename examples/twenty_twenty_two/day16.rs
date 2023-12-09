use aoc::dict;
use fnv::{FnvHashMap, FnvHashSet};
use itertools::Itertools;
use rayon::prelude::IntoParallelIterator;
use rayon::prelude::*;
use regex::*;
use smallstr::SmallString;

type S = SmallString<[u8; 2]>;

#[derive(Debug, Hash)]
struct Valve {
    rate: usize,
    tunnels: Vec<S>,
}

fn parse(s: &str) -> FnvHashMap<S, Valve> {
    let re =
        Regex::new(r#"Valve (..) has flow rate=(\d+); tunnels? leads? to valves? (.+)"#).unwrap();
    s.lines()
        .map(|l| {
            let caps = re.captures(l).unwrap();
            (
                SmallString::from(&caps[1]),
                Valve {
                    rate: caps[2].parse::<usize>().unwrap(),
                    tunnels: caps[3]
                        .split(",")
                        .map(|t| SmallString::from(t.trim()))
                        .collect_vec(),
                },
            )
        })
        .collect()
}

fn clone_with<T>(v: &Vec<T>, t: &T) -> Vec<T>
where
    T: Ord + Clone,
{
    let mut r = v.clone();
    r.push(t.clone());
    r.sort();
    r
}

fn most_pressure(
    time: usize,
    loc: S,
    open: Vec<S>,
    map: &FnvHashMap<S, Valve>,
    targets: &FnvHashSet<S>,
    memo: &mut FnvHashMap<(usize, S, Vec<S>), usize>,
) -> usize {
    if let Some(ret) = memo.get(&(time, loc.clone(), open.clone())) {
        return *ret;
    }
    let this_minute = open
        .iter()
        .filter_map(|n| map.get(n))
        .map(|v| v.rate)
        .sum::<usize>();
    // println!("At {} for minute {} with {:?} open, flows {}", loc, time, open, this_minute);
    if time == 1 {
        // Last minute, just the presure of what's done
        return this_minute;
    }

    let mut possibles = vec![];
    if let Some(v) = map.get(&loc) {
        v.tunnels.iter().for_each(|t| {
            possibles.push((t.clone(), open.clone()));
        });
        if targets.contains(&loc) && v.rate > 0 && !open.contains(&loc) {
            possibles.push((loc.clone(), clone_with(&open, &loc)));
        }
    }
    let ret = possibles
        .into_iter()
        .map(|(l, o)| most_pressure(time - 1, l, o, map, targets, memo) + this_minute)
        .max()
        .unwrap_or(0);
    memo.insert((time, loc, open), ret);
    ret
}

pub fn part1(input: String) -> String {
    let valves = parse(&input);
    // println!("Start: {:?}", valves.get(&"AA".to_owned()));
    // Dynamic problem. Primary axis is minute, state space is location, valves open
    most_pressure(
        30,
        SmallString::from("AA"),
        vec![],
        &valves,
        &valves.keys().cloned().collect::<FnvHashSet<S>>(),
        &mut dict! {},
    )
    .to_string()
}

pub fn part2(input: String) -> String {
    let valves = parse(&input);
    let valvuable = valves
        .iter()
        .filter_map(|(k, v)| if v.rate > 0 { Some(k.clone()) } else { None })
        .collect::<FnvHashSet<S>>();
    let partitions = valvuable.clone().into_iter().powerset().collect_vec();
    partitions
        .into_par_iter()
        .map(|p0| {
            let t0 = p0.into_iter().collect::<FnvHashSet<S>>();
            let t1 = valvuable.difference(&t0).cloned().collect::<FnvHashSet<S>>();
            most_pressure(26, "AA".into(), vec![], &valves, &t0, &mut dict!())
                + most_pressure(26, "AA".into(), vec![], &valves, &t1, &mut dict!())
        })
        .max()
        .unwrap()
        .to_string()
}

#[test]
fn test1() {
    let s = r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
	Valve BB has flow rate=13; tunnels lead to valves CC, AA
	Valve CC has flow rate=2; tunnels lead to valves DD, BB
	Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
	Valve EE has flow rate=3; tunnels lead to valves FF, DD
	Valve FF has flow rate=0; tunnels lead to valves EE, GG
	Valve GG has flow rate=0; tunnels lead to valves FF, HH
	Valve HH has flow rate=22; tunnel leads to valve GG
	Valve II has flow rate=0; tunnels lead to valves AA, JJ
	Valve JJ has flow rate=21; tunnel leads to valve II"#
        .to_owned();
    assert_eq!("1651", part1(s.clone()));
    assert_eq!("1707", part2(s));
}
