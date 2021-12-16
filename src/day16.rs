use crate::utils::*;
use itertools::Itertools;

fn parse_hex(s: &str) -> Vec<u8> {
	s.chars().flat_map(|c| c.to_digit(16)).flat_map(|n| to_bits(n as usize)).collect()
}

enum Packet {
	Literal(u8, usize),
	TotalSize(u8, usize, Vec<Packet>),
	SubPacketCount(u8, usize, Vec<Packet>),
}

impl Packet {
	fn from_bitstring(b: &[u8]) -> (Packet, usize) {
		let version = from_bits(&b[0..=2]) as u8;
		let id = from_bits(&b[3..=5]);
		match (id, b[6]) {
			(4, _) => {
				let mut n = 0;
				let mut i = 6;
				let mut done = false;
				while !done {
					n <<= 4;
					n += from_bits(&b[i+1..i+5]);
					i += 5;
					if done {
						break;
					}
					if b[i] == 0 {
						done = true;
					}
				}
				while i % 4 != 0 {
					i += 1;
				}
				(Packet::Literal(version, n), i)
			},
			(_, 0) => {
				let size = from_bits(&b[7..22]);
				let mut res = vec![];
				let mut i = 7;
				while i < size {
					let (p, next_i) = Packet::from_bitstring(&b[i..]);
					res.push(p);
					i = next_i;
				}
				(Packet::TotalSize(version, size, res), size) 
			},
			(_, 1) => {
				let count = from_bits(&b[7..18]);
				let mut res = vec![];
				let mut i = 7;
				for _ in 0..count {
					let (p, next_i) = Packet::from_bitstring(&b[i..]);
					i = next_i;
				}
				(Packet::SubPacketCount(version, count, res), i)
			},
			_ => unimplemented!(),
		}
	}

	fn version(&self) -> u8 {
		match self {
			Packet::Literal(v, _) => *v,
			Packet::TotalSize(v, _, _) => *v,
			Packet::SubPacketCount(v, _, _) => *v,
		}
	}

	fn visit<T>(&self, f: &dyn Fn(&Packet, &Vec<T>) -> T) -> T {
		match self {
			Packet::Literal(_, _) => f(self, &vec![]),
			Packet::TotalSize(_, _, child) => f(self, &child.iter().map(|i| i.visit(f)).collect_vec()),
			Packet::SubPacketCount(_, _, child) => f(self, &child.iter().map(|i| i.visit(f)).collect_vec()),
		}
	}
}

pub fn part1(input: String) -> String {
	let bits = parse_hex(&input);
	let (packet, n) = Packet::from_bitstring(&bits);
	assert_eq!(n, bits.len());
	packet.visit(&|p: &Packet, cs: &Vec<usize>| p.version() as usize + cs.iter().sum::<usize>()).to_string()
}


pub fn part2(input: String) -> String {
	"part2".to_string()
}
