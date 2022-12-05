use aoc::utils::{parse_nums_from_lines, transpose};
use itertools::Itertools;

struct Instr {
    how_many: isize,
    from: usize,
    to: usize,
}

// every 4th, starting from 1
fn parse(s: &str) -> (Vec<Vec<char>>, Vec<Instr>) {
    let mut lines = s.lines();
    let mut map: Vec<Vec<char>> = Vec::new();
    while let Some(l) = lines.next() {
        if l.trim().is_empty() {
            map.pop(); // remove index line
            break;
        }
        map.push(l.chars().skip(1).step_by(4).collect_vec());
    }
    map = transpose(&map);
    map.iter_mut().for_each(|c| {
        c.reverse();
        while let Some(' ') = c.last() {
            c.pop();
        }
    });
    (
        map,
        parse_nums_from_lines(lines)
            .into_iter()
            .map(|ns| Instr {
                how_many: ns[0],
                from: ns[1] as usize - 1,
                to: ns[2] as usize - 1,
            })
            .collect_vec(),
    )
}

pub fn part1(input: String) -> String {
    let (mut map, instrs) = parse(&input);
    // println!("{:?}", map);
    for i in instrs {
        for _c in 0..i.how_many {
            let e = map[i.from].pop().unwrap();
            map[i.to].push(e);
        }
    }
    map.iter().flat_map(|c| c.last()).cloned().collect()
}

pub fn part2(input: String) -> String {
    let (mut map, instrs) = parse(&input);
    // println!("{:?}", map);
    for i in instrs {
		let mut tmp = vec![];
        for _c in 0..i.how_many {
            let e = map[i.from].pop().unwrap();
			tmp.insert(0, e);
        }
		map[i.to].extend(tmp);
    }
    map.iter().flat_map(|c| c.last()).cloned().collect()
}
