use aoc::{dict, parse, utils::gimme_usizes_once};
use fnv::{FnvHashMap, FnvHashSet};
use itertools::Itertools;
use regex::Regex;
use take_until::TakeUntilExt;

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
    _affected: &FnvHashMap<String, Vec<usize>>,
    targets: &FnvHashSet<String>,
) {
    while !targets.iter().all(|t| wires.contains_key(t)) {
        for gate in gates {
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
        }
    }
}

pub fn part1(input: String) -> String {
    let mut wires = parse_initial(&input);
    let (gates, affected) = parse_gates(&input);
    let targets = gates
        .iter()
        .map(|g| &g.output)
        .filter(|o| o.starts_with('z'))
        .cloned()
        .collect::<FnvHashSet<_>>();
    run_until_values(&mut wires, &gates, &affected, &targets);
    let binary = targets
        .into_iter()
        .sorted_by(|a, b| gimme_usizes_once(b)[0].cmp(&gimme_usizes_once(a)[0]))
        .map(|t| wires[&t].to_string())
        .join("");
	format!("{}", usize::from_str_radix(&binary, 2).unwrap())
}

pub fn part2(input: String) -> String {
    "part2".to_string()
}
