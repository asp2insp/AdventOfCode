use chomp::*;
use chomp::ascii::{skip_whitespace,decimal};

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
		ret (hp, d)
	}
}

enum Effect {
	Shield,
	Poison,
	Recharge,
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
		use Spell::*;
		match self {
			MagicMissile => 53,
			Drain => 73,
			Shield => 113,
			Poison => 173,
			Recharge => 229,
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
		}
	}

	fn play(&mut self, s: &Spell) {
		use Spell::*;
		use Effect::*;
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
				self.effects.push((Shield, 6));
			},
			&Poison => {
				self.effects.push((Poison, 6));
			},
			&Recharge => {
				self.effects.push((Recharge, 5));
			},
		}

		// Boss turn
		self.apply_effects();
		let damage = if self.boss_damage <= self.armor {
			1
		} else {
			self.boss_damage - self.armor
		}
		if damage > self.hp {
			self.hp = 0;
		} else {
			self.hp -= damage;
		}
	}

	fn apply_effects(&mut self) {
		use Effect::*;
		self.armor = 0;
		self.effects = self.effects.iter()
			.map(|et| {
				match et.0 {
					&Shield => {
						self.armor += 7;
					},
					&Poison => {
						if self.boss_hp >= 3 {
							self.boss_hp -= 3;
						} else {
							self.boss_hp = 0;
						}
					},
					&Recharge => {
						self.mana += 101;
					},
				};
				(et.1, et.2 - 1)
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
