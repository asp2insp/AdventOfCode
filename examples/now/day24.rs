use aoc::{
    dict, makeset, parse,
    utils::{gimme_usizes_once, IterUtils, ToDebugString},
};
use chrono::format;
use fnv::{FnvHashMap, FnvHashSet};
use itertools::Itertools;
use regex::Regex;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Op {
    And,
    Or,
    Xor,
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::And => write!(f, "AND"),
            Op::Or => write!(f, "OR"),
            Op::Xor => write!(f, "XOR"),
        }
    }
}

#[derive(Clone)]
struct Gate {
    op: Op,
    input1: String,
    input2: String,
    output: String,
}

impl Gate {
    fn has_input(&self, input: &str) -> bool {
        self.input1 == input || self.input2 == input
    }

    fn other_input(&self, input: &str) -> &str {
        if self.input1 == input {
            &self.input2
        } else {
            &self.input1
        }
    }

    fn has_inputs(&self, i1: &str, i2: &str) -> bool {
        self.has_input(i1) && self.has_input(i2)
    }
}

fn parse_gates(input: &str) -> (Vec<Gate>, FnvHashMap<String, Vec<usize>>) {
    let mut gates = Vec::new();
    let mut affected = dict!();
    let re = Regex::new(r"(\w+) (AND|OR|XOR) (\w+) -> (\w+)").unwrap();
    input
        .lines()
        .skip_while(|l| !l.is_empty())
        .skip(1)
        .for_each(|l| {
            let caps = re.captures(l).unwrap();
            let op = match &caps[2] {
                "AND" => Op::And,
                "OR" => Op::Or,
                "XOR" => Op::Xor,
                _ => panic!(),
            };
            gates.push(Gate {
                op,
                input1: caps[1].to_string(),
                input2: caps[3].to_string(),
                output: caps[4].to_string(),
            });
            affected
                .entry(caps[1].to_string())
                .or_insert(Vec::new())
                .push(gates.len() - 1);
            affected
                .entry(caps[3].to_string())
                .or_insert(Vec::new())
                .push(gates.len() - 1);
        });
    (gates, affected)
}

fn parse_initial(input: &str) -> FnvHashMap<String, usize> {
    let mut wires = dict!();
    input.lines().take_while(|l| l.contains(':')).for_each(|l| {
        let (name, val) = l.split(": ").collect_tuple().expect(l);
        wires.insert(name.to_string(), parse!(val, usize));
    });
    wires
}

fn run_until_values(
    wires: &mut FnvHashMap<String, usize>,
    gates: &[Gate],
    affected: &FnvHashMap<String, Vec<usize>>,
    target_prefix: char,
) {
    let mut aff = FnvHashSet::default();
    aff.extend(0..gates.len());
    while !gates
        .iter()
        .map(|g| &g.output)
        .filter(|o| o.starts_with(target_prefix))
        .all(|t| wires.contains_key(t))
    {
        let mut aff2 = FnvHashSet::default();
        for gate_idx in aff {
            let gate = &gates[gate_idx];
            if wires.contains_key(&gate.output) {
                continue;
            }
            let input1 = match wires.get(&gate.input1) {
                Some(v) => *v,
                None => continue,
            };
            let input2 = match wires.get(&gate.input2) {
                Some(v) => *v,
                None => continue,
            };
            let output = match gate.op {
                Op::And => input1 & input2,
                Op::Or => input1 | input2,
                Op::Xor => input1 ^ input2,
            };
            wires.insert(gate.output.clone(), output);
            aff2.extend(affected.get(&gate.output).unwrap_or(&Vec::new()));
        }
        aff = aff2;
        if aff.is_empty() {
            break;
        }
    }
}

pub fn part1(input: String) -> String {
    let mut wires = parse_initial(&input);
    let (gates, affected) = parse_gates(&input);
    run_until_values(&mut wires, &gates, &affected, 'z');
    wires_to_num(&wires, 'z').to_string()
}

fn wires_to_num(wires: &FnvHashMap<String, usize>, prefix: char) -> usize {
    usize::from_str_radix(
        &wires
            .iter()
            .filter(|(k, _)| k.starts_with(prefix))
            .sorted_by(|a, b| gimme_usizes_once(b.0)[0].cmp(&gimme_usizes_once(a.0)[0]))
            .map(|(_, v)| v.to_string())
            .join(""),
        2,
    )
    .unwrap()
}

pub fn part2(input: String) -> String {
    let (mut gates, affected) = parse_gates(&input);
    let mut carry_outs = dict!();

    for i in 1..45 {
        let xin = format!("x{:#02}", i);
        let yin = format!("y{:#02}", i);
        let zout = format!("z{:#02}", i);

        // Find xin ^ yin -> half_sum_wire
        let sum_xor_1 = gates
            .iter()
            .find(|g| g.has_inputs(&xin, &yin) && g.op == Op::Xor);
        // Find half_sum_wire ^ carry_in -> zout
        let sum_xor_2 = gates.iter().find(|g| g.output == zout && g.op == Op::Xor);
        // Figure out the name of the half_sum_wire
        let half_sum_wire = sum_xor_1.map(|g| g.output.clone());
        // Figure out the name of the carry_in
        let carry_in = sum_xor_2.and_then(|g| half_sum_wire.as_ref().map(|h| g.other_input(&h)));

        // Find xin & yin -> half_carry_wire
        let carry_and_1 = gates
            .iter()
            .find(|g| g.has_inputs(&xin, &yin) && g.op == Op::And);
        // Find cin & half_sum_wire -> other_half_carry_wire
        let carry_and_2 = half_sum_wire.as_ref()
            .and_then(|hsw| gates.iter().find(|g| g.has_input(&hsw) && g.op == Op::And));
        // Find half_carry_wire | other_half_carry_wire -> carry_out
        let carry_or_out = carry_and_1
            .and_then(|c1| carry_and_2.map(|c2| (c1, c2)))
            .and_then(|(c1, c2)| {
                gates
                    .iter()
                    .find(|g| g.has_inputs(&c1.output, &c2.output) && g.op == Op::Or)
            });
        // Figure out the name of the carry_out
        let carry_out = carry_or_out.map(|g| g.output.clone());
        if let Some(ref cout) = carry_out {
            carry_outs.insert(i, cout.clone());
        }

        if sum_xor_1.is_none()
            || sum_xor_2.is_none()
            || carry_and_1.is_none()
            || carry_and_2.is_none()
            || carry_or_out.is_none()
            || half_sum_wire.is_none()
            || carry_in.is_none()
            || carry_out.is_none()
        {
            println!("Failed to find gates for {}", i);

            continue;
        }
    }

    // Graphviz
    // for (i, g) in gates.iter().enumerate() {
    //     println!("{}[label={}]", i, g.op);
    //     println!("{} -> {}", g.input1, i);
    //     println!("{} -> {}", g.input2, i);
    //     println!("{} -> {}", i, g.output);
    // }
    // Found by inspecting the graphviz from the above failed to find gates
    let swaps: [(i32, &str, &str); 4] = [
        (8, "z08", "mvb"),
        (14, "jss", "rds"),
        (18, "wss", "z18"),
        (23, "z23", "bmn"),
    ];

    for (_, a, b) in &swaps {
        let new_gates = swap_gates(&gates, a, b);
        gates = new_gates;
    }
    let x = 0b0101110101010101110101010001101010;
    let y = 0b0110101011101110110011110101101011;
    let answer = x + y;
    let wires = run_with_nums(x, y, &gates, &affected);
    let current = wires_to_num(&wires, 'z');
    if current != answer {
        println!("Failed to get correct answer: {:#b} != {:#b}", current, answer);
    }
    swaps.iter().flat_map(|(_, a, b)| {
        vec![a, b]
    })
    .sorted()
    .join(",")
}

// for xi in 0..48 {
//     for yi in 0..48 {
//         let x = 1 << xi;
//         let y = 1 << yi;
//         let answer = x + y;
//         let wires = run_with_nums(x, y, &gates, &affected);
//         let current = wires_to_num(&wires, 'z');
//         if current != answer {
//             let possibles = find_possible_mismatches(answer, current, &gates).into_iter().collect_vec();
//             for p in 0..possibles.len() {
//                 for q in p + 1..possibles.len() {
//                     let a = &possibles[p];
//                     let b = &possibles[q];
//                     // println!("Checking {} and {} for {}", a, b, answer);
//                     let new_gates = swap_gates(&gates, a, b);
//                     let new_wires = run_with_nums(x, y, &new_gates, &affected);
//                     let new_current = wires_to_num(&new_wires, 'z');
//                     if new_current == answer {
//                         swaps.push((a.clone(), b.clone()));
//                         println!("Swapping {} and {} to get {} -> {}", a, b, answer, new_current);
//                     }
//                 }
//             }
//         }
//     }
// }

fn swap_gates(gates: &[Gate], a: &str, b: &str) -> Vec<Gate> {
    gates
        .iter()
        .map(|g| {
            let mut g2 = g.clone();
            if g2.output == a {
                g2.output = b.to_string();
            } else if g.output == b {
                g2.output = a.to_string();
            }
            g2
        })
        .collect_vec()
}

fn find_possible_mismatches(answer: usize, current: usize, gates: &[Gate]) -> FnvHashSet<String> {
    let current = format!("{:#b}", current).chars().rev().collect_vec();
    let mut mismatches = format!("{:#b}", answer)
        .chars()
        .rev()
        .enumerate()
        .zip(&current)
        .filter(|((_i, a), c)| a != *c)
        .map(|((i, _), _)| format!("z{:#02}", i))
        .collect_vec();
    let mut possibles = makeset!();
    while let Some(n) = mismatches.pop() {
        if n.starts_with('x') || n.starts_with('y') {
            continue;
        }
        for g in gates {
            if g.output == n {
                mismatches.push(g.input1.clone());
                mismatches.push(g.input2.clone());
            }
        }
        possibles.insert(n);
    }
    possibles
}

fn run_with_nums(
    x: usize,
    y: usize,
    gates: &[Gate],
    affected: &FnvHashMap<String, Vec<usize>>,
) -> FnvHashMap<String, usize> {
    let mut wires = prep_wires(x, y);
    run_until_values(&mut wires, gates, affected, 'z');
    wires
}

fn prep_wires(x: usize, y: usize) -> FnvHashMap<String, usize> {
    let mut wires = dict!();
    for i in 0..64 {
        let val = (x >> i) & 1;
        wires.insert(format!("x{:#02}", i), val);
        let val = (y >> i) & 1;
        wires.insert(format!("y{:#02}", i), val);
    }
    wires
}
