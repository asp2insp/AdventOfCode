use crate::utils::*;
use std::collections::HashSet;

#[derive(Debug)]
enum Seg {
    Plain(Direction),
    Branch(Vec<Vec<Seg>>),
}

// Take input starting from 1 past group start, return 1 past group end offset
fn find_group_end(s: &[char]) -> usize {
    let mut count = 1;
    let mut i = 0;
    while count > 0 {
        match s[i] {
            '(' => count += 1,
            ')' => count -= 1,
            _ => {}
        };
        i += 1;
    }
    i
}

fn parse(s: &[char]) -> Seg {
    let mut segs = vec![vec![]];
    let mut i = 0;
    while i < s.len() {
        match s[i] {
            '^' | '$' => i += 1,
            'N' | 'S' | 'E' | 'W' => {
                ppush!(&mut segs, Seg::Plain(Direction::from_char(s[i]).unwrap()));
                i += 1;
            }
            '|' => {
                segs.push(vec![]);
                i += 1;
            }
            '(' => {
                let next_i = find_group_end(&s[i + 1..]);
                ppush!(&mut segs, parse(&s[i + 1..i + next_i]));
                i += 1 + next_i;
            }
            ')' => unreachable!("Unexpected end paren"),
            a => unreachable!("WTF is {}", a),
        };
    }
    Seg::Branch(segs)
}

fn fill_out_map(
    map: &mut Grid<()>,
    points: HashSet<Point>,
    re: Seg,
) -> HashSet<Point> {
    match re {
        Seg::Plain(d) => {
            let mut next = HashSet::new();
            for p in points {
                let door = map.drive(p, d).unwrap();
                let room = map.drive(door, d).unwrap();
                map.set(door, '+', ());
                map.set(room, '.', ());
                next.insert(room);
            }
            next
        }
        Seg::Branch(arms) => {
            let mut all = HashSet::new();
            for arm in arms {
                let mut next = points.clone();
                for seg in arm {
                    next = fill_out_map(map, next, seg);
                }
                all.extend(next.into_iter());
            }
            all
        }
    }
}

pub fn part1(input: String) -> String {
    let re = parse(&input.chars().collect::<Vec<_>>());
    let mut map: Grid<()> = Grid::default();
    map.wall_char = '#';
    let origin = Point{x: 0, y: 0};
	map.set(origin, '.', ());
    fill_out_map(&mut map, makeset! {origin}, re);
    map.dfs_path_bulk(
        origin,
        map.iter_range(None, None)
            .filter(|(_, c, _)| *c == '.')
			.map(|(p, _, _)| p)
            .collect(),
        Some(|p| {
            map.get(p)
                .map(|(c, _)| if *c == '+' { 1 } else { 0 })
                .unwrap_or(isize::MAX)
        }),
    )
    .values()
    .map(|(c, _)| c)
    .max()
    .unwrap()
    .to_string()
}

pub fn part2(input: String) -> String {
    let re = parse(&input.chars().collect::<Vec<_>>());
    let mut map: Grid<()> = Grid::default();
    map.wall_char = '#';
    let origin = Point{x: 0, y: 0};
    map.set(origin, '.', ());
    fill_out_map(&mut map, makeset! {origin}, re);
    map.dfs_path_bulk(
        origin,
        map.iter_range(None, None)
            .filter(|(_, c, _)| *c == '.')
			.map(|(p, _, _)| p)
            .collect(),
        Some(|p| {
            map.get(p)
                .map(|(c, _)| if *c == '+' { 1 } else { 0 })
                .unwrap_or(isize::MAX)
        }),
    )
    .values()
    .map(|(c, _)| *c)
    .filter(|c| *c >= 1000)
    .count()
    .to_string()
}

#[test]
fn test_parse() {
    let testcase = "^ENWWW(NEEE|SSE(EE|N))$";
    let expected = "Branch([
		[Plain(E), Plain(N), Plain(W), Plain(W), Plain(W), Branch([
			[Plain(N), Plain(E), Plain(E), Plain(E)], 
			[Plain(S), Plain(S), Plain(E), Branch([
				[Plain(E), Plain(E)], 
				[Plain(N)]
			])
		]])
	]])"
    .replace("\n", "")
    .replace("\t", "");
    let actual = parse(&testcase.chars().collect::<Vec<_>>());
    assert_eq!(expected, format!("{:?}", actual));
}
