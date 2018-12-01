use chomp::prelude::{U8Input, SimpleResult, parse_only, token, many1, skip_many};
use chomp::ascii::{decimal};
use std::collections::HashSet;

fn sep<I: U8Input>(i: I) -> SimpleResult<I, u8> {
    parse!{i;
        token(b',') <|> token(b' ')
    }
}

fn instruction<I: U8Input>(i: I) -> SimpleResult<I, (u8, isize)> {
    parse!{i;
        let turn: u8 = token(b'R') <|> token(b'L');
		let steps = decimal();
                   skip_many(sep);
		ret (turn, steps)
	}
}

fn all_instructions<I: U8Input>(i: I) -> SimpleResult<I, Vec<(u8, isize)>> {
    parse!{i;
		let v = many1(instruction);
		ret v
	}
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn follow_direction(x: isize, y: isize, d: self::Direction, n: isize)
    -> (isize, isize, self::Direction) {
    use self::Direction::*;
    match d {
        North => (x, y - n, d),
        South => (x, y + n, d),
        East => (x + n, y, d),
        West => (x - n, y, d),
    }
}

fn follow_direction_all_blocks(x: isize, y: isize, d: self::Direction, n: isize)
    -> Vec<(isize, isize)> {
    let mut ret = vec![];
    let mut x = x;
    let mut y = y;
    for _ in 0..n {
        let (x_new, y_new, _) = follow_direction(x, y, d, 1);
        ret.push((x_new, y_new));
        x = x_new;
        y = y_new;
    }
    ret
}

fn get_direction(d: Direction, turn: u8) -> self::Direction {
    use self::Direction::*;
    match (d, turn) {
        (North, b'R') => East,
        (North, b'L') => West,
        (South, b'R') => West,
        (South, b'L') => East,
        (East, b'R') => South,
        (East, b'L') => North,
        (West, b'R') => North,
        (West, b'L') => South,
        _ => unimplemented!(),
    }
}

fn follow_directions(instrs: &Vec<(u8, isize)>) -> (isize, isize) {
    use self::Direction::*;
    let endpoint = instrs.iter()
        .fold((0, 0, North), |(x, y, d), ref i| {
            //println!("{}, {}, {:?}, {}, {}", x, y, d, i.0, i.1);
            follow_direction(x, y, get_direction(d, i.0), i.1)
        });
    (endpoint.0, endpoint.1)
}

pub fn part1(input: String) -> String {
    let instrs = parse_only(all_instructions, input.as_bytes()).unwrap();
    let (x, y) = follow_directions(&instrs);
    let distance = x.abs() + y.abs();
    format!("Endpoint: ({},{}). {} blocks away", x, y, distance)
}

pub fn part2(input: String) -> String {
    let instrs = parse_only(all_instructions, input.as_bytes()).unwrap();

    let mut x = 0;
    let mut y = 0;
    let mut d = self::Direction::North;
    let mut visited = HashSet::new();
    visited.insert((0, 0));
    for i in &instrs {
        d = get_direction(d, i.0);
        let blocks = follow_direction_all_blocks(x, y, d, i.1);
        for (x_new, y_new) in blocks {
            if visited.contains(&(x_new, y_new)) {
                let distance = x_new.abs() + y_new.abs();
                return format!("({}, {}). {} blocks away", x_new, y_new, distance)
            } else {
                visited.insert((x_new, y_new));
                x = x_new;
                y = y_new;
            }
        }
    }
    format!("No repeats")
}
