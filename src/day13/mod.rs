use std::str;
use chomp::*;
use chomp::ascii::{skip_whitespace,decimal,is_alpha};

fn gain_or_lose(i: Input<u8>) -> U8Result<i64> {
	or(i,
		|i| parse!{i;
			string(b"gain");
			ret 1i64
		},
		|i| parse!{i;
			string(b"lose");
			ret -1i64
		})
}

fn alpha_string(i: Input<u8>) -> U8Result<String> {
	parse!{i;
		let s = take_while(is_alpha);
		ret str::from_utf8(s).unwrap().to_string()
	}
}

fn happy(i: Input<u8>) -> U8Result<(String, String, i64)> {
	parse!{i;
		let n1 = alpha_string();
				 skip_whitespace();
				 string(b"would");
				 skip_whitespace();
		let pm = gain_or_lose();
				 skip_whitespace();
		let hu: i64 = decimal();
				 skip_whitespace();
				 string(b"happiness units by sitting next to");
				 skip_whitespace();
		let n2 = alpha_string();
				 token(b'.');
				 skip_whitespace();
		ret (n1, n2, hu*pm)
	}
}

fn all_happy(i: Input<u8>) -> U8Result<Vec<(String, String, i64)>> {
	parse!{i;
		let v = many1(happy);
		ret v
	}
}


pub fn part1(input: String) -> String {
	format!("{:?}", parse_only(all_happy, input.as_bytes()))
}


pub fn part2(input: String) -> String {
	"part2".to_string()
}
