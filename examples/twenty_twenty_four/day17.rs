use aoc::utils::gimme_usize_nums;
use itertools::Itertools;

enum Yield {
    Done,
    Continue,
    Out(usize),
}

impl Yield {
    fn to_string(&self) -> String {
        match self {
            Yield::Done => "".to_string(),
            Yield::Continue => "".to_string(),
            Yield::Out(x) => x.to_string(),
        }
    }
}

#[derive(Clone)]
struct Interp {
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
    ip: usize,
    program: Vec<usize>,
}

impl Interp {
    fn step(&mut self) -> Yield {
        if self.ip >= self.program.len() {
            return Yield::Done;
        }

        let instr = self.program[self.ip];
        let operand = self.value_of(self.program[self.ip + 1]);
        let literal = self.program[self.ip + 1];
        self.ip += 2;
        let mut out = Yield::Continue;
        match instr {
            0 => {
                // A_DV
                self.reg_a = self.reg_a / 2usize.pow(operand as u32);
            }
            1 => {
                // BXL
                self.reg_b = self.reg_b ^ literal;
            }
            2 => {
                // BST
                self.reg_b = operand % 8;
            }
            3 => {
                // JNZ
                if self.reg_a != 0 {
                    self.ip = literal;
                }
            }
            4 => {
                // BXC
                self.reg_b = self.reg_b ^ self.reg_c;
            }
            5 => {
                // OUT
                out = Yield::Out(operand % 8);
            }
            6 => {
                // B_DV
                self.reg_b = self.reg_a / 2usize.pow(operand as u32);
            }
            7 => {
                // C_DV
                self.reg_c = self.reg_a / 2usize.pow(operand as u32);
            }
            _ => panic!("Invalid instruction {}", instr),
        }
        out
    }

    fn value_of(&self, combo_operand: usize) -> usize {
        match combo_operand {
            0..=3 => combo_operand,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            7 => 7, // reserved for future use
            _ => panic!("Invalid operand {}", combo_operand),
        }
    }
}

pub fn part1(input: String) -> String {
    let nums = gimme_usize_nums(&input);
    let mut interp = Interp {
        reg_a: nums[0][0],
        reg_b: nums[1][0],
        reg_c: nums[2][0],
        ip: 0,
        program: nums[4].to_vec(),
    };
    let mut result = vec![];
    loop {
        match interp.step() {
            Yield::Done => break,
            Yield::Continue => continue,
            Yield::Out(x) => result.push(x),
        }
    }
    result.into_iter().join(",")
}

pub fn part2(input: String) -> String {
    let nums = gimme_usize_nums(&input);
    let interp = Interp {
        reg_a: nums[0][0],
        reg_b: nums[1][0],
        reg_c: nums[2][0],
        ip: 0,
        program: nums[4].to_vec(),
    };
    let mut a = find_parts_rec(&interp, interp.program.len() - 1, 0).unwrap();
    a <<= 3;
    // println!("About to find lower digits {}", a);
    // Fill in the lowest digits
    while run_comp(&interp, a)[..] != interp.program[..] {
        a += 1;
    }
    assert_eq!(&interp.program, &run_comp(&interp, a)[..]);
    a.to_string()
}

fn find_parts_rec(interp: &Interp, digit_idx: usize, a: usize) -> Option<usize> {
    if digit_idx == 0 {
        return Some(a);
    }
    (0..8usize)
        .map(|j| (a << 3) | j)
        .filter(|&next_a| {
            let out = run_comp(&interp, next_a);
            if out[0..] == interp.program[digit_idx..] {
                // println!("{:?}", out);
                true
            } else {
                false
            }
        })
        .filter_map(|next_a| find_parts_rec(interp, digit_idx - 1, next_a))
        .next()
}

fn run_comp(comp: &Interp, a: usize) -> Vec<usize> {
    let mut i = comp.clone();
    i.reg_a = a;
    let mut out = vec![];
    loop {
        match i.step() {
            Yield::Done => break,
            Yield::Continue => continue,
            Yield::Out(x) => out.push(x),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r"Register A: 117440
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
        assert_eq!(part1(input.to_string()), "0,3,5,4,3,0");
    }

    #[test]
    fn test_part2() {
        let input = r"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
        let nums = gimme_usize_nums(&input);
        let interp = Interp {
            reg_a: nums[0][0],
            reg_b: nums[1][0],
            reg_c: nums[2][0],
            ip: 0,
            program: nums[4].to_vec(),
        };
        assert_eq!(vec![0, 3, 5, 4, 3, 0], run_comp(&interp, 117440));
        assert_eq!(part2(input.to_string()), "117440");
    }
}
