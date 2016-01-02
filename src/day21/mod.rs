use chomp::*;
use chomp::ascii::{skip_whitespace,decimal};

static WEAPONS: &'static [(usize, usize, usize)] = &[(8, 4, 0), (10, 5, 0), (25, 6, 0), (40, 7, 0), (74, 8, 0)];
static ARMOR: &'static [(usize, usize, usize)] = &[(13, 0, 1), (31, 0, 2), (53, 0, 3), (75, 0, 4), (102, 0, 5), (0, 0, 0)];
static RINGS: &'static [(usize, usize, usize)] = &[(25, 1, 0), (50, 2, 0), (100, 3, 0), (20, 0, 1), (40, 0, 2), (80, 0, 3), (0, 0, 0), (0, 0, 0)];

fn combos() -> Vec<Vec<(usize, usize, usize)>> {
	let mut v: Vec<Vec<(usize, usize, usize)>> = Vec::new();
	for w in WEAPONS {
		for a in ARMOR {
			for ir in RINGS.iter().enumerate() {
				for jr in RINGS.iter().enumerate() {
					if jr.0 != ir.0 {
						v.push(vec![w.clone(), a.clone(), ir.1.clone(), jr.1.clone()]);
					}
				}
			}
		}
	}
	v
}

fn merge(c: &Vec<(usize, usize, usize)>) -> (usize, usize, usize) {
	c.iter().fold((0, 0, 0), |sum, comb| {
		(sum.0 + comb.0,
		 sum.1 + comb.1,
	     sum.2 + comb.2)
	})
}

fn damage(d: usize, a: usize) -> usize {
	if a >= d {
		1
	} else {
		d - a
	}
}

// Combo = (Cost, Damage, Armor)
fn wins_game(combo: &Vec<(usize, usize, usize)>, hp: usize, d: usize, a: usize) -> bool {
	let c = merge(combo);
	let boss_damage = damage(d, c.2);
	let my_damage = damage(c.1, a);

	let mut my_hp = 100;
	let mut boss_hp = hp;

	loop {
		if my_damage >= boss_hp {
			return true;
		} else if boss_damage >= my_hp {
			return false;
		}
		boss_hp -= my_damage;
		my_hp -= boss_damage;
	}
}

fn desc(i: Input<u8>) -> U8Result<char> {
	parse!{i;
		         		take_while(|c| c != b':');
				 		token(b':');
						ret ':'
	}
}

fn boss(i: Input<u8>) -> U8Result<(usize, usize, usize)> {
	parse!{i;
		         		desc();
				 	    skip_whitespace();
		let hp: usize = decimal();
						skip_whitespace();
						desc();
						skip_whitespace();
		let d: usize  = decimal();
						skip_whitespace();
						desc();
						skip_whitespace();
		let a: usize  = decimal();
						skip_whitespace();
		ret (hp, d, a)
	}
}

pub fn part1(input: String) -> String {
	let (hp, d, a) = parse_only(boss, input.as_bytes()).unwrap();
	let cost = combos().iter()
		.filter(|c| wins_game(c, hp, d, a))
		.fold(4096, |m, c| {
			let cost = merge(c).0;
			if cost < m {
				cost
			} else {
				m
			}
		});
	format!("{:?}", cost)
}


pub fn part2(input: String) -> String {
	let (hp, d, a) = parse_only(boss, input.as_bytes()).unwrap();
	let cost = combos().iter()
		.filter(|c| !wins_game(c, hp, d, a))
		.fold(0, |m, c| {
			let cost = merge(c).0;
			if cost > m {
				cost
			} else {
				m
			}
		});
	format!("{:?}", cost)
}
