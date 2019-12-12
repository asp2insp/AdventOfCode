use itertools::*;
use regex::*;
use num::integer;

#[derive(Debug, Clone, PartialEq)]
struct Planet {
	pos: [isize; 3],
	vel: [isize; 3],
}

fn parse_planets(input: &str) -> Vec<Planet> {
	let re = Regex::new(r#"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>"#).unwrap();
	input
		.lines()
		.map(|l| {
			let caps = re.captures(l.trim()).unwrap();
			Planet {
				pos: [
					caps[1].parse().unwrap(),
					caps[2].parse().unwrap(),
					caps[3].parse().unwrap(),
				],
				vel: [0, 0, 0],
			}
		})
		.collect()
}

fn simulate_step(planets: &mut [Planet]) {
	// Apply gravity to update velocities
	for c in (0..planets.len()).combinations(2) {
		let (i, j) = (c[0], c[1]);
		for axis in 0..3 {
			let (app_i, app_j) = match (planets[i].pos[axis], planets[j].pos[axis]) {
				(si, sj) if si == sj => (0, 0),
				(si, sj) if si > sj => (-1, 1),
				_ => (1, -1),
			};
			planets[i].vel[axis] += app_i;
			planets[j].vel[axis] += app_j;
		}
	}
	// Apply velocities to update pos
	for p in planets.iter_mut() {
		p.pos[0] += p.vel[0];
		p.pos[1] += p.vel[1];
		p.pos[2] += p.vel[2];
	}
}

fn energy(p: &Planet) -> isize {
	p.pos.iter().cloned().map(isize::abs).sum::<isize>()
		* p.vel.iter().cloned().map(isize::abs).sum::<isize>()
}

pub fn part1(input: String) -> String {
	let mut planets = parse_planets(&input);
	for _ in 0..1000 {
		simulate_step(&mut planets);
	}
	planets.iter().map(energy).sum::<isize>().to_string()
}

pub fn part2(input: String) -> String {
	let mut planets = parse_planets(&input);
	let target = planets.clone();
	let mut count = 0;
	let mut axes = vec![usize::max_value(); 3];
	loop {
		count += 1;
		simulate_step(&mut planets);
		for axis in 0..3 {
			if planets.iter().zip(target.iter()).all(|(p, t)| p.pos[axis] == t.pos[axis] && p.vel[axis] == t.vel[axis]) {
				axes[axis] = axes[axis].min(count);
			}
		}
		if axes.iter().all(|c| c < &usize::max_value()) {
			let prod: usize = axes.iter().product();
			let gcd: usize = integer::gcd(axes[0], integer::gcd(axes[1], axes[2]));
			let ans = prod / gcd;
			return format!("{:?}: not actually?? {}", axes, ans)
		}
	}
}
