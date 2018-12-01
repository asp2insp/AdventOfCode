use chomp::prelude::{U8Input, SimpleResult, parse_only, token, string, many1};
use chomp::ascii::{decimal, skip_whitespace};

use self::Instruction::*;

#[derive(Debug, Clone)]
enum Dest {
	Bot(usize),
	Output(usize),
}

#[derive(Debug, Clone)]
enum Instruction {
	Input {
		bot: usize,
		value: usize,
	},
	Program {
		bot: usize,
		low: Dest,
		high: Dest,
	},
}

#[derive(Debug, Default, Clone)]
struct Bot {
	nums: Vec<usize>,
	program: Option<Instruction>,
}

fn input<I: U8Input>(i: I) -> SimpleResult<I, Instruction> {
	parse!{i;
				string(b"value ");
		let n = decimal();
				string(b" goes to bot ");
		let b = decimal();
		ret Input {
			bot: b,
			value: n,
		}
	}
}

fn bot_dest<I: U8Input>(i: I) -> SimpleResult<I, Dest> {
	parse!{i;
				string(b"bot ");
		let b = decimal();
		ret Dest::Bot(b)
	}
}

fn out_dest<I: U8Input>(i: I) -> SimpleResult<I, Dest> {
	parse!{i;
				string(b"output ");
		let b = decimal();
		ret Dest::Output(b)
	}
}

fn program<I: U8Input>(i: I) -> SimpleResult<I, Instruction> {
	parse!{i;
				string(b"bot ");
		let b = decimal();
				string(b" gives low to ");
		let low = bot_dest() <|> out_dest();
				string(b" and high to ");
		let high = bot_dest() <|> out_dest();
		ret Program {
			bot: b,
			low: low,
			high: high,
		}
	}
}

fn instr<I: U8Input>(i: I) -> SimpleResult<I, Instruction> {
	parse!{i;
		let l = program() <|> input();
				skip_whitespace();
		ret l
	}
}

fn all_instrs<I: U8Input>(i: I) -> SimpleResult<I, Vec<Instruction>> {
	parse!{i;
		let v = many1(instr);
		ret v
	}
}

pub fn part1(input: String) -> String {
	let instructions = parse_only(all_instrs, input.as_bytes()).unwrap();
	let mut bots = vec![];
	for i in instructions {
		match i {
			Input {bot: b, value: n} => {
				if b >= bots.len() {
					bots.resize(b+1, Bot::default());
				}
				bots[b].nums.push(n);
			},
			Program { bot, .. } => {
				if bot >= bots.len() {
					bots.resize(bot+1, Bot::default());
				}
				bots[bot].program = Some(i);
			},
		}
	}
	let mut i = 0;
	let mut outputs = vec![];
	loop {
		i = (i + 1) % bots.len();
		if bots[i].nums.len() != 2 {
			continue
		}
		let low_num = *bots[i].nums.iter().min().unwrap();
		let high_num = *bots[i].nums.iter().max().unwrap();
		if low_num == 17 && high_num == 61 {
			return format!("{}", i);
		}
		let p = bots[i].program.clone();
		if let Some(Program {ref low, ref high, .. }) = p {
			match low {
				&Dest::Bot(b) => bots[b].nums.push(low_num),
				&Dest::Output(o) => {
					if o <= 3 {
						outputs.push(o);
					}
				},
			};
			match high {
				&Dest::Bot(b) => bots[b].nums.push(high_num),
				&Dest::Output(o) => {
					if o <= 3 {
						outputs.push(o);
					}
				},
			};
		} else {
			unimplemented!()
		}
		bots[i].nums.clear();
	}
}


pub fn part2(input: String) -> String {
	let instructions = parse_only(all_instrs, input.as_bytes()).unwrap();
	let mut bots = vec![];
	for i in instructions {
		match i {
			Input {bot: b, value: n} => {
				if b >= bots.len() {
					bots.resize(b+1, Bot::default());
				}
				bots[b].nums.push(n);
			},
			Program { bot, .. } => {
				if bot >= bots.len() {
					bots.resize(bot+1, Bot::default());
				}
				bots[bot].program = Some(i);
			},
		}
	}
	let mut i = 0;
	let mut outputs = vec![];
	loop {
		i = (i + 1) % bots.len();
		if bots[i].nums.len() != 2 {
			continue
		}
		let low_num = *bots[i].nums.iter().min().unwrap();
		let high_num = *bots[i].nums.iter().max().unwrap();
		let p = bots[i].program.clone();
		if let Some(Program {ref low, ref high, .. }) = p {
			match low {
				&Dest::Bot(b) => bots[b].nums.push(low_num),
				&Dest::Output(o) => {
					if o <= 2 {
						outputs.push(low_num);
					}
				},
			};
			match high {
				&Dest::Bot(b) => bots[b].nums.push(high_num),
				&Dest::Output(o) => {
					if o <= 2 {
						outputs.push(high_num);
					}
				},
			};
		} else {
			unimplemented!()
		}
		bots[i].nums.clear();
		if outputs.len() == 3 {
			return format!("{}", outputs.iter().fold(1, |p, i| p * *i))
		}
	}
}
