use aoc::{dict, makeset, parse, utils::gimme_usizes_once};
use fnv::{FnvHashMap, FnvHashSet};
use itertools::Itertools;
use regex::Regex;

enum Op {
    And,
    Or,
    Xor,
}

struct Gate {
    op: Op,
    input1: String,
    input2: String,
    output: String,
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
    }
}

pub fn part1(input: String) -> String {
    let mut wires = parse_initial(&input);
    let (gates, affected) = parse_gates(&input);
    run_until_values(&mut wires, &gates, &affected, 'z');
    wires_to_num(&wires, "z").to_string()
}

fn wires_to_num(wires: &FnvHashMap<String, usize>, prefix: &str) -> usize {
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
    let mut wires = parse_initial(&input);
    let x = wires_to_num(&wires, "x");
    let y = wires_to_num(&wires, "y");
    let answer = format!("{:#b}", x + y).chars().rev().collect_vec();
    let (gates, affected) = parse_gates(&input);
    run_until_values(&mut wires, &gates, &affected, 'z');
    let current = format!("{:#b}", wires_to_num(&wires, "z"))
        .chars()
        .rev()
        .collect_vec();
    let mut mismatches = answer
        .iter()
        .enumerate()
        .zip(&current)
        .filter(|((i, a), c)| a != c)
        .map(|((i, _), _)| format!("z{:#02}", i))
        .collect_vec();
    let mut possibles = makeset!();
    while let Some(n) = mismatches.pop() {
        if n.starts_with('x') || n.starts_with('y') {
            continue;
        }
        for g in &gates {
            if g.output == n {
                mismatches.push(g.input1.clone());
                mismatches.push(g.input2.clone());
            }
        }
        possibles.insert(n);
    }
    println!("{:?}", possibles.len());
    println!("{}\n{}", answer.iter().join(""), current.iter().join(""));
    "part2".to_string()
}
