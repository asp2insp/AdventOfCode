use chomp::*;
use chomp::ascii::{skip_whitespace,decimal};
use self::Spell::*;
use self::Effect::*;

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

#[derive(Clone)]
enum Effect {
	Shielded,
	Poisoned,
	Recharging,
}

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

	fn play(&mut self, s: &Spell) {
		self.apply_effects();
		if self.hp == 0  || self.boss_hp == 0 { // game already over
			return;
		}
		let cost = s.get_cost();
		if cost > self.mana {
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

		// Boss turn
		self.apply_effects();
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
}

pub fn part1(input: String) -> String {
	let (boss_hp, boss_damage) = parse_only(boss, input.as_bytes()).unwrap();
	"part1".to_string()
}


pub fn part2(input: String) -> String {
	"part2".to_string()
}
