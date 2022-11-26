use chomp::*;
use chomp::ascii::{skip_whitespace,decimal};
use self::Spell::*;
use self::Effect::*;
use rand::distributions::{IndependentSample, Range};
use rand;

fn desc(i: Input<u8>) -> U8Result<char> {
	parse!{i;
		         		take_while(|c| c != b':');
				 		token(b':');
						ret ':'
	}
}

fn boss(i: Input<u8>) -> U8Result<(usize, usize)> {
	parse!{i;
		         		desc();
				 	    skip_whitespace();
		let hp: usize = decimal();
						skip_whitespace();
						desc();
						skip_whitespace();
		let d: usize  = decimal();
		ret (hp, d)
	}
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Effect {
	Shielded,
	Poisoned,
	Recharging,
}

#[derive(Clone, Debug)]
enum Spell {
	MagicMissile,
	Drain,
	Shield,
	Poison,
	Recharge,
}

impl Spell {
	fn get_cost(&self) -> usize {
		match self {
			&MagicMissile => 53,
			&Drain => 73,
			&Shield => 113,
			&Poison => 173,
			&Recharge => 229,
		}
	}
}

#[derive(Debug)]
struct Player {
	hp: usize,
	mana: usize,
	effects: Vec<(Effect, usize)>,
	armor: usize,
	boss_hp: usize,
	boss_damage: usize,
	total_mana: usize,
}

impl Player {
	fn new(boss_hp: usize, boss_damage: usize) -> Player {
		Player {
			hp: 50,
			mana: 500,
			effects: vec![],
			boss_hp: boss_hp,
			boss_damage: boss_damage,
			total_mana: 0,
			armor: 0,
		}
	}

	fn can_play_spell(&self, s: &Spell) -> bool {
		if s.get_cost() > self.mana {
			false
		} else {
			match s {
				&Shield => self.effects.iter().fold(true, |b, e| b && (e.0 != Shielded || e.1 <= 1)),
				&Poison => self.effects.iter().fold(true, |b, e| b && (e.0 != Poisoned || e.1 <= 1)),
				&Recharge => self.effects.iter().fold(true, |b, e| b && (e.0 != Recharging || e.1 <= 1)),
				_ => true,
			}
		}
	}

	fn my_turn(&mut self, s: &Spell) {
		let cost = s.get_cost();
		if !self.can_play_spell(s) { // Running out or doubling effect means losing
			self.hp = 0;
			return;
		}

		self.mana -= cost;
		self.total_mana += cost;
		match s {
			&MagicMissile => {
				if self.boss_hp >= 4 {
					self.boss_hp -= 4;
				} else {
					self.boss_hp = 0
				}
			},
			&Drain => {
				if self.boss_hp >= 2 {
					self.boss_hp -= 2;
				} else {
					self.boss_hp = 0
				}
				self.hp += 2;
			},
			&Shield => {
				self.effects.push((Shielded, 6));
			},
			&Poison => {
				self.effects.push((Poisoned, 6));
			},
			&Recharge => {
				self.effects.push((Recharging, 5));
			},
		}
	}

	fn boss_turn(&mut self) {
		let damage = if self.boss_damage <= self.armor {
			1
		} else {
			self.boss_damage - self.armor
		};
		if damage > self.hp {
			self.hp = 0;
		} else {
			self.hp -= damage;
		}
	}

	fn play(&mut self, s: &Spell) {
		// My turn
		self.apply_effects();
		if self.hp == 0  || self.boss_hp == 0 { // game already over
			return;
		}
		self.my_turn(s);

		// Boss turn
		self.apply_effects();
		if self.hp == 0  || self.boss_hp == 0 { // game already over
			return;
		}
		self.boss_turn();
	}

	fn play2(&mut self, s: &Spell) {
		// My turn
		if self.hp < 2 { // game over
			return;
		} else {
			self.hp -= 1; // Hard mode
		}
		self.apply_effects();
		if self.hp == 0  || self.boss_hp == 0 { // game already over
			return;
		}
		self.my_turn(s);

		// Boss turn
		self.apply_effects();
		if self.hp == 0  || self.boss_hp == 0 { // game already over
			return;
		}
		self.boss_turn();
	}

	fn apply_effects(&mut self) {
		self.armor = 0;
		for et in &self.effects {
			match &et.0 {
				&Shielded => {
					self.armor += 7;
				},
				&Poisoned => {
					if self.boss_hp >= 3 {
						self.boss_hp -= 3;
					} else {
						self.boss_hp = 0;
					}
				},
				&Recharging => {
					self.mana += 101;
				},
			};
		}
		self.effects = self.effects.iter()
			.map(|et| {
				(et.0.clone(), et.1 - 1)
			})
			.filter(|et| et.1 > 0)
			.collect();
	}

	fn still_going(&self) -> bool {
		self.hp > 0 && self.boss_hp > 0
	}
}

struct SpellChooser {
	rng: rand::ThreadRng,
	spells: Vec<Spell>,
	between: Range<usize>,
}

impl SpellChooser {
	fn new() -> SpellChooser {
		SpellChooser {
			rng: rand::thread_rng(),
			spells: vec![MagicMissile,
		 				  Drain,
						  Shield,
						  Poison,
						  Recharge,],
			between: Range::new(0, 5),
		}
	}

	fn next(&mut self) -> Spell {
		self.spells[self.between.ind_sample(&mut self.rng)].clone()
	}
}

fn seed(boss_hp: usize, boss_damage: usize, use_2: bool) -> Vec<Spell> {
	let mut spc = SpellChooser::new();
	loop {
		let mut p = Player::new(boss_hp, boss_damage);
		let mut v: Vec<Spell> = Vec::new();
		while p.still_going() {
			let mut next = spc.next();
			let mut retries = 5;
			while retries > 0 && !p.can_play_spell(&next) {
				next = spc.next();
				retries -= 1;
			}
			if use_2 {
				p.play2(&next);
			} else {
				p.play(&next);
			}
			v.push(next);
		}
		// println!("Seed: {:?} => \n\t {:?}", v, p);
		// Keep trying until we get a winning combo
		if p.hp > 0 && p.boss_hp == 0 {
			return v
		}
	}
}

fn gen_pop(n: usize, boss: &(usize, usize), use_2: bool) -> Vec<Vec<Spell>> {
	let mut v: Vec<Vec<Spell>> = vec![];
	for i in 0..n {
		v.push(seed(boss.0, boss.1, use_2));
	}
	v
}

fn gen_pop2(n: usize, boss: &(usize, usize), use_2: bool) -> Vec<Vec<Spell>> {
	let mut v: Vec<Vec<Spell>> = vec![];
	let seed = vec![Shield, Recharge, Poison, Shield, Recharge,
					Poison, Shield, Recharge, Poison, Shield,
					MagicMissile, Poison, MagicMissile];
	for i in 1..n {
		v.push(mutate(&seed, 50));
	}
	v.push(seed);
	v
}

/// Mutate will mutate each element with probability percentage equal
/// to the mut_rate. If it chooses to mutate, one of three mutations will
/// take place:
/// Delete -- removes the element
/// Insert -- insert a new random Spell
/// Swap -- exchanges the spell for a random one
fn mutate(v: &Vec<Spell>, mut_rate: usize) -> Vec<Spell> {
	let mut new = v.clone();
	let mut rng = rand::thread_rng();
	let choices = Range::new(0, 3);
	let percent = Range::new(0, 100);
	let mut spc = SpellChooser::new();
	let mut i = 0;
	while i < new.len() {
		if percent.ind_sample(&mut rng) < mut_rate { // Should mutate this index
			match choices.ind_sample(&mut rng) {
				0 => {
					new.remove(i);
				},
				1 => {
					new.insert(i, spc.next());
				},
				2 => {
					new[i] = spc.next();
				},
				_ => {},
			};
		}
		i += 1;
	}
	new
}

/// Expectation: Receives a sorted vector
fn run_generation(current: Vec<Vec<Spell>>,
	                top_n: usize,
				   rand_n: usize,
				 mut_rate: usize,
				     boss: (usize, usize),
				    use_2: bool) -> Vec<Vec<Spell>> {
	let mut v: Vec<Vec<Spell>> = Vec::new();
	for i in 0..top_n { // Take the top_n best
		v.push(current[i].clone());
	}
	for i in 0..(current.len() - top_n - rand_n) { // Fill the generation, reserving space for rand
		v.push(mutate(&current[i % top_n], mut_rate)); // Sample a survivor and mutate it

		// TODO implement crossover
	}
	v.append(&mut gen_pop(rand_n, &boss, use_2)); // Finish by adding the random pop
	v.sort_by(|a, b| total_cost(a, &boss, use_2).cmp(&total_cost(b, &boss, use_2)));
	v
}

fn total_cost(v: &Vec<Spell>, boss: &(usize, usize), use_2: bool) -> usize {
	let mut p = Player::new(boss.0, boss.1);
	for s in v {
		if !p.can_play_spell(s) { // cut out bad combos early
			break;
		}
		if use_2 {
			p.play2(s);
		} else {
			p.play(s);
		}
	}
	if p.hp == 0 || p.boss_hp > 0 { // Penalize losses or incompletes
		10_000
	} else {
		p.total_mana
	}
}

pub fn part1(input: String) -> String {
	let boss = parse_only(boss, input.as_bytes()).unwrap();
	let mut curr = gen_pop(10, &boss, false);
	for _ in 0..90 {
		curr.push(vec![]);
	}
	curr.sort_by(|a, b| total_cost(a, &boss, false).cmp(&total_cost(b, &boss, false)));
	println!("Seeded");
	for i in 0..11 {
		if i % 10 == 0 {
			println!("Gen {} best: {} \n {:?}", i, total_cost(&curr[0], &boss, false), &curr[0]);
		}
		curr = run_generation(curr, 10, 1, 10, boss, false);
	}
	format!("{}", total_cost(&curr[0], &boss, false))
}

pub fn part2(input: String) -> String {
	let boss = parse_only(boss, input.as_bytes()).unwrap();
	let mut curr = gen_pop2(1000, &boss, true);
	curr.sort_by(|a, b| total_cost(a, &boss, true).cmp(&total_cost(b, &boss, true)));
	println!("Seeded");
	for i in 0..501 {
		if i % 100 == 0 {
			println!("Gen {} best: {} \n {:?}", i, total_cost(&curr[0], &boss, true), &curr[0]);
		}
		curr = run_generation(curr, 100, 0, 10, boss, true);
	}

	// Play through winning move:
	// let mut p = Player::new(boss.0, boss.1);
	// println!("Winner: {:?}", p);
	// for s in &curr[0] {
	// 	p.play2(s);
	// 	println!("Winner: {:?}", p);
	// }

	format!("{}", total_cost(&curr[0], &boss, true))
}
