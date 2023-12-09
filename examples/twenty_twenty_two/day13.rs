use aoc::{make_ord, utils::*};
use itertools::Itertools;

#[derive(PartialEq, Eq, Clone)]
enum Packet {
    List(Vec<Packet>),
    Int(usize),
}

make_ord!(Packet, |this, other| match (this, other) {
    (Packet::Int(l), Packet::Int(r)) => l.cmp(r),
    (Packet::List(l), Packet::List(r)) => l.cmp(r),
    (Packet::List(l), Packet::Int(rs)) => l.cmp(&vec![Packet::Int(*rs)]),
    (Packet::Int(ls), Packet::List(r)) => vec![Packet::Int(*ls)].cmp(r),
});

fn make_packet<'a>(s: &'a [char]) -> (Packet, &'a [char]) {
    if s[0] == '[' {
        let (mut this, next) = get_matched_chunk(s, Some(('[', ']')), Some(','));
        this = &this[1..this.len() - 1]; // Strip start and end brackets
        let mut v = vec![];
        while this.len() > 0 {
            let (item, rest) = make_packet(this);
            this = rest;
            v.push(item);
        }
        (Packet::List(v), next)
    } else {
        let (this, next) = get_matched_chunk(s, Some(('[', ']')), Some(','));
        (
            Packet::Int(String::from_iter(this).parse::<usize>().unwrap()),
            next,
        )
    }
}

impl From<&str> for Packet {
    fn from(s: &str) -> Self {
        make_packet(&s.chars().collect_vec()).0
    }
}

fn parse(s: &str) -> Vec<(Packet, Packet)> {
    gimme_chunks(s)
        .into_iter()
        .map(|pair| (Packet::from(pair[0]), Packet::from(pair[1])))
        .collect()
}

pub fn part1(input: String) -> String {
    parse(&input)
        .into_iter()
        .enumerate()
        .filter(|(_, (l, r))| l < r)
        .map(|(i, _)| i + 1)
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: String) -> String {
    let mut all_packets: Vec<Packet> = parse(&input)
        .into_iter()
        .flat_map(|p| [p.0, p.1])
        .collect_vec();
    let d1 = make_packet(&"[[2]]".to_char_array()).0;
    let d2 = make_packet(&"[[6]]".to_char_array()).0;
    all_packets.push(d1.clone());
    all_packets.push(d2.clone());
    all_packets.sort();
    (((&all_packets).idx(&d1) + 1) * ((&all_packets).idx(&d2) + 1)).to_string()
}
