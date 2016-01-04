use self::Instr::*;
use std::collections::HashMap;
use chomp::*;
use std::str;
use chomp::ascii::{skip_whitespace,decimal,signed,is_alpha};
use chomp::primitives::{State, IntoInner, InputClone};

fn reg_instr(i: Input<u8>) -> U8Result<Instr> {
	parse!{i;
		let s = take_while(is_alpha);
				skip_whitespace();
		let r = satisfy(is_alpha);
		ret match s {
			b"hlf" => Hlf(r as char),
			b"tpl" => Tpl(r as char),
			b"inc" => Inc(r as char),
			_ => Err(format!("reg_instr tried to parse {}", str::from_utf8(s).unwrap())),

		}
	}
}

fn jmp_offset(i: Input<u8>) -> U8Result<Instr> {
	parse!{i;
		let s      = take_while(is_alpha);
				     skip_whitespace();
		let o: i32 = signed(decimal);
		ret match s {
			b"jmp" => Jmp(o),
			_ => Err(format!("jmp_offset tried to parse {}", str::from_utf8(s).unwrap())),

		}
	}
}

fn jmp_cond_offset(i: Input<u8>) -> U8Result<Instr> {
	parse!{i;
		let s      = take_while(is_alpha);
				     skip_whitespace();
		let r      = satisfy(is_alpha);
					 string(b", ");
		let o: i32 = signed(decimal);
		ret match s {
			b"jie" => Jie(r as char, o),
			b"jio" => Jio(r as char, o),
			_ => Err(format!("jmp_cond tried to parse {}", str::from_utf8(s).unwrap())),
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

fn instr(i: Input<u8>) -> U8Result<Instr> {
	let instrs = vec![
		jmp_cond_offset as fn(Input<u8>) -> ParseResult<u8, Instr, Error<u8>>,
		jmp_offset as fn(Input<u8>) -> ParseResult<u8, Instr, Error<u8>>,
		reg_instr as fn(Input<u8>) -> ParseResult<u8, Instr, Error<u8>>,
	];
	parse!{i;
		let i = must_one_of(&instrs);
				skip_whitespace();
		ret i
	}
}

fn all_instrs(i: Input<u8>) -> U8Result<Vec<Instr>> {
	parse!{i;
		let v = many1(instr);
		ret v
	}
}

// hlf r sets register r to half its current value, then continues with the next instruction.
// tpl r sets register r to triple its current value, then continues with the next instruction.
// inc r increments register r, adding 1 to it, then continues with the next instruction.
// jmp offset is a jump; it continues with the instruction offset away relative to itself.
// jie r, offset is like jmp, but only jumps if register r is even ("jump if even").
// jio r, offset is like jmp, but only jumps if register r is 1 ("jump if one", not odd).
#[derive(Debug, Clone)]
enum Instr {
	Hlf(char),
	Tpl(char),
	Inc(char),
	Jmp(i32),
	Jie(char, i32),
	Jio(char, i32),
	Err(String),
}

fn run(reg: &mut HashMap<char, usize>, instrs: &Vec<Instr>) {
	let mut i = 0i32;
	loop {
		if i < 0 || i as usize >= instrs.len() {
			break;
		}
		let instr = instrs[i as usize].clone();
		match instr {
			Hlf(c) => {
				*reg.get_mut(&c).unwrap() /= 2;
				i += 1;
			},
			Tpl(c) => {
				*reg.get_mut(&c).unwrap() *= 3;
				i += 1;
			},
			Inc(c) => {
				*reg.get_mut(&c).unwrap() += 1;
				i += 1;
			},
			Jmp(o) => {
				i += o;
			},
			Jie(c, o) => {
				if *reg.get(&c).unwrap() % 2 == 0 {
					i += o;
				} else {
					i += 1;
				}
			},
			Jio(c, o) => {
				if *reg.get(&c).unwrap() == 1 {
					i += o;
				} else {
					i += 1;
				}
			},
			Err(e) => panic!(e),
		}
	}
}

pub fn part1(input: String) -> String {
	let mut reg: HashMap<char, usize> = HashMap::new();
	reg.insert('a', 0);
	reg.insert('b', 0);
	let instrs = parse_only(all_instrs, input.as_bytes()).unwrap();
	run(&mut reg, &instrs);

	format!("{}", reg.get(&'b').unwrap())
}


pub fn part2(input: String) -> String {
	let mut reg: HashMap<char, usize> = HashMap::new();
	reg.insert('a', 1);
	reg.insert('b', 0);
	let instrs = parse_only(all_instrs, input.as_bytes()).unwrap();
	run(&mut reg, &instrs);

	format!("{}", reg.get(&'b').unwrap())
}
