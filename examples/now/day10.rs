use std::{collections::VecDeque, usize};

use itertools::Itertools;
use regex::Regex;
use nalgebra::{DMatrix, DVector};
use z3::{Optimize, ast::Int};

struct Machine {
    pattern: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<f64>,
}

fn push_buttons(v: &Vec<bool>, bs: &[usize]) -> Vec<bool> {
    let mut res = v.clone();
    for &b in bs {
        res[b] = !res[b];
    }
    res
}

fn push_joltage(v: &Vec<usize>, bs: &[usize]) -> Vec<usize> {
    let mut res = v.clone();
    for &b in bs {
        res[b] += 1;
    }
    res
}

impl Machine {
    fn num_lights(&self) -> usize {
        self.pattern.len()
    }

    fn num_jolts(&self) -> usize {
        self.joltage.len()
    }

    fn count_button_presses(&self) -> usize {
        // BFS
        let mut q = VecDeque::new();
        q.push_back((vec![false; self.num_lights()], 0));
        while let Some((state, count)) = q.pop_front() {
            if state == self.pattern {
                return count;
            }
            for b in &self.buttons {
                q.push_back((push_buttons(&state, &b[..]), count + 1));
            }
        }
        usize::MAX
    }

    fn count_joltage_presses(&self) -> usize {
        let b = DVector::from_vec(self.joltage.clone());
		let mut a = DMatrix::from_element(self.num_jolts(), self.buttons.len(), 0.0f64);
		for (c, b) in self.buttons.iter().enumerate() {
			for &i in b {
				// println!("{},{} = 1 in should be {}x{} but is actually {}x{}", i, c, self.num_jolts(), self.buttons.len(), a.nrows(), a.ncols());
				a[(i,c)] = 1f64;
			}
		}
		let x = a.svd(true, true).solve(&b, 0.001).unwrap();
		x.iter().sum::<f64>() as usize
    }

    fn z3_joltage_presses(&self) -> usize {
        let opt = Optimize::new();

        // Create integer variables for each button (number of times pressed)
        let button_vars: Vec<Int> = (0..self.buttons.len())
            .map(|i| Int::new_const(format!("button_{}", i)))
            .collect();

        // Each button must be pressed a non-negative number of times
        for var in &button_vars {
            opt.assert(&var.ge(&Int::from_i64(0)));
        }

        // For each joltage position, the sum of button presses must equal the target
        for (jolt_idx, &target_joltage) in self.joltage.iter().enumerate() {
            let mut sum = Int::from_i64(0);
            for (button_idx, button_positions) in self.buttons.iter().enumerate() {
                if button_positions.contains(&jolt_idx) {
                    sum = sum + &button_vars[button_idx];
                }
            }
            opt.assert(&sum.eq(&Int::from_i64(target_joltage as i64)));
        }

        // Minimize the total number of button presses
        let total_presses: Int = button_vars.iter().fold(Int::from_i64(0), |acc, var| acc + var);
        opt.minimize(&total_presses);

        // Solve and extract result
        if opt.check(&[]) == z3::SatResult::Sat {
            let model = opt.get_model().unwrap();
            let result = model.eval(&total_presses, true).unwrap().as_i64().unwrap();
            result as usize
        } else {
            panic!("No solution found!");
        }
    }
}

fn parse_machine(s: &str) -> Machine {
    //[.###] (0,2,3) (1,2) (0) (0,2) (1,3) {32,23,39,9}
    let re = Regex::new(r"\[([.#]+)\] ((\([\d,]+\) )+)\{([\d,]+)\}").unwrap();
    let cap = re.captures(s).unwrap();
    Machine {
        pattern: cap[1].chars().map(|c| c == '#').collect_vec(),
        buttons: cap[2]
            .trim()
            .split_ascii_whitespace()
            .map(|bs| {
                bs.trim_start_matches('(')
                    .trim_end_matches(')')
                    .split(',')
                    .map(|d| d.parse::<usize>().unwrap())
                    .collect_vec()
            })
            .collect_vec(),
        joltage: cap[4]
            .split(',')
            .map(|d| d.parse::<f64>().unwrap())
            .collect_vec(),
    }
}

pub fn part1(input: String) -> String {
    input
        .lines()
        .map(parse_machine)
        .map(|m| m.count_button_presses())
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: String) -> String {
    input
        .lines()
        .map(parse_machine)
        // .collect_vec()
        // .into_par_iter()
        .map(|m| {
            let c = m.z3_joltage_presses();
            // let c = m.count_joltage_presses();
            // println!("Found another in {c}");
            c
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;
        let count = input
            .lines()
            .map(parse_machine)
            .map(|m| m.count_button_presses())
            .sum::<usize>();
        assert_eq!(7, count);
    }

    #[test]
    fn test_part_2() {
        let input = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;
        let count = input
            .lines()
            .map(parse_machine)
            .map(|m| m.count_joltage_presses())
            .sum::<usize>();
        assert_eq!(33, count);
    }
}
