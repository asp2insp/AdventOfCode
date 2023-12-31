use aoc::utils::{gimme_nums, P3};
use num::{
    rational::{BigRational, Ratio},
    FromPrimitive,
};

const BOT: isize = 200000000000000;
const TOP: isize = 400000000000000;

fn parse(s: &str) -> Vec<V3> {
    gimme_nums(s)
        .into_iter()
        .map(|v| V3::new(P3::new(v[0], v[1], v[2]), P3::new(v[3], v[4], v[5])))
        .collect()
}

struct V3 {
    pos: P3,
    vel: P3,
}

fn det2(a: [[BigRational; 2]; 2]) -> BigRational {
    a[0][0].clone() * a[1][1].clone() - a[0][1].clone() * a[1][0].clone()
}

fn det3(a: [[BigRational; 3]; 3]) -> BigRational {
    a[0][0].clone() * det2([[a[1][1].clone(), a[1][2].clone()], [a[2][1].clone(), a[2][2].clone()]])
        - a[0][1].clone() * det2([[a[1][0].clone(), a[1][2].clone()], [a[2][0].clone(), a[2][2].clone()]])
        + a[0][2].clone() * det2([[a[1][0].clone(), a[1][1].clone()], [a[2][0].clone(), a[2][1].clone()]])
}

impl V3 {
    fn new(pos: P3, vel: P3) -> Self {
        Self { pos, vel }
    }

    fn intersection_xy(&self, other: &V3) -> Option<(isize, isize)> {
        let a = self.pos.x;
        let da = self.vel.x;

		let b = self.pos.y;
        let db = self.vel.y;

		let c = other.pos.x;
		let dc = other.vel.x;

		let d = other.pos.y;
		let dd = other.vel.y;

        // a + da * t = c + dc * u => a - c = dc * u - da * t
		// b + db * t = d + dd * u => b - d = dd * u - db * t
		let coeffs = [
			[dc, -da],
			[dd, -db],
		];
		let constants = [a - c, b - d];
		if let Some((t, _u)) = cramer_2simple(coeffs, constants) {
			Some((a + da * t, b * db * t))
		} else {
			return None;
		}
    }
}

pub fn part1(input: String) -> String {
    let vectors = parse(&input);
    let mut count = 0;
    for i in 0..vectors.len() {
        for j in i + 1..vectors.len() {
            if let Some((x, y)) = vectors[i].intersection_xy(&vectors[j]) {
                if x >= BOT && x <= TOP && y >= BOT && y <= TOP {
                    count += 1;
                }
            }
        }
    }
    count.to_string()
}

fn print_coeffs(
    first: &V3,
    second: &V3,
) -> (
    BigRational,
    BigRational,
    BigRational,
    BigRational,
    BigRational,
) {
    // (db - de) * x + ( e - b ) * dx + (dd - da) * y + (a - d) * dy = db * a - da * b + dd * e - de * d
    let a = first.pos.x;
    let b = first.pos.y;
    let da = first.vel.x;
    let db = first.vel.y;
    let d = second.pos.x;
    let e = second.pos.y;
    let dd = second.vel.x;
    let de = second.vel.y;

    let xcoeff = db - de;
    let dx_coeff = e - b;
    let ycoeff = dd - da;
    let dy_coeff = a - d;
    let constant = db * a - da * b + dd * e - de * d;

    // println!(
    //     "{} + {} + {} + {} = {}",
    //     xcoeff, dx_coeff, ycoeff, dy_coeff, constant
    // );
    (
        Ratio::from_isize(xcoeff).unwrap(),
        Ratio::from_isize(dx_coeff).unwrap(),
        Ratio::from_isize(ycoeff).unwrap(),
        Ratio::from_isize(dy_coeff).unwrap(),
        Ratio::from_isize(constant).unwrap(),
    )
}

fn replace_col<const N: usize>(coeffs: [[BigRational; N]; N], constants: [BigRational; N], col: usize) -> [[BigRational; N]; N] {
    let mut new_coeffs = coeffs;
    for i in 0..N {
        new_coeffs[i][col] = constants[i].clone();
    }
    new_coeffs
}

fn det4(a: [[BigRational; 4]; 4]) -> BigRational {
    a[0][0].clone()
        * det3([
            [a[1][1].clone(), a[1][2].clone(), a[1][3].clone()],
            [a[2][1].clone(), a[2][2].clone(), a[2][3].clone()],
            [a[3][1].clone(), a[3][2].clone(), a[3][3].clone()],
        ])
        - a[0][1].clone()
            * det3([
                [a[1][0].clone(), a[1][2].clone(), a[1][3].clone()],
                [a[2][0].clone(), a[2][2].clone(), a[2][3].clone()],
                [a[3][0].clone(), a[3][2].clone(), a[3][3].clone()],
            ])
        + a[0][2].clone()
            * det3([
                [a[1][0].clone(), a[1][1].clone(), a[1][3].clone()],
                [a[2][0].clone(), a[2][1].clone(), a[2][3].clone()],
                [a[3][0].clone(), a[3][1].clone(), a[3][3].clone()],
            ])
        - a[0][3].clone()
            * det3([
                [a[1][0].clone(), a[1][1].clone(), a[1][2].clone()],
                [a[2][0].clone(), a[2][1].clone(), a[2][2].clone()],
                [a[3][0].clone(), a[3][1].clone(), a[3][2].clone()],
            ])
}

fn cramer_4(coeffs: [[BigRational; 4]; 4], constants: [BigRational; 4]) -> (BigRational, BigRational, BigRational, BigRational) {
    let d = det4(coeffs.clone());
    let x1 = det4(replace_col(coeffs.clone(), constants.clone(), 0)) / d.clone();
    let x2 = det4(replace_col(coeffs.clone(), constants.clone(), 1)) / d.clone();
    let x3 = det4(replace_col(coeffs.clone(), constants.clone(), 2)) / d.clone();
    let x4 = det4(replace_col(coeffs.clone(), constants.clone(), 3)) / d.clone();
    (x1, x2, x3, x4)
}

fn cramer_2simple(coeffs: [[isize; 2]; 2], constants: [isize; 2]) -> Option<(isize, isize)> {
	let d = coeffs[0][0] * coeffs[1][1] - coeffs[0][1] * coeffs[1][0];
	if d == 0 {
		return None;
	}
	let x1 = constants[0] * coeffs[1][1] - constants[1] * coeffs[1][0] / d.clone();
	let x2 = coeffs[0][0] * constants[1] - coeffs[0][1] * constants[0] / d.clone();
	Some((x1, x2))
}

fn cramer_2(coeffs: [[BigRational; 2]; 2], constants: [BigRational; 2]) -> Option<(BigRational, BigRational)> {
	let d = det2(coeffs.clone());
	if d == Ratio::from_isize(0).unwrap() {
		return None;
	}
	let x1 = det2(replace_col(coeffs.clone(), constants.clone(), 0)) / d.clone();
	let x2 = det2(replace_col(coeffs.clone(), constants.clone(), 1)) / d.clone();
	Some((x1, x2))
}

pub fn part2(input: String) -> String {
    let vectors = parse(&input);
    let (x1, dx1, y1, dy1, c1) = print_coeffs(&vectors[0], &vectors[2]);
    let (x2, dx2, y2, dy2, c2) = print_coeffs(&vectors[1], &vectors[3]);
    let (x3, dx3, y3, dy3, c3) = print_coeffs(&vectors[4], &vectors[5]);
    let (x4, dx4, y4, dy4, c4) = print_coeffs(&vectors[6], &vectors[7]);
    let coeffs = [
        [x1, dx1, y1, dy1],
        [x2, dx2, y2, dy2],
        [x3, dx3, y3, dy3],
        [x4, dx4, y4, dy4],
    ];
    let constants = [c1, c2, c3, c4];
    let (x, dx, y, dy) = cramer_4(coeffs, constants);
    // println!("x = {}, dx = {}, y = {}, dy = {}", x, dx, y, dy);
	let a0 = Ratio::from_isize(vectors[0].pos.x).unwrap();
	let da0 = Ratio::from_isize(vectors[0].vel.x).unwrap();
    let t0 = (a0 - x.clone()) / (dx - da0);
	let b0 = Ratio::from_isize(vectors[2].pos.y).unwrap();
	let db0 = Ratio::from_isize(vectors[2].vel.y).unwrap();
    let t1 = (b0 - y.clone()) / (dy - db0);
    let c0 = Ratio::from_isize(vectors[0].pos.z).unwrap();
    let dc0 = Ratio::from_isize(vectors[0].vel.z).unwrap();
    let c1 = Ratio::from_isize(vectors[2].pos.z).unwrap();
    let dc1 = Ratio::from_isize(vectors[2].vel.z).unwrap();
    // This gives us the equations:
    // 1z + t0 * dz = c0 + t0 * dc0
    // 1z + t1 * dz = c1 + t1 * dc1
	let coeffs = [
		[Ratio::from_isize(1).unwrap(), t0.clone()],
		[Ratio::from_isize(1).unwrap(), t1.clone()],
	];
	let constants = [c0 + t0 * dc0, c1 + t1 * dc1];
	let (z, _dz) = cramer_2(coeffs, constants).unwrap();
    (x + y + z).to_string()
}
