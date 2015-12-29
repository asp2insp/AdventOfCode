use chomp::*;
use chomp::ascii::{skip_whitespace};

fn light(i: Input<u8>) -> U8Result<bool> {
	or(i, parser!{
			token(b'#');
			ret true
		},
		parser!{
			token(b'.');
			ret false
		},
	)
}

fn line_of_lights(i: Input<u8>) -> U8Result<Vec<bool>> {
	parse!{i;
		let v = many1(light);
				skip_whitespace();
		ret v
	}
}

fn all_lines(i: Input<u8>) -> U8Result<Vec<Vec<bool>>> {
	parse!{i;
		let v = many1(line_of_lights);
		ret v
	}
}

fn tn(b: bool) -> usize {
	if b { 1 } else { 0 }
}

fn count_neighbors(r: usize, c: usize, grid: &Vec<Vec<bool>>) -> usize {
	let mut count = 0usize;
	let max_inner = grid.len()-1;
	// Row above
	if r > 0 {
		if c > 0 {
			count += tn(grid[r-1][c-1]);
		}
		count += tn(grid[r-1][c]);
		if c < max_inner {
			count += tn(grid[r-1][c+1]);
		}
	}
	// Sides
	if c > 0 {
		count += tn(grid[r][c-1]);
	}
	if c < max_inner {
		count += tn(grid[r][c+1]);
	}

	// Row below
	if r < max_inner {
		if c < max_inner {
			count += tn(grid[r+1][c+1]);
		}
		count += tn(grid[r+1][c]);
		if c > 0 {
			count += tn(grid[r+1][c-1]);
		}
	}
	count
}

fn play(grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
	let mut new_grid = vec![
		vec![false; 100]; 100
	];
	for r in 0..grid.len() {
		for c in 0..grid[r].len() {
			new_grid[r][c] = match grid[r][c] {
				true => {match count_neighbors(r, c, grid) {
						2 | 3 => true,
						_ => false,
					}},
				false => {match count_neighbors(r, c, grid) {
					3 => true,
					_ => false,
				}}
			}
		}
	}
	new_grid
}

fn is_corner(r: usize, c: usize, grid: &Vec<Vec<bool>>) -> bool {
	if r == 0 {
		c == 0 || c == grid.len()-1
	} else if c == 0 {
		r == 0 || r == grid.len()-1
	} else {
		false
	}
}

fn play_stuck_corners(grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
	let mut new_grid = vec![
		vec![false; 100]; 100
	];
	for r in 0..grid.len() {
		for c in 0..grid[r].len() {
			new_grid[r][c] = if is_corner(r, c, grid) {
				true
			} else {
				match grid[r][c] {
					true => {match count_neighbors(r, c, grid) {
							2 | 3 => true,
							_ => false,
						}},
					false => {match count_neighbors(r, c, grid) {
							3 => true,
							_ => false,
						}}
				}
			}
		}
	}
	new_grid
}

pub fn part1(input: String) -> String {
	let mut grid = parse_only(all_lines, input.as_bytes()).unwrap();
	for _ in 0..100 {
		grid = play(&grid);
	}
	let count = grid.iter()
		.flat_map(|v| v.iter())
		.fold(0, |sum, l| sum + tn(*l));
	format!("{}", count)
}


pub fn part2(input: String) -> String {
	let mut grid = parse_only(all_lines, input.as_bytes()).unwrap();
	let max_inner = grid.len()-1;
	grid[0][0] = true;
	grid[0][max_inner] = true;
	grid[max_inner][0] = true;
	grid[max_inner][max_inner] = true;

	for _ in 0..100 {
		grid = play_stuck_corners(&grid);
	}

	let count = grid.iter()
		.flat_map(|v| v.iter())
		.fold(0, |sum, l| sum + tn(*l));
	format!("{}", count)
}
