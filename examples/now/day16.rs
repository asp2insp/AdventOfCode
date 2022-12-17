use aoc::dict;
use fnv::FnvHashMap;
use itertools::Itertools;
use rayon::prelude::IntoParallelIterator;
use regex::*;
use smallstr::SmallString;
use rayon::prelude::*;

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
        if v.rate > 0 && !open.contains(&loc) {
            possibles.push((loc.clone(), clone_with(&open, &loc)));
        }
    }
    let ret = possibles
        .into_iter()
        .map(|(l, o)| most_pressure(time - 1, l, o, map, memo) + this_minute)
        .max()
        .unwrap_or(0);
    memo.insert((time, loc, open), ret);
    ret
}

pub fn part1(input: String) -> String {
    let valves = parse(&input);
    // println!("Start: {:?}", valves.get(&"AA".to_owned()));
    // Dynamic problem. Primary axis is minute, state space is location, valves open
    most_pressure(30, SmallString::from("AA"), vec![], &valves, &mut dict! {}).to_string()
}

fn replace_with<T>(v: &Vec<T>, f: &T, t: &T) -> Vec<T>
where
    T: Ord + Clone,
{
    let mut r = v.clone();
	if &r[0] == f {
		r[0] = t.clone();
	} else if &r[1] == f {
		r[1] = t.clone();
	}
	r
}

fn most_pressure2(
    time: usize,
    locs: Vec<S>,
    open: Vec<S>,
    map: &FnvHashMap<S, Valve>,
    memo: &mut FnvHashMap<(usize, Vec<S>, Vec<S>), usize>,
) -> usize {
    if let Some(ret) = memo.get(&(time, locs.clone(), open.clone())) {
        return *ret;
    }
    let this_minute = open
        .iter()
        .filter_map(|n| map.get(n))
        .map(|v| v.rate)
        .sum::<usize>();
    if time == 1 {
        // Last minute, just the presure of what's done
        return this_minute;
    }

    let mut possibles = vec![];
	let mv = map.get(&locs[0]).unwrap();
	mv.tunnels.iter().for_each(|t| {
		possibles.push((replace_with(&locs, &locs[0], t), open.clone()));
	});
	if mv.rate > 0 && !open.contains(&locs[0]) {
		possibles.push((locs.clone(), clone_with(&open, &locs[0])));
	}

	let ev = map.get(&locs[1]).unwrap();
	let mut possibles2 = vec![];
	for p in possibles.into_iter() {
		ev.tunnels.iter().for_each(|t| {
			possibles2.push((replace_with(&p.0, &locs[1], t), p.1.clone()));
		});
		if ev.rate > 0 && !p.1.contains(&locs[1]) {
			possibles2.push((p.0.clone(), clone_with(&p.1, &locs[1])));
		}
	}
	possibles2.sort();
	possibles2.dedup();

    let ret = possibles2
        .into_iter()
        .map(|(l, o)| most_pressure2(time - 1, l, o, map, memo) + this_minute)
        .max()
        .unwrap_or(0);
    memo.insert((time, locs, open), ret);
    ret
}



pub fn part2(input: String) -> String {
    let valves = parse(&input);
    // println!("Start: {:?}", valves.get(&"AA".to_owned()));
    // Dynamic problem. Primary axis is minute, state space is location, elephant loc, valves open
    println!("{:?}", build_adj_list(&valves));

    let starts = valves.get(&SmallString::from("AA")).unwrap().tunnels.clone();
    let starts = starts.iter().cartesian_product(starts.iter()).collect_vec();
    starts.into_par_iter()
        .map(|ss| {
            most_pressure2(
                25,
                vec![ss.0.clone(), ss.1.clone()],
                vec![],
                &valves,
                &mut dict! {},
            )
        })
        .max()
        .unwrap()
        .to_string()
}

fn build_adj_list(map: &FnvHashMap<S, Valve>) -> Vec<(S, S, usize)> {
    let mut r = vec![];
    for (n, v) in map {
        for t in &v.tunnels {
            r.push((n.clone(), t.clone(), 1));
        }
    }
    let zeros = map.keys().filter(|&k| map.get(k).unwrap().rate == 0).collect_vec();
    for z in zeros {
        let mut rnew = vec![];
        let adjs = r.iter().filter(|(f, _, _)| f == z).cloned().collect_vec();
        for row in r.into_iter() {
            if row.0 == *z {
                continue;
            } else if row.1 == *z {
                for a in &adjs {
                    if row.0 != a.1 {
                        rnew.push((row.0.clone(), a.1.clone(), row.2 + a.2));
                    }
                }
            } else {
                rnew.push(row);
            }
        }
        rnew.sort();
        rnew.dedup_by_key(|r| (r.0.clone(), r.1.clone()));
        r = rnew;
    }
    r
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
