use std::ops::RangeInclusive;
use itertools::*;
use std::collections::HashSet;

#[derive(Debug)]
struct Rule {
	name: String,
	range1: RangeInclusive<usize>,
	range2: RangeInclusive<usize>,
}

impl Rule {
	fn is_valid(&self, n: &usize) -> bool {
		self.range1.contains(n) || self.range2.contains(n)
	}
}

fn parse_rule(s: &str) -> Option<Rule> {
	let mut parts = s.split(':');
	let name = parts.next()?.to_owned();
	let mut ranges = parts.next()?.split("or");
	let r1 = ranges.next()?.trim().split('-').map(|n| n.parse::<usize>().unwrap()).collect_tuple::<(usize, usize)>()?;
	let r2 = ranges.next()?.trim().split('-').map(|n| n.parse::<usize>().unwrap()).collect_tuple::<(usize, usize)>()?;
	Some(Rule {
		name: name,
		range1: RangeInclusive::new(r1.0, r1.1),
		range2: RangeInclusive::new(r2.0, r2.1),
	})
}

fn parse_ticket(s: &str) -> Vec<usize> {
	s.split(',').map(|n| n.parse::<usize>().unwrap()).collect::<Vec<_>>()
}

fn parse(input: &str) -> (Vec<Rule>, Vec<usize>, Vec<Vec<usize>>) {
	let mut lines = input.lines();
	let mut rules = vec![];
	loop {
		let l = lines.next().unwrap();
		if l.is_empty() {
			break;
		}
		rules.push(parse_rule(l).unwrap());
	}
	// Skip one then parse the ticket
	lines.next();
	let my_ticket = parse_ticket(lines.next().unwrap());
	// Skip two then parse nearby tickets
	lines.next();
	lines.next();
	let other_tickets = lines.take_while(|l| !l.is_empty()).map(|l| parse_ticket(l)).collect::<Vec<_>>();

	(rules, my_ticket, other_tickets)
}

pub fn part1(input: String) -> String {
	let (rules, _, other_tickets) = parse(&input);

	other_tickets.iter()
		.flat_map(|t| t.iter())
		.filter(|n| rules.iter().all(|r| !r.is_valid(*n)))
		.sum::<usize>()
		.to_string()
}


fn ordering(mut possibles: Vec<Vec<&Rule>>) -> Vec<&Rule> {
	let mut done = HashSet::new();
	loop {
		if let Some(rname) = possibles.iter().find(|rs| rs.len() == 1 && !done.contains(&rs[0].name)).map(|r| r[0].name.clone()) {
			done.insert(rname.clone());
			for p in possibles.iter_mut() {
				if p.len() > 1 {
					p.retain(|r| r.name != rname);
				}
			}
		} else {
			break
		}
	}
	possibles.into_iter().map(|v| v[0]).collect()
}

pub fn part2(input: String) -> String {
	let (rules, my_ticket, other_tickets) = parse(&input);
	let valid_tickets = other_tickets.into_iter()
		.filter(|t| !t.iter().any(|n| rules.iter().all(|r| !r.is_valid(n))))
		.collect::<Vec<_>>();
	
	let mut possibles = (0..my_ticket.len())
		.map(|i| rules.iter().filter(|r| valid_tickets.iter().all(|t| r.is_valid(&t[i]))).collect::<Vec<_>>())
		.collect::<Vec<_>>();
	
	let final_rules = ordering(possibles);
	// println!("{:?}", final_rules);
	final_rules.iter()
		.enumerate()
		.filter(|(_,r)| r.name.contains("departure"))
		.map(|(i,_)| my_ticket[i])
		.product::<usize>()
		.to_string()
}
