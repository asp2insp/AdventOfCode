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

#[derive(Debug)]
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

fn parse<'a>(input: &'a str) -> Vec<Reaction<'a>> {
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

pub fn part1(input: String) -> String {
	let reacs = parse(&input);
	format!("FUEL: {:?}", calc_costs(&reacs)["FUEL"])
}

pub fn part2(input: String) -> String {
	"part2".to_string()
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
	assert_eq!(Rational::new(165, 1), calc_costs(&reacs)["FUEL"]);
}
