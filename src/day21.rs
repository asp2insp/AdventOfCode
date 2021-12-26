use crate::utils::*;
use std::collections::HashMap;

// Player 1 starting position: 4
// Player 2 starting position: 2

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct CappedAmt {
	val: usize,
	cap: usize,
}

impl std::ops::AddAssign<usize> for CappedAmt {
	fn add_assign(&mut self, rhs: usize) {
		for _ in 0..rhs {
			self.val += 1;
			if self.val == self.cap+1 {
				self.val = 1;
			}
		}
	}
}

impl std::ops::Deref for CappedAmt {
	type Target = usize;
	fn deref(&self) -> &usize {
		&self.val
	}
}

fn m(mut pos: usize, amt: usize, cap: usize) -> usize {
	for _ in 0..amt {
		pos += 1;
		if pos == cap+1 {
			pos = 1;
		}
	}
	pos
}

pub fn part1(input: String) -> String {
	let mut players = [CappedAmt{val: 4, cap: 10}, CappedAmt{val: 2, cap: 10}];
	let mut scores = [0, 0];
	let mut d = CappedAmt{val: 0, cap: 100};
	let mut rolls = 0;
	let mut p = 2;
	loop {
		p = toggle(p, 1, 2);
		for _ in 0..3 {
			rolls += 1;
			d += 1;
			players[p-1] += *d;
		}
		scores[p-1] += *players[p-1];

		// println!("Player {} moves to space {} for a total score of {}", p, *players[p-1], scores[p-1]);
		
		if scores[p-1] >= 1000 {
			// Game over
			let loser = toggle(p, 1, 2) - 1;
			println!("{} * {}", rolls, scores[loser]);
			return (rolls * scores[loser]).to_string()
		}
	}
	unreachable!()
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct GameState {
	turn: usize,
	roll: usize,
	positions: [CappedAmt; 2],
	scores: [usize; 2],
}

impl GameState {
	fn with_roll(&self, d: usize) -> GameState {
		let roll = (self.roll + 1) % 3;
		let mut positions = self.positions;
		let mut turn = self.turn;
		let mut scores = self.scores;
		positions[turn] += d;
		if roll == 0 {
			scores[turn] += *positions[turn];
			turn = toggle(turn, 0, 1);
		}
		GameState {
			turn,
			roll,
			positions,
			scores,
		}
	}
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct Wins (usize, usize);

impl std::ops::AddAssign for Wins {
	fn add_assign(&mut self, rhs: Self) {
		self.0 += rhs.0;
		self.1 += rhs.1;
	}
}

fn run_dirac(state: GameState, hoard: &mut HashMap<GameState, Wins>) -> Wins {
	if hoard.contains_key(&state) {
		return hoard[&state]
	}
	let ret = if state.scores[0] >= 21 {
		Wins(1, 0)
	} else if state.scores[1] >= 21 {
		Wins(0, 1)
	} else {
		(1..=3).map(|d| {
			run_dirac(state.with_roll(d), hoard)
		})
		.fold(Wins(0, 0), |mut a, b| {
			a += b;
			a
		})
	};
	hoard.insert(state, ret);
	ret
}


pub fn part2(input: String) -> String {
	let mut players = [CappedAmt{val: 4, cap: 10}, CappedAmt{val: 2, cap: 10}];
	let init = GameState {
		turn: 0,
		roll: 0,
		positions: players,
		scores: [0, 0],
	};
	format!("{:?}", run_dirac(init, &mut HashMap::new()))
}
