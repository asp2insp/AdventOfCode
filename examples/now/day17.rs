use aoc::{dict, makeset, utils::Point};
use fnv::FnvHashSet;
use itertools::Itertools;

fn make_shapes() -> Vec<Vec<Point>> {
    vec![
        // horizontal 4
        vec![
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(2, 0),
            Point::new(3, 0),
        ],
        // cross
        vec![
            Point::new(0, 1),
            Point::new(1, 0),
            Point::new(1, 1),
            Point::new(1, 2),
            Point::new(2, 1),
        ],
        // L
        vec![
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(2, 0),
            Point::new(2, 1),
            Point::new(2, 2),
        ],
        // vertical 4
        vec![
            Point::new(0, 0),
            Point::new(0, 1),
            Point::new(0, 2),
            Point::new(0, 3),
        ],
        // block
        vec![
            Point::new(0, 0),
            Point::new(0, 1),
            Point::new(1, 1),
            Point::new(1, 0),
        ],
    ]
}

fn run_n(s: &str, n: usize) -> (FnvHashSet<Point>, Vec<isize>) {
    let dirs = s.trim().chars().collect_vec();
    let shapes = make_shapes();
    let mut i = 0;
    let mut top = 0;
    let mut board = makeset!();
	let mut r = Vec::new();
    for si in 0..n {
        let mut s = shapes[si % shapes.len()]
            .iter()
            .map(|p| Point::new(2 + p.x, top + 3 + p.y))
            .collect_vec();
        // if si <= 10 {
        // 	let mut b = board.clone();
        // 	b.extend(s.iter().cloned());
        // 	print_board(&b);
        // }
        loop {
            // println!("top: {} s[0]: {:?} d: {}", top, s[0], dirs[i%dirs.len()]);
            // Move over
            match dirs[i % dirs.len()] {
                '>' => {
                    let is_blocked = s
                        .iter()
                        .any(|p| p.x == 6 || board.contains(&p.offset((1, 0))));
                    if !is_blocked {
                        s.iter_mut().for_each(|p| p.x += 1);
                    }
                }
                '<' => {
                    let is_blocked = s
                        .iter()
                        .any(|p| p.x == 0 || board.contains(&p.offset((-1, 0))));
                    if !is_blocked {
                        s.iter_mut().for_each(|p| p.x -= 1);
                    }
                }
                _ => unreachable!(),
            }
            // Increment direction
            i = (i + 1) % dirs.len();
            // Move down. If can't move down, add to board and move to next shape
            let is_blocked = s
                .iter()
                .any(|p| p.y == 0 || board.contains(&p.offset((0, -1))));
            if is_blocked {
				let ntop = top.max(s.iter().map(|p| p.y).max().unwrap() + 1);
				r.push(ntop-top);
                top = ntop;
                board.extend(s);
                // println!("Rest. Next. New Top {}", top);
                break;
            } else {
                s.iter_mut().for_each(|p| p.y -= 1);
            }
        }
    }
    (board ,r)
}

pub fn part1(input: String) -> String {
    (run_n(&input, 2022).0.iter().map(|p| p.y).max().unwrap() + 1).to_string()
}

fn print_board(board: &FnvHashSet<Point>) {
    let mut y = board.iter().map(|p| p.y).max().unwrap();
    while y >= 0 {
        let mut row = vec!['.'; 7];
        board
            .iter()
            .filter(|p| p.y == y)
            .map(|p| p.x)
            .for_each(|x| row[x as usize] = '#');
        println!("{}|{}|", y % 10, row.into_iter().join(""));
        y -= 1;
    }
    println!(" +-------+");
}

pub fn part2(input: String) -> String {
    let mut target: u64 = 1000000000000;
    let (_, increments) = run_n(&input, 6000);
    let consideration = increments.split_at(1000).1;
	let mut max = 0;
	let mut size = 0;
	let mut seen = dict!();
	let mut win = vec![];
	for (i, w) in consideration.windows(101).enumerate() {
		if let Some(idx) = seen.get(&w) {
			size = consideration[*idx..i].iter().sum::<isize>();
			println!("Found repeat of size {} with height {}", i - idx, size);
			max = i - idx;
			win = consideration[*idx..i].to_owned();
			break;
		} else {
			seen.insert(w, i);
		}
	}
	let mut count = 0;
	let mut i = 0;
	// Count until the first repeat
	while increments[i..i+101] != win[..101] {
		i += 1;
		count += increments[i];
	}
	// Middle cycle
	target -= i as u64;
	count += (target / max as u64) as isize * size;
	// End cycle
	target %= max as u64;
	for j in 0..target as usize {
		count += increments[i + j]
	}
	count.to_string()
}

#[test]
fn test1() {
    assert_eq!(
        "3068",
        part1(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>".to_owned())
    );
}
