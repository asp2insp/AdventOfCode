use regex::Regex;
use std::collections::HashMap;
use aoc::utils::BetterRange;
use itertools::Itertools;

#[derive(Debug)]
struct Part {
	x: isize,
	m: isize,
	a: isize,
	s: isize,
}

impl Part {
	fn get_x(&self) -> isize {
		self.x
	}
	fn get_m(&self) -> isize {
		self.m
	}
	fn get_a(&self) -> isize {
		self.a
	}
	fn get_s(&self) -> isize {
		self.s
	}
}

#[derive(Clone)]
enum Dest {
	Reject,
	Accept,
	Workflow(String),
}

type Rule = Box<dyn Fn(&Part) -> Option<Dest>>;

fn parse(input: &str) -> (Vec<Part>, HashMap<String, Vec<Rule>>) {
	let mut parts = vec![];
	let mut rules = HashMap::new();
	// parse e.g. {x=1526,m=251,a=2007,s=1686}
	let part_re = Regex::new(r#"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}"#).unwrap();
	for part in part_re.captures_iter(input) {
		parts.push(Part {
			x: part[1].parse().unwrap(),
			m: part[2].parse().unwrap(),
			a: part[3].parse().unwrap(),
			s: part[4].parse().unwrap(),
		});
	}
	// parse e.g. rj{s>2173:A,a>1545:R,m<3290:A,R}
	let workflow_re = Regex::new(r#"(?P<name>\w+)\{(?P<rules>[^}]+)+\}"#).unwrap();
	let rule_re = Regex::new(r#"(?P<prop>[xmas])(?P<op>[<>])(?P<val>\d+):(?P<dest>\w+)"#).unwrap();
	for work in workflow_re.captures_iter(input) {
		let mut rs = vec![];
		for rule in work["rules"].split(',') {
			if let Some(caps) = rule_re.captures(rule) {
				let cmp = match &caps["op"] {
					"<" => isize::lt,
					">" => isize::gt,
					_ => panic!("Unknown operator"),
				};
				let extract = match &caps["prop"] {
					"x" => Part::get_x,
					"m" => Part::get_m,
					"a" => Part::get_a,
					"s" => Part::get_s,
					_ => panic!("Unknown property"),
				};
				let dest = match &caps["dest"] {
					"A" => Dest::Accept,
					"R" => Dest::Reject,
					w => Dest::Workflow(w.to_string()),
				};
				let val: isize = caps["val"].parse().unwrap();
				rs.push(Box::new(move |p: &Part| {
					if cmp(&extract(p), &val) {
						Some(dest.clone())
					} else {
						None
					}
				}) as Rule);
			} else {
				let dest = match rule {
					"A" => Dest::Accept,
					"R" => Dest::Reject,
					w => Dest::Workflow(w.to_string()),
				};
				let final_rule = Box::new(move |_: &Part| Some(dest.clone())) as Rule;
				rs.push(final_rule);
			}
		}
		rules.insert(work["name"].to_string(), rs);
	}
	(parts, rules)
}

pub fn part1(input: String) -> String {
	let (parts, rules) = parse(&input);
	let mut accepted = vec![];
	for p in parts {
		let mut state = "in".to_owned();
		loop {
			match rules.get(&state).and_then(|rs| rs.iter().find_map(|f| f(&p))) {
				Some(Dest::Accept) => {
					accepted.push(p);
					break;
				},
				Some(Dest::Reject) => {
					break;
				},
				Some(Dest::Workflow(w)) => {
					state = w.to_string();
				},
				None => {
					panic!("No rule for {}, {:?}", state, p);
				}
			}
		}
	}
	accepted.into_iter().map(|p| p.x+p.m+p.a+p.s).sum::<isize>().to_string()
}

#[derive(Clone, Debug)]
struct RangePart {
	x: BetterRange,
	m: BetterRange,
	a: BetterRange,
	s: BetterRange,
}

impl RangePart {
	fn apply(&self, cmp: &str, val: usize, which: &str) -> RangePart {
		let mut newpart = self.clone();
		match which {
			"x" => newpart.x = newpart.x.restrict(cmp, val),
			"m" => newpart.m = newpart.m.restrict(cmp, val),
			"a" => newpart.a = newpart.a.restrict(cmp, val),
			"s" => newpart.s = newpart.s.restrict(cmp, val),
			"" => {},
			_ => panic!("Unknown which {}", which),
		};
		newpart
	}

	fn is_valid(&self) -> bool {
		!(self.x.is_empty() || self.m.is_empty() || self.a.is_empty() || self.s.is_empty())
	}
}

pub fn part2(input: String) -> String {
	let mut rules = HashMap::new();
	let workflow_re = Regex::new(r#"(?P<name>\w+)\{(?P<rules>[^}]+)+\}"#).unwrap();
	let rule_re = Regex::new(r#"(?P<prop>[xmas])(?P<op>[<>])(?P<val>\d+):(?P<dest>\w+)"#).unwrap();
	for work in workflow_re.captures_iter(&input) {
		let mut rs = vec![];
		for rule in work["rules"].split(',') {
			if let Some(caps) = rule_re.captures(rule) {
				let cmp = caps["op"].to_string();
				let which = caps["prop"].to_string();
				let dest = match &caps["dest"] {
					"A" => Dest::Accept,
					"R" => Dest::Reject,
					w => Dest::Workflow(w.to_string()),
				};
				let val: usize = caps["val"].parse().unwrap();
				rs.push((which, cmp, val, dest));
			} else {
				let dest = match rule {
					"A" => Dest::Accept,
					"R" => Dest::Reject,
					w => Dest::Workflow(w.to_string()),
				};
				let final_rule = ("".to_owned(), "".to_owned(), 0, dest);
				rs.push(final_rule);
			}
		}
		rules.insert(work["name"].to_string(), rs);
	}

	let mut rangers = vec![("in".to_owned(), RangePart {
		x: BetterRange::new(0, usize::MAX),
		m: BetterRange::new(0, usize::MAX),
		a: BetterRange::new(0, usize::MAX),
		s: BetterRange::new(0, usize::MAX),
	})];

	let mut accepted = vec![];
	let mut state = "in".to_owned();
	while !rangers.is_empty() {
		rangers = rangers.into_iter().flat_map(|(state, rp)| {
			// TODO this needs to be fold -- where the subsequent rule receives a modified RangePart that
			// doesn't pass the first rule
			rules.get(&state).unwrap().iter().filter_map(|(which, cmp, val, dest)| {
				let newrp = rp.apply(cmp, *val, which);
				if newrp.is_valid() {
					match dest {
						Dest::Accept => {
							accepted.push(newrp);
							None
						},
						Dest::Reject => None,
						Dest::Workflow(w) => Some((w.to_string(), newrp)),
					}
				} else {
					None
				}
			})
			.collect_vec()
		})
		.collect::<Vec<_>>();
	}
	format!("{:?}", accepted)
}
