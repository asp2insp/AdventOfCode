use aoc::utils::*;
use itertools::Itertools;

use std::fs::File;
use std::io::{Error, Write};

fn log_grid(g: &Grid<()>, loc: Point) -> Result<(), Error> {
    let path = "lines.txt";

    let mut output = File::create(path)?;
    let s = g.to_string_windowed((0, loc.y - 10, 151, loc.y + 30));
    output.write(&s.as_bytes())?;
    let ten_millis = std::time::Duration::from_millis(100);
    std::thread::sleep(ten_millis);
    Ok(())
}

pub fn part1(input: String) -> String {
    let mut parts = input.split("\n\n");
    let g = parts.next().unwrap();
    let instructions = parts.next().unwrap();
    let mut g = Grid::new_preserving_whitespace(g, |_| ());
    let mut loc = Point::new(0, g.top_bound);
    while g.read_pt(&loc) != '.' {
        loc = loc.offset((1, 0));
    }
    let top_bound = g.top_bound;
    let right_bound = g.right_bound;
    g.clear_bounds();
    g.wall_char = '#';
    let mut d = Direction::E;
    for (num, mut i) in &instructions.chars().group_by(|c| c.is_numeric()) {
        let mut s = String::new();
        if num {
            let steps = i.collect::<String>().parse::<usize>().unwrap();
            s.push_str(&format!("Forward {} ", steps));
            for _ in 0..steps {
                g.set(loc, '@', ());
                if let Some(mut lnew) = g.drive(loc, d) {
                    let mut needs_wrap = false;
                    if let Some((' ', _)) = g.get(lnew) {
                        needs_wrap = true;
                    } else if let None = g.get(lnew) {
                        needs_wrap = true;
                    } else if lnew.x < 0 || lnew.y < 0 {
                        needs_wrap = true;
                    } else if lnew.x > right_bound || lnew.y > top_bound {
                        needs_wrap = true;
                    }
                    if needs_wrap {
                        let mut shore = ' ';
                        while let Some((c, _)) = g.get(lnew.offset_dir(d.opposite())) {
                            if *c == ' ' {
                                break;
                            }
                            shore = *c;
                            lnew = lnew.offset_dir(d.opposite());
                        }
                        if shore == '#' {
                            lnew = loc;
                        }
                    }
                    loc = lnew;
                } // else no need to change
            }
        } else {
            let turn = i.next().unwrap();
            d = d.turn(turn);
            s.push_str(&format!("Turn {} to go {:?}", turn, d));
        }
        // println!("{} => {:?}", s, loc);
        // log_grid(&g, loc).unwrap();
    }
    ((1000 * (top_bound - loc.y + 1))
        + (4 * (loc.x + 1))
        + match d {
            Direction::N => 3,
            Direction::S => 1,
            Direction::E => 0,
            Direction::W => 2,
        })
    .to_string()
}

pub fn part2(input: String) -> String {
    let mut parts = input.split("\n\n");
    let g = parts.next().unwrap();
    let instructions = parts.next().unwrap();

    let side_a = string_window(g, 50, 100, 0, 50);
    let g_a = Grid::new(&side_a, ()).with_wall('#');
    let side_b = string_window(g, 100, 150, 0, 50);
    let g_b = Grid::new(&side_b, ()).with_wall('#');
    let side_c = string_window(g, 50, 100, 50, 50);
    let g_c = Grid::new(&side_c, ()).with_wall('#');
    let side_d = string_window(g, 0, 50, 100, 50);
    let g_d = Grid::new(&side_d, ()).with_wall('#');
    let side_e = string_window(g, 50, 100, 100, 50);
    let g_e = Grid::new(&side_e, ()).with_wall('#');
    let side_f = string_window(g, 0, 50, 150, 50);
    let g_f = Grid::new(&side_f, ()).with_wall('#');

	let to_big_grid = |g1: &Grid<()>, p: Point| {
		match g1 {
			g if g == &g_a => Point::new(p.x + 50, 50-p.y),
			g if g == &g_b => Point::new(p.x + 100, 50-p.y),
			g if g == &g_c => Point::new(p.x + 50, 100-p.y),
			g if g == &g_d => Point::new(p.x, 150-p.y),
			g if g == &g_e => Point::new(p.x + 50, 150-p.y),
			g if g == &g_f => Point::new(p.x, 200-p.y),
			_ => unreachable!(),
		}
	};

	let dim = 49;
    let mut loc = (&g_a, Point::new(0, dim), Direction::E);

    for (num, mut i) in &instructions.chars().group_by(|c| c.is_numeric()) {
        if num {
            let steps = i.collect::<String>().parse::<usize>().unwrap();
            for _ in 0..steps {
                if let Some(lnew) = loc.0.drive(loc.1, loc.2) {
                    loc.1 = lnew;
                } else {
                    if !loc.0.in_bounds(loc.1.offset_dir(loc.2)) {
                        // Transition
                        use Direction::*;
                        let lnew = match loc {
                            // A
                            (g, p, N) if g == &g_a => (&g_f, Point::new(0, dim - p.x), E),
                            (g, p, S) if g == &g_a => (&g_c, Point::new(p.x, dim), S),
                            (g, p, E) if g == &g_a => (&g_b, Point::new(0, p.y), E),
                            (g, p, W) if g == &g_a => (&g_d, Point::new(0, dim - p.y), E),

                            // B
                            (g, p, N) if g == &g_b => (&g_f, Point::new(p.x, 0), N),
                            (g, p, S) if g == &g_b => (&g_c, Point::new(dim, dim - p.x), W),
                            (g, p, E) if g == &g_b => (&g_e, Point::new(dim, dim - p.y), W),
                            (g, p, W) if g == &g_b => (&g_a, Point::new(dim, p.y), W),

                            // C
                            (g, p, N) if g == &g_c => (&g_a, Point::new(p.x, 0), N),
                            (g, p, S) if g == &g_c => (&g_e, Point::new(p.x, dim), S),
                            (g, p, E) if g == &g_c => (&g_b, Point::new(dim - p.y, 0), N),
                            (g, p, W) if g == &g_c => (&g_d, Point::new(dim - p.y, dim), S),

                            // D
                            (g, p, N) if g == &g_d => (&g_c, Point::new(0, dim - p.x), E),
                            (g, p, S) if g == &g_d => (&g_f, Point::new(p.x, dim), S),
                            (g, p, E) if g == &g_d => (&g_e, Point::new(0, p.y), E),
                            (g, p, W) if g == &g_d => (&g_a, Point::new(0, dim - p.y), E),

                            // E
                            (g, p, N) if g == &g_e => (&g_c, Point::new(p.x, 0), N),
                            (g, p, S) if g == &g_e => (&g_f, Point::new(dim, dim - p.x), W),
                            (g, p, E) if g == &g_e => (&g_b, Point::new(dim, dim - p.y), W),
                            (g, p, W) if g == &g_e => (&g_d, Point::new(dim, p.y), W),

                            // F
                            (g, p, N) if g == &g_f => (&g_d, Point::new(p.x, 0), N),
                            (g, p, S) if g == &g_f => (&g_b, Point::new(p.x, dim), S),
                            (g, p, E) if g == &g_f => (&g_e, Point::new(dim - p.y, 0), N),
                            (g, p, W) if g == &g_f => (&g_a, Point::new(dim - p.y, dim), S),
							_ => unreachable!(),
                        };
						// println!("{:?} {:?}", loc.1, loc.2);
						assert!(lnew.1.x == 0 || lnew.1.x == dim || lnew.1.y == 0 || lnew.1.y == dim);
                        if lnew.0.read_pt(&lnew.1) != '#' {
                            loc = lnew;
                        }
                    } // Else we hit a wall, no action needed
                }
            }
			// println!("{:?} == {:?}", loc.1, to_big_grid(loc.0, loc.1));
        } else {
            let turn = i.next().unwrap();
            loc.2 = loc.2.turn(turn);
        }
    }
	let fin = to_big_grid(loc.0, loc.1);
	// println!("{:?} {:?}", fin, loc.2);
    (1000 * fin.y
        + (4 * (fin.x + 1))
        + match loc.2 {
            Direction::N => 3,
            Direction::S => 1,
            Direction::E => 0,
            Direction::W => 2,
        })
    .to_string()
}

