use itertools::*;
use lazy_static::lazy_static;
use regex::*;

lazy_static! {
    static ref RE: Regex = Regex::new(r"\s*(\w+) (\d+) (\d+) (\d+)").unwrap();
}

fn parse_instruction(s: &str) -> Instruction {
    let n = RE.captures(s).unwrap();
    Instruction {
        op: n[1].into(),
        a: n[2].parse().unwrap(),
        b: n[3].parse().unwrap(),
        c: n[4].parse().unwrap(),
    }
}

fn parse_program(s: &str) -> (usize, Vec<Instruction>) {
    (
        s.lines()
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .next()
            .unwrap()
            .parse()
            .unwrap(),
        s.lines().skip(1).map(|l| parse_instruction(l)).collect(),
    )
}

#[derive(Copy, Clone, Debug)]
struct Computer {
    reg: [usize; 6],
}

impl Computer {
    fn run(&mut self, i: &Instruction) {
        match i.op {
            Addr => self.reg[i.c] = self.reg[i.a] + self.reg[i.b],
            Addi => self.reg[i.c] = self.reg[i.a] + i.b,

            Mulr => self.reg[i.c] = self.reg[i.a] * self.reg[i.b],
            Muli => self.reg[i.c] = self.reg[i.a] * i.b,

            Banr => self.reg[i.c] = self.reg[i.a] & self.reg[i.b],
            Bani => self.reg[i.c] = self.reg[i.a] & i.b,

            Borr => self.reg[i.c] = self.reg[i.a] | self.reg[i.b],
            Bori => self.reg[i.c] = self.reg[i.a] | i.b,

            Setr => self.reg[i.c] = self.reg[i.a],
            Seti => self.reg[i.c] = i.a,

            Gtri => self.reg[i.c] = if self.reg[i.a] > i.b { 1 } else { 0 },
            Gtir => self.reg[i.c] = if i.a > self.reg[i.b] { 1 } else { 0 },
            Gtrr => self.reg[i.c] = if self.reg[i.a] > self.reg[i.b] { 1 } else { 0 },

            Eqri => self.reg[i.c] = if self.reg[i.a] == i.b { 1 } else { 0 },
            Eqir => self.reg[i.c] = if i.a == self.reg[i.b] { 1 } else { 0 },
            Eqrr => self.reg[i.c] = if self.reg[i.a] == self.reg[i.b] { 1 } else { 0 },
        };
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    op: Opcode,
    a: usize,
    b: usize,
    c: usize,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtri,
    Gtir,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}
use self::Opcode::*;

impl<'a> From<&'a str> for Opcode {
    fn from(s: &'a str) -> Opcode {
        match s {
            "addr" => Addr,
            "addi" => Addi,
            "mulr" => Mulr,
            "muli" => Muli,
            "banr" => Banr,
            "bani" => Bani,
            "borr" => Borr,
            "bori" => Bori,
            "setr" => Setr,
            "seti" => Seti,
            "gtri" => Gtri,
            "gtir" => Gtir,
            "gtrr" => Gtrr,
            "eqir" => Eqir,
            "eqri" => Eqri,
            "eqrr" => Eqrr,
            other => panic!("Unknown opcode {}", other),
        }
    }
}

pub fn part1(input: String) -> String {
    let (iptr_i, prog) = parse_program(&input);
    let mut computer = Computer { reg: [0; 6] };
    let mut iptr = 0;
    while iptr < prog.len() {
        computer.reg[iptr_i] = iptr;
        let instr = &prog[iptr];
        computer.run(instr);
        iptr = computer.reg[iptr_i] + 1;
    }
    computer.reg[0].to_string()
}

fn sum_of_divisors(n: usize) -> usize {
    (1..=n).filter(|i| n % i == 0).sum()
}

pub fn part2(input: String) -> String {
    let (iptr_i, prog) = parse_program(&input);
    let mut computer = Computer { reg: [0; 6] };
    computer.reg[0] = 1;
    let mut iptr = 0;
    while iptr < prog.len() {
        if iptr == 1 {
            return sum_of_divisors(computer.reg[3]).to_string();
        }
        computer.reg[iptr_i] = iptr;
        let instr = &prog[iptr];
        computer.run(instr);
        iptr = computer.reg[iptr_i] + 1;
    }
    computer.reg[0].to_string()
}
