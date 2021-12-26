use itertools::*;
use num::iter::RangeInclusive;
use regex::*;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Instr {
    on_off: bool,
    reg: Region,
}

impl Instr {
    fn count(&self) -> isize {
        let m = if self.on_off { 1 } else { -1 };
        self.reg.count() as isize * m
    }

    fn with(&self, r: &Region) -> Instr {
        Instr {
            on_off: self.on_off,
            reg: r.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Region {
    x: std::ops::RangeInclusive<isize>,
    y: std::ops::RangeInclusive<isize>,
    z: std::ops::RangeInclusive<isize>,
}

impl Region {
    fn is_empty(&self) -> bool {
        self.x.is_empty() || self.y.is_empty() || self.z.is_empty()
    }

    fn intersects(&self, other: &Region) -> bool {
        !self.get_overlap(other).is_empty()
    }

    fn get_overlap(&self, other: &Region) -> Region {
        Region {
            x: *self.x.start().max(other.x.start())..=*self.x.end().min(other.x.end()),
            y: *self.y.start().max(other.y.start())..=*self.y.end().min(other.y.end()),
            z: *self.z.start().max(other.z.start())..=*self.z.end().min(other.z.end()),
        }
    }

    fn count(&self) -> isize {
        (self.x.clone().count() * self.y.clone().count() * self.z.clone().count()) as isize
    }
}

fn parse(s: &str) -> Vec<Instr> {
    let re = Regex::new(r"(on|off) x=([-\d]+)..([-\d]+),y=([-\d]+)..([-\d]+),z=([-\d]+)..([-\d]+)")
        .unwrap();
    re.captures_iter(s)
        .map(|cap| Instr {
            on_off: if &cap[1] == "on" { true } else { false },
            reg: Region {
                x: parse!(cap[2], isize)..=parse!(cap[3], isize),
                y: parse!(cap[4], isize)..=parse!(cap[5], isize),
                z: parse!(cap[6], isize)..=parse!(cap[7], isize),
            },
        })
        .collect_vec()
}

fn execute(v: &mut HashMap<(isize, isize, isize), bool>, i: &Instr) {
    for x in i.reg.x.clone() {
        if x < -50 || x > 50 {
            continue;
        }
        for y in i.reg.y.clone() {
            if y < -50 || y > 50 {
                continue;
            }
            for z in i.reg.z.clone() {
                if z < -50 || z > 50 {
                    continue;
                }
                v.insert((x, y, z), i.on_off);
            }
        }
    }
}

pub fn part1(input: String) -> String {
    let mut map = HashMap::new();
    parse(&input).iter().for_each(|i| {
        execute(&mut map, i);
    });
    map.values().filter(|v| **v).count().to_string()
}

fn how_many_are_on(instrs: &[Instr], up_to: isize, reg: Region, hoard: &mut HashMap<(isize, Region), isize>) -> isize {
	if up_to == -1 || reg.is_empty() {
		return 0
	}
    if hoard.contains_key(&(up_to, reg.clone())) {
        return hoard[&(up_to, reg.clone())]
    }
	let mut total = 0;
	for i in 0..=up_to {
		let region = instrs[i as usize].reg.clone().get_overlap(&reg);
		if instrs[i as usize].on_off {
            #[cfg(test)]
			println!(">> [{}]: +{}", i, region.count());
			total += region.count();
		}
		let below = how_many_are_on(&instrs, i-1 as isize, region, hoard);
		total -= below;
        #[cfg(test)]
        {
            println!(">> [{}]: -{}", i, below);
            println!(">> [{}]: {}", i, total);
        }
	}
    hoard.insert((up_to, reg), total);
	total
}

pub fn part2(input: String) -> String {
    let mut instructions: Vec<Instr> = vec![];
    let mut count: isize = 0;
    let input = parse(&input);


	let all = Region {
		x: isize::MIN..=isize::MAX,
		y: isize::MIN..=isize::MAX,
		z: isize::MIN..=isize::MAX,
	};
	how_many_are_on(&input, input.len() as isize -1, all, &mut HashMap::new()).to_string()
}

//  111111        l1 + l2 - (l1 ^ l1)
//     1111111
//     0000       l

#[test]
fn test_overlap() {
    let r1 = 3isize..=5isize;
    let r2 = 6isize..=10isize;
    assert_eq!(
        0,
        (*r1.start().max(r2.start())..=*r1.end().min(r2.end()))
            .into_iter()
            .count()
    );
}

#[test]
fn test_given() {
    let s = r"on x=10..12,y=10..12,z=10..12
	on x=11..13,y=11..13,z=11..13
	off x=9..11,y=9..11,z=9..11
	on x=10..10,y=10..10,z=10..10";

    assert_eq!("39", part2(s.to_string()));
}
