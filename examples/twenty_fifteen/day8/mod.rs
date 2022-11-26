use chomp::*;
use chomp::ascii::{is_alpha,is_whitespace};
use chomp::primitives::{State, IntoInner, InputClone};

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

enum MemChar {
	Norm(char),
	EscChar,
	EscAscii(char, char),
	Quote,
	Whitespace,
}

fn code_size(mc: &MemChar) -> usize {
	match mc {
		&MemChar::Norm(_) => 1,
		&MemChar::EscChar => 2,
		&MemChar::EscAscii(_,_) => 4,
		&MemChar::Quote => 1,
		&MemChar::Whitespace => 0,
	}
}

fn mem_size(mc: &MemChar) -> usize {
	match mc {
		&MemChar::Norm(_) => 1,
		&MemChar::EscChar => 1,
		&MemChar::EscAscii(_,_) => 1,
		&MemChar::Quote => 0,
		&MemChar::Whitespace => 0,
	}
}

fn repr_size(mc: &MemChar) -> usize {
	match mc {
		&MemChar::Norm(_) => 1,
		&MemChar::EscChar => 4,
		&MemChar::EscAscii(_,_) => 5,
		&MemChar::Quote => 3,
		&MemChar::Whitespace => 0,
	}
}

fn ignored_char(i: Input<u8>) -> U8Result<MemChar> {
	or(i, |i| parse!{i;
				token(b'"');
				ret MemChar::Quote
			},
          |i| parse!{i;
	  			satisfy(is_whitespace);
	  			ret MemChar::Whitespace
	  		})
}

fn normal_char(i: Input<u8>) -> U8Result<MemChar> {
	parse!{i;
		let i1    = satisfy(is_alpha);
		ret MemChar::Norm(i1 as char)
	}
}

fn esc_char(i: Input<u8>) -> U8Result<MemChar> {
	or(i, |i| parse!{i;
			string(b"\\\"");
			ret MemChar::EscChar
		  },
		  |i| parse!{i;
		  	string(b"\\\\");
		  	ret MemChar::EscChar
		  })
}

fn is_hex(i: u8) -> bool {
	match i {
		b'0'...b'9' => true,
		b'a'...b'f' => true,
		b'A'...b'F' => true,
		_        => false,
	}
}

fn esc_ascii(i: Input<u8>) -> U8Result<MemChar> {
	parse!{i;
					string(b"\\x");
		let i1    = satisfy(is_hex);
		let i2    = satisfy(is_hex);
		ret MemChar::EscAscii(i1 as char, i2 as char)
	}
}

fn chars(i: Input<u8>) -> U8Result<MemChar> {
	let char_types = vec![
			esc_char as fn(Input<u8>) -> ParseResult<u8, MemChar, Error<u8>>,
			normal_char as fn(Input<u8>) -> ParseResult<u8, MemChar, Error<u8>>,
			esc_ascii as fn(Input<u8>) -> ParseResult<u8, MemChar, Error<u8>>,
			ignored_char,
		];
	parse!{i;
		let g = must_one_of(&char_types);
		ret g
	}
}

fn all_chars(i: Input<u8>) -> U8Result<Vec<MemChar>> {
    parse!{i;
        let r = many1(chars);
		ret r
    }
}

pub fn part1(input: String) -> String {
	let parsed = parse_only(all_chars, input.as_bytes()).unwrap();
	let code_total = parsed.iter().fold(0usize, |sum, mc| sum + code_size(mc));
	let mem_total = parsed.iter().fold(0usize, |sum, mc| sum + mem_size(mc));
	format!("code {}, mem {}", code_total, mem_total)
}


pub fn part2(input: String) -> String {
	let parsed = parse_only(all_chars, input.as_bytes()).unwrap();
	let code_total = parsed.iter().fold(0usize, |sum, mc| sum + code_size(mc));
	let repr_total = parsed.iter().fold(0usize, |sum, mc| sum + repr_size(mc));
	format!("code {}, repr {}", code_total, repr_total)
}
