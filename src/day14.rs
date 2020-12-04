use num::integer::{lcm, Integer};
use std::collections::HashMap;

macro_rules! frac (
	($n:literal / $d:literal) => {
		Rational::new($n, $d)
	};
);

#[derive(Debug, Hash, PartialEq, Clone, Copy)]
struct Item<'a> {
	name: &'a str,
	amount: usize,
}

#[derive(Debug, Clone)]
struct Reaction<'a> {
	output: Item<'a>,
	inputs: Vec<Item<'a>>,
}

fn parse_item<'a>(input: &'a str) -> Item<'a> {
	let mut nn = input.split(" ");
	Item {
		amount: nn.next().unwrap().parse().unwrap(),
		name: nn.next().unwrap(),
	}
}

fn reduce<'a>(i: &Item<'a>, reacs: &[Reaction<'a>]) -> Vec<Item<'a>> {
	let r = reacs
		.iter()
		.filter(|r| r.output.name == i.name)
		.next()
		.unwrap();
	let n = lcm(i.amount, r.output.amount);
	r.inputs
		.iter()
		.map(|e| Item {
			name: e.name,
			amount: e.amount * n,
		})
		.collect()
}

fn parse<'a>(input: &'a str) -> HashMap<&'a str, Reaction<'a>> {
	input
		.lines()
		.map(|l| {
			let mut parts = l.trim().split(" => ");
			Reaction {
				inputs: parts
					.next()
					.unwrap()
					.split(",")
					.map(str::trim)
					.map(parse_item)
					.collect(),
				output: parse_item(parts.next().unwrap()),
			}
		})
		.map(|r| (r.output.name, r))
		.collect()
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Rational {
	numer: usize,
	denom: usize,
}

impl Rational {
	fn new(n: usize, d: usize) -> Rational {
		let g = n.gcd(&d);
		Rational {
			numer: n / g,
			denom: d / g,
		}
	}
}

impl std::ops::Mul for Rational {
	type Output = Rational;
	fn mul(self, rhs: Self) -> Self {
		let n = self.numer * rhs.numer;
		let d = self.denom * rhs.denom;
		Rational::new(n, d)
	}
}

impl std::ops::Add for Rational {
	type Output = Rational;
	fn add(self, rhs: Self) -> Self {
		let lcd = lcm(self.denom, rhs.denom);
		Rational::new(
			self.numer * (lcd / self.denom) + rhs.numer * (lcd / rhs.denom),
			lcd,
		)
	}
}

impl PartialOrd for Rational {
	fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
		let lcd = lcm(self.denom, rhs.denom);
		Some((self.numer * lcd).cmp(&(rhs.numer * lcd)))
	}
}

fn calc_costs<'a>(reacs: &[Reaction<'a>]) -> HashMap<&'a str, Rational> {
	let mut costs: HashMap<&str, Rational> = HashMap::new();
	costs.insert("ORE", Rational::new(1, 1));
	let mut done = false;
	while !done {
		done = true;
		for reac in reacs {
			if costs.contains_key(reac.output.name) {
				continue;
			}
			if reac.inputs.iter().all(|i| costs.contains_key(i.name)) {
				let cost = reac
					.inputs
					.iter()
					.map(|i| {
						Rational::new(i.amount, reac.output.amount) * *costs.get(i.name).unwrap()
					})
					.fold(Rational::new(0, 1), |c, n| c + n);
				done = false;
				costs.insert(reac.output.name, cost);
				println!("Found new min cost for {} of {:?}", reac.output.name, cost);
			}
		}
	}
	costs
}

fn produce_item<'a>(
	item: &Item<'a>,
	reacs: &HashMap<&'a str, Reaction<'a>>,
	repo: &mut HashMap<&'a str, usize>,
) -> usize {
	if item.name == "ORE" {
		*repo.entry("ORE").or_insert(0) += item.amount;
		return item.amount;
	}
	if *repo.get(item.name).unwrap_or(&0) >= item.amount {
		return 0
	}
	let mut cost = 0;
	let reac = reacs.get(item.name).unwrap();
	let mut num_n = 1;
	while reac.output.amount * num_n < item.amount {
		num_n += 1;
	}
	for needed in &reac.inputs {
		cost += produce_item(&Item{amount: needed.amount * num_n, ..*needed}, reacs, repo);
		*repo.get_mut(needed.name).unwrap() -= needed.amount * num_n;
	}
	*repo.entry(item.name).or_insert(0) += reac.output.amount * num_n;
	cost
}

fn get_depth<'a>(item: &'a str, reacs: &HashMap<&'a str, Reaction>) -> usize {
	if item == "ORE" {
		0
	} else {
		1 + reacs.get(item).expect(item).inputs.iter().map(|i| get_depth(i.name, reacs)).max().unwrap_or(0)
	}
}

fn div_round_up(num: usize, denom: usize) -> usize {
	if num % denom == 0 {
		num / denom
	} else {
		num / denom + 1
	}
}

fn cost_for_fuel<'a>(amount: usize, reacs: &HashMap<&'a str, Reaction<'a>>) -> usize {
	let mut order = reacs.values().cloned().collect::<Vec<_>>();
	order.sort_by_key(|r| get_depth(r.output.name, reacs));
	let mut needed: HashMap<&'a str, usize> = HashMap::new();
	needed.insert("FUEL", amount);
	for reac in order.into_iter().rev() {
		let num_n = div_round_up(*needed.get(reac.output.name).unwrap_or(&0), reac.output.amount);
		needed.remove(reac.output.name);
		reac.inputs.into_iter().for_each(|i| { *needed.entry(i.name).or_insert(0) += i.amount * num_n});
	}
	needed["ORE"]
}

pub fn part1(input: String) -> String {
	let reacs = parse(&input);
	cost_for_fuel(1, &reacs).to_string()
}

const INIT_ORE: usize = 1_000_000_000_000;

pub fn part2(input: String) -> String {
	let reacs = parse(&input);
	let mut low = 1;
	let mut high = 1_000_000_000;
	while high > (low + 1) {
		let guess = (high + low) / 2;
		println!("{}, {}, {}", high, guess, low);

		let cost = cost_for_fuel(guess, &reacs);
		if cost > INIT_ORE {
			high = guess;
		} else if cost < INIT_ORE {
			low = guess;
		} else {
			return guess.to_string();
		}
	}
	low.to_string()
}

#[test]
fn test_basics() {
	assert_eq!(frac!(2 / 5), frac!(2 / 10) + frac!(1 / 5));
	assert_eq!(frac!(3 / 14), frac!(3 / 7) * frac!(1 / 2));
	assert_eq!(frac!(9 / 2), frac!(1 / 1) * frac!(9 / 2));
	assert_eq!(frac!(9 / 2), frac!(0 / 1) + frac!(9 / 2));
}

#[test]
fn test_small() {
	let reacs = parse(
		r#"9 ORE => 2 A
	8 ORE => 3 B
	7 ORE => 5 C
	3 A, 4 B => 1 AB
	5 B, 7 C => 1 BC
	4 C, 1 A => 1 CA
	2 AB, 3 BC, 4 CA => 1 FUEL"#,
	);
	let cost = cost_for_fuel(1, &reacs);
	assert_eq!(165, cost);
}


#[test]
fn test_large() {
	let reacs = parse(
		r#"171 ORE => 8 CNZTR
		7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
		114 ORE => 4 BHXH
		14 VRPVC => 6 BMBT
		6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
		6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
		15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
		13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
		5 BMBT => 4 WPTQ
		189 ORE => 9 KTJDG
		1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
		12 VRPVC, 27 CNZTR => 2 XDBXC
		15 KTJDG, 12 BHXH => 5 XCVML
		3 BHXH, 2 VRPVC => 7 MZWV
		121 ORE => 7 VRPVC
		7 XCVML => 6 RJRHP
		5 BHXH, 4 VRPVC => 5 LTCX"#,
	);
	let cost = cost_for_fuel(1, &reacs);
	assert_eq!(2210736, cost);
	assert_eq!(999998346916, cost_for_fuel(460664, &reacs))
}
