use std::collections::{LinkedList,HashMap};
use std::str;
use chomp::*;
use chomp::ascii::{skip_whitespace,is_alpha,decimal};
use chomp::primitives::{State, IntoInner, InputClone};

enum I {
	Const(u16),
	Name(String),
}

struct Gate {
	output: String,
	inputs: Vec<I>,
	calc: fn(Vec<u16>) -> u16,
}

impl Gate {
	fn satisfied(&self, registers: &HashMap<String, u16>) -> bool {
		for i in &self.inputs {
			match i {
				&I::Const(_) => {},
				&I::Name(ref s)  => {
					if !registers.contains_key(s) {
						return false;
					}
				}
			}
		}
		true
	}

	fn compute(&self, registers: &mut HashMap<String, u16>) {
		let signals: Vec<u16> = self.inputs.iter()
			.map(|input| {
				match input {
					&I::Const(v) => v,
					&I::Name(ref s)  => *registers.get(s).unwrap(),
				}
			})
			.collect();
		let calc = self.calc;
		registers.insert(self.output.clone(), calc(signals));
	}
}

fn    b_and(v: Vec<u16>) -> u16 { v[0] & v[1] }
fn     b_or(v: Vec<u16>) -> u16 { v[0] | v[1] }
fn    b_not(v: Vec<u16>) -> u16 { !v[0] }
fn  b_const(v: Vec<u16>) -> u16 { v[0] }
fn b_lshift(v: Vec<u16>) -> u16 { v[0] << v[1] }
fn b_rshift(v: Vec<u16>) -> u16 { v[0] >> v[1] }

fn in_wire(i: Input<u8>) -> U8Result<I> {
	or(i, |i| parse!{i;
				let n = take_while1(is_alpha);
				ret I::Name(str::from_utf8(n).unwrap().to_string())},
          |i| parse!{i;
				let c: u16 = decimal();
				ret I::Const(c)})
}

fn shift_gate(i: Input<u8>) -> U8Result<Gate> {
	parse!{i;
		let i1    = in_wire();
				    skip_whitespace();
		let instr = take_while1(is_alpha);
				    skip_whitespace();
		let shift: u16 = decimal();
					skip_whitespace();
					string(b"->");
					skip_whitespace();
		let o     = take_while1(is_alpha);
					skip_whitespace();
		ret Gate {
			output: str::from_utf8(o).unwrap().to_string(),
			inputs: vec![i1, I::Const(shift),],
			calc: match str::from_utf8(instr).unwrap() {
				"RSHIFT"  => b_rshift,
				"LSHIFT"  => b_lshift,
				_     => { panic!("shift_gate called with non rshift/lshift"); }
			},
		}
	}
}

fn bin_gate(i: Input<u8>) -> U8Result<Gate> {
	parse!{i;
		let i1    = in_wire();
				    skip_whitespace();
		let instr = take_while1(is_alpha);
				    skip_whitespace();
		let i2    = take_while1(is_alpha);
					skip_whitespace();
					string(b"->");
					skip_whitespace();
		let o     = take_while1(is_alpha);
					skip_whitespace();
		ret Gate {
			output: str::from_utf8(o).unwrap().to_string(),
			inputs: vec![i1, I::Name(str::from_utf8(i2).unwrap().to_string())],
			calc: match str::from_utf8(instr).unwrap() {
				"AND" => b_and,
				"OR"  => b_or,
				_     => { panic!("bin_gate called with non and/or"); }
			},
		}
	}
}

fn mon_gate(i: Input<u8>) -> U8Result<Gate> {
	parse!{i;
		let instr = take_while(is_alpha);
				    skip_whitespace();
		let i1    = in_wire();
					skip_whitespace();
					string(b"->");
					skip_whitespace();
		let o     = take_while(is_alpha);
					skip_whitespace();
		ret Gate {
			output: str::from_utf8(o).unwrap().to_string(),
			inputs: vec![i1],
			calc: match str::from_utf8(instr).unwrap() {
				"NOT" => b_not,
				_     => { panic!("mon_gate called with non not"); }
			},
		}
	}
}

fn const_gate(i: Input<u8>) -> U8Result<Gate> {
	parse!{i;
		let i1    = in_wire();
					skip_whitespace();
					string(b"->");
					skip_whitespace();
		let o     = take_while(is_alpha);
					skip_whitespace();
		ret Gate {
			output: str::from_utf8(o).unwrap().to_string(),
			inputs: vec![i1],
			calc: b_const,
		}
	}
}

fn must_one_of<'a, I, T, E, F>(i: Input<'a, I>, fs: &Vec<F>) -> ParseResult<'a, I, T, E>
  where F: Fn(Input<'a, I>) -> ParseResult<'a, I, T, E> {
	  let res = fs.iter()
	  .map(|f| {
		  f(i.clone()).into_inner()
	  })
	  .find(|state| {
		  match state {
	          &State::Data(_, _)    => true,
	          &State::Error(_, _)   => false,
	          &State::Incomplete(_) => false,
	      }
	  });
	  match res {
		  Some(State::Data(b, d))    => b.ret(d),
		  _ => fs[0](i.clone()),
	  }
}

fn gate(i: Input<u8>) -> U8Result<Gate> {
	let gates = vec![
			const_gate as fn(Input<u8>) -> ParseResult<u8, Gate, Error<u8>>,
			mon_gate as fn(Input<u8>) -> ParseResult<u8, Gate, Error<u8>>,
			bin_gate as fn(Input<u8>) -> ParseResult<u8, Gate, Error<u8>>,
			shift_gate as fn(Input<u8>) -> ParseResult<u8, Gate, Error<u8>>,
		];
	parse!{i;
		let g = must_one_of(&gates);
		ret g
	}
}

fn all_gates(i: Input<u8>) -> U8Result<Vec<Gate>> {
    parse!{i;
        let r = many1(gate);
		ret r
    }
}

pub fn part1(input: String) -> String {
	let mut gates: Vec<Gate> = parse_only(all_gates, input.as_bytes()).unwrap_or(vec![]);
	let mut working_list: LinkedList<Gate> = LinkedList::new();
	loop {
		match gates.pop() {
			None => { break; },
			Some(g) => { working_list.push_back(g); },
		}
	}

	let mut regs: HashMap<String, u16> = HashMap::new();
	loop {
		match working_list.pop_front() {
			None => { break; }
			Some(g) => {
				if g.satisfied(&regs) {
					g.compute(&mut regs);
				} else {
					working_list.push_back(g);
				}
			}
		}
	}
	format!("{}", regs.get("a").unwrap())
}


pub fn part2(input: String) -> String {
	let mut gates: Vec<Gate> = parse_only(all_gates, input.as_bytes()).unwrap_or(vec![]);
	let mut working_list: LinkedList<Gate> = LinkedList::new();
	loop {
		match gates.pop() {
			None => { break; },
			Some(g) => { working_list.push_back(g); },
		}
	}

	let mut regs: HashMap<String, u16> = HashMap::new();
	regs.insert("b".to_string(), 16076u16);

	loop {
		match working_list.pop_front() {
			None => { break; }
			Some(g) => {
				if g.satisfied(&regs) {
					g.compute(&mut regs);
				} else {
					working_list.push_back(g);
				}
			}
		}
	}
	format!("{}", regs.get("a").unwrap())
}
