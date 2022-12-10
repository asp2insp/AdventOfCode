use aoc::parse;
use itertools::Itertools;

enum Instr {
    Addx(isize),
    Noop,
}

impl Instr {
    fn cycles(&self) -> isize {
        match self {
            Instr::Addx(_) => 2,
            Noop => 1,
        }
    }
}

fn parse(s: &str) -> Vec<Instr> {
    s.lines()
        .map(|l| {
            let parts = l.split_ascii_whitespace().collect_vec();
            match parts[0] {
                "noop" => Instr::Noop,
                "addx" => Instr::Addx(parse!(parts[1])),
                _ => unimplemented!(),
            }
        })
        .collect_vec()
}

const SAMPLES: &[usize] = &[20, 60, 100, 140, 180, 220];

pub fn part1(input: String) -> String {
    let mut x = 1;
    let mut cycles = 0;
    let mut out = 0;
    for i in parse(&input) {
        for _ in 1..=i.cycles() {
            cycles += 1;
            if SAMPLES.contains(&cycles) {
                // println!("{} * {} = {}", cycles, x, cycles as isize * x);
                out += x * cycles as isize;
            }
        }
        match i {
            Instr::Addx(n) => {
                // println!("+ {}", n);
                x += n;
            }
            Instr::Noop => {}
        }
    }
    out.to_string()
}

pub fn part2(input: String) -> String {
    let width = 40;
    let height = 6;
    let mut out = vec![vec![' '; width]; height];
    let mut x: isize = 1;
    let mut cycles: usize = 0;
    for i in parse(&input) {
        for _ in 0..i.cycles() {
            let cx = cycles % width;
            let cy = cycles / width;
            if x.abs_diff(cx as isize) <= 1 {
				// println!("# at {} {}", cy, cx);
                out[cy][cx] = '#'
            }
			cycles += 1;
        }
        match i {
            Instr::Addx(n) => {
                x += n;
            }
            Instr::Noop => {}
        }
    }
    println!(
        "{}",
        out.into_iter()
            .map(|r| r.into_iter().collect::<String>())
            .join("\n")
    );
    "".to_owned()
}
