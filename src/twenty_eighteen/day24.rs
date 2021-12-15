use itertools::Itertools;
use regex::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Group {
    units: usize,
    hp: usize,
    immune: Vec<String>,
    weak: Vec<String>,
    damage: usize,
    damage_type: String,
    initiative: usize,
}

impl Group {
    fn effective_power(&self) -> usize {
        self.units * self.damage
    }

    fn damage_to(&self, other: &Group) -> usize {
        if other.immune.contains(&self.damage_type) {
            0
        } else if other.weak.contains(&self.damage_type) {
            self.effective_power() * 2
        } else {
            self.effective_power()
        }
    }
}

fn parse_army(s: &str) -> Vec<Group> {
    let re = Regex::new(r"(?P<units>\d+) units each with (?P<hp>\d+) hit points (?P<mods>\([^)]+\)\s*)?with an attack that does (?P<damage>\d+) (?P<type>\w+) damage at initiative (?P<init>\d+)").unwrap();
    re.captures_iter(s)
        .map(|cap| {
            let mut weak = vec![];
            let mut immune = vec![];
            if let Some(modt) = cap.name("mods") {
                for modt_s in modt.as_str().split(";") {
                    let modt_s = modt_s.trim().trim_start_matches("(").trim_end_matches(")");
                    if modt_s.starts_with("weak") {
                        weak = modt_s
                            .trim_start_matches("weak to ")
                            .split(",")
                            .map(str::trim)
                            .map(str::to_string)
                            .collect_vec();
                    } else {
                        immune = modt_s
                            .trim_start_matches("immune to ")
                            .split(",")
                            .map(str::trim)
                            .map(str::to_string)
                            .collect_vec();
                    }
                }
            }
            Group {
                units: parse!(cap["units"], usize),
                hp: parse!(cap["hp"], usize),
                immune,
                weak,
                damage: parse!(cap["damage"], usize),
                damage_type: cap["type"].to_string(),
                initiative: parse!(cap["init"], usize),
            }
        })
        .collect_vec()
}

fn select_targets(from: &[Group], to: &[Group]) -> HashMap<usize, usize> {
    let mut map = HashMap::new();
    let mut seen = makeset![];
    for attack in (0..from.len())
        .filter(|i| from[*i].units > 0)
        .sorted_by(|&a, &b| {
            from[a]
                .effective_power()
                .cmp(&from[b].effective_power())
                .then(from[a].initiative.cmp(&from[b].initiative))
                .reverse()
        })
    {
        if let Some(defend) = (0..to.len())
            .filter(|i| !seen.contains(i) && to[*i].units > 0)
            .max_by(|&a, &b| {
                from[attack]
                    .damage_to(&to[a])
                    .cmp(&from[attack].damage_to(&to[b]))
                    .then(to[a].effective_power().cmp(&to[b].effective_power()))
					.then(to[a].initiative.cmp(&to[b].initiative))
            })
        {
            if from[attack].damage_to(&to[defend]) == 0 {
                continue;
            }
            seen.insert(defend);
            map.insert(attack, defend);
        }
    }
    map
}

fn run_battle(mut armies: Vec<Vec<Group>>, is_debug: bool) -> (usize, usize) {
	let army_count = armies[0].len();
	while armies[0].iter().any(|g| g.units > 0) && armies[1].iter().any(|g| g.units > 0) {
        // Target Selection
        let immune_targets = select_targets(&armies[0], &armies[1]);
        let infect_targets = select_targets(&armies[1], &armies[0]);
		if immune_targets.len() == 0 && infect_targets.len() == 0 {
			// stalemate
			break
		}
        let targets = vec![immune_targets, infect_targets];

        // attacks
        let order = (0..2)
            .cartesian_product(0..army_count)
            .sorted_by_key(|(a, i)| usize::MAX - armies[*a][*i].initiative)
            .collect_vec();
        for (a, i) in order {
            if !targets[a].contains_key(&i) {
                continue;
            }
            let att = &armies[a][i];
            let target = targets[a][&i];
            let dfn = &armies[(a + 1) % 2][target];
            let damage = att.damage_to(&dfn);

			if is_debug {
				println!(
					"{}.{}({}) hits {}.{}({}) for {} ({} units)",
					a,
					i,
					att.units,
					(a + 1) % 2,
					target,
					dfn.units,
					damage,
					damage / dfn.hp
				);
			}

            let m = &mut armies[(a + 1) % 2][target];
            m.units = m.units.saturating_sub(damage / m.hp);
        }
		if is_debug {
        	println!(
				"> {:?}\n  {:?}",
				armies[0].iter().map(|g| g.units).collect_vec(),
				armies[1].iter().map(|g| g.units).collect_vec()
			);
		}
    }

    armies
            .iter()
            .map(|a| a.iter().map(|g| g.units).sum::<usize>())
            .collect_tuple()
			.unwrap()
}

pub fn part1(input: String) -> String {
	let armies = input.split("\n\n")
		.map(parse_army)
		.collect_vec();
   format!("{:?}", run_battle(armies, false))
}

fn boost(n: usize, armies: &Vec<Vec<Group>>) -> Vec<Vec<Group>> {
	let mut res = armies.clone();
	res[0].iter_mut().for_each(|g| g.damage += n);
	res
}

pub fn part2(input: String) -> String {
    let armies = input.split("\n\n")
		.map(parse_army)
		.collect_vec();
	let mut low: usize = 0;
	let mut high: usize = 1000*1000;
	while high.saturating_sub(low) > 1 {
		let next_mid = low +  (high - low) / 2;
		let (imm, inf) = run_battle(boost(next_mid, &armies), false);
		if imm == 0 {
			low = next_mid;
		} else if inf == 0 {
			high = next_mid;
		} else {
			low = low.saturating_sub(10);
			high += 10;
		}
		println!("{} - {}", low, high);
	}
   format!("{:?}", run_battle(boost(high.max(low), &armies), false))
}

#[test]
fn test() {
    let s = r"Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4";

    assert_eq!("(0, 5216)", part1(s.to_string()));
}
