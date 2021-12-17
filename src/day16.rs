use crate::utils::*;
use itertools::Itertools;

fn parse_hex(s: &str) -> Vec<u8> {
	s.chars().flat_map(|c| c.to_digit(16)).flat_map(|n| to_bits(n as usize, Some(4))).collect()
}

#[derive(Debug, Eq, PartialEq)]
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
				#[cfg(test)]
				println!("{:?} => {} {} Literal", b, version, id);
				let mut n = 0;
				let mut i = 6;
				let mut done = false;
				loop {
					if b[i] == 0 {
						done = true;
					}
					n <<= 4;
					n += from_bits(&b[i+1..i+5]);
					i += 5;
					if done {
						break;
					}
				}
				(Packet::Literal(version, n), i)
			},
			(_, 0) => {
				let size = from_bits(&b[7..22]);
				#[cfg(test)]
				println!("{:?} => {} {} TotalSize {}", b, version, id, size);
				let mut res = vec![];
				let mut i = 22;
				while i < 22+size {
					let (p, next_i) = Packet::from_bitstring(&b[i..22+size]);
					res.push(p);
					i += next_i;
				}
				(Packet::TotalSize(version, id, res), i)
			},
			(_, 1) => {
				let count = from_bits(&b[7..18]);
				#[cfg(test)]
				println!("{:?} => {} {} SubPacketCount {}", b, version, id, count);
				let mut res = vec![];
				let mut i = 18;
				for _ in 0..count {
					let (p, next_i) = Packet::from_bitstring(&b[i..]);
					res.push(p);
					i += next_i;
				}
				(Packet::SubPacketCount(version, id, res), i)
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
	// println!("{:?}", packet);
	packet.visit(&|p: &Packet, cs: &Vec<usize>| p.version() as usize + cs.iter().sum::<usize>()).to_string()
}

fn op_visit(p: &Packet, cs: &Vec<usize>) -> usize {
	use Packet::*;
	match p {
		Literal(_, n) => *n,
		TotalSize(_, 0, _) | SubPacketCount(_, 0, _) => cs.iter().sum::<usize>(),
		TotalSize(_, 1, _) | SubPacketCount(_, 1, _) => cs.iter().product::<usize>(),
		TotalSize(_, 2, _) | SubPacketCount(_, 2, _) => *cs.iter().min().unwrap(),
		TotalSize(_, 3, _) | SubPacketCount(_, 3, _) => *cs.iter().max().unwrap(),
		TotalSize(_, 5, _) | SubPacketCount(_, 5, _) => if cs[0] > cs[1] {1} else {0},
		TotalSize(_, 6, _) | SubPacketCount(_, 6, _) => if cs[0] < cs[1] {1} else {0},
		TotalSize(_, 7, _) | SubPacketCount(_, 7, _) => if cs[0] == cs[1] {1} else {0},
		_ => unreachable!(),
	}
}

pub fn part2(input: String) -> String {
	let bits = parse_hex(&input);
	let (packet, n) = Packet::from_bitstring(&bits);
	// println!("{:?}", packet);
	packet.visit(&op_visit).to_string()
}

#[test]
fn test_examples() {
	use Packet::*;
	let just_literal = "D2FE28";
	let bs = parse_hex(just_literal);
	assert_eq!(vec![1,1,0,1,0,0,1,0,1,1,1,1,1,1,1,0,0,0,1,0,1,0,0,0], bs);
	assert_eq!(Literal(6, 2021), Packet::from_bitstring(&bs).0);

	let op_with_len = "38006F45291200";
	let bs = parse_hex(op_with_len);
	assert_eq!(TotalSize(1, 6, vec![Literal(6, 10), Literal(2, 20)]), Packet::from_bitstring(&bs).0);

	let op_with_packets = "EE00D40C823060";
	let bs = parse_hex(op_with_packets);
	assert_eq!(SubPacketCount(7, 3, vec![Literal(2, 1), Literal(4, 2), Literal(1, 3)]), Packet::from_bitstring(&bs).0);
}

#[test]
fn test_part1() {
	assert_eq!("16", part1("8A004A801A8002F478".to_string()));
	assert_eq!("31", part1("A0016C880162017C3686B18A3D4780".to_string()));
	assert_eq!("12", part1("620080001611562C8802118E34".to_string()));
	assert_eq!("23", part1("C0015000016115A2E0802F182340".to_string()));
}