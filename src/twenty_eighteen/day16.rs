use itertools::*;
use chrono::{NaiveDateTime,Timelike};
use std::collections::{HashMap,HashSet};
use time::Duration;
use rayon::prelude::*;
use regex::*;
use std::mem;
use permutohedron::LexicalPermutation;

lazy_static! {
    static ref RE: Regex = Regex::new(r"\w+:\s+\[(\d+), (\d+), (\d+), (\d+)\]").unwrap();
}

#[derive(Debug, Clone)]
struct Example {
    before: [usize; 4],
    i: Instruction,
    after: [usize; 4],
}

impl Example {
    fn with_op(&self, op: usize) -> Example {
        let mut n = self.clone();
        n.i.op = op;
        n
    }
}

fn parse_instruction(s: &str) -> Instruction {
    let n: Vec<usize> = s.split_whitespace()
        .map(|si| si.parse().unwrap())
        .collect();
    Instruction {
        op: n[0],
        a: n[1],
        b: n[2],
        c: n[3],
    }
}

fn parse_arr(s: &str) -> [usize; 4] {
    let cap = RE.captures(s).unwrap();
    [
        cap[1].parse().unwrap(),
        cap[2].parse().unwrap(),
        cap[3].parse().unwrap(),
        cap[4].parse().unwrap(),
    ]
}

fn parse_examples(s: &str) -> Vec<Example> {
    s.lines().take(3120)
        .tuples()
        .map(|(b, o, a, _)| Example {
            before: parse_arr(b),
            i: parse_instruction(o),
            after: parse_arr(a),
        })
        .collect()
}

fn parse_program(s: &str) -> Vec<Instruction> {
    s.lines().skip(3122)
        .map(|l| parse_instruction(l))
        .collect()
}

// Run the example and return whether it worked
fn run_example(e: &Example, table: &[Opcode]) -> bool {
    let Example{before, i, after} = e;
    let mut c = Computer {table: table, reg: *before};
    c.run(i);
    c.reg == *after
}

// Run an example for all opcodes and return the number which worked
fn get_working_ops_for_example(e: &Example) -> Vec<Opcode> {
    let table = vec![Addr, Addi, Mulr, Muli, Banr, Bani, 
        Borr, Bori, Setr, Seti,
        Gtri, Gtir, Gtrr, Eqir, Eqri, Eqrr];
    (0..table.len())
        .map(|op| e.with_op(op))
        .filter(|e2| run_example(e2, &table))
        .map(|e2| table[e2.i.op])
        .collect()
}

pub fn part1(input: String) -> String {
    let eg = parse_examples(&input);
    eg.iter()
        .map(|e| get_working_ops_for_example(e).len())
        .filter(|n| *n >= 3)
        .count()
        .to_string()
}

#[derive(Copy, Clone, Debug)]
struct Computer<'a> {
    table: &'a [Opcode],
    reg: [usize; 4],
}

impl <'a> Computer<'a> {
    fn run(&mut self, i: &Instruction) {
        match self.table[i.op] {
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
            
            Gtri => self.reg[i.c] = if self.reg[i.a] > i.b {1} else {0},
            Gtir => self.reg[i.c] = if i.a > self.reg[i.b] {1} else {0},
            Gtrr => self.reg[i.c] = if self.reg[i.a] > self.reg[i.b] {1} else {0},
            
            Eqri => self.reg[i.c] = if self.reg[i.a] == i.b {1} else {0},
            Eqir => self.reg[i.c] = if i.a == self.reg[i.b] {1} else {0},
            Eqrr => self.reg[i.c] = if self.reg[i.a] == self.reg[i.b] {1} else {0},
        };
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    op: usize,
    a: usize,
    b: usize,
    c: usize,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Opcode {
    Addr, Addi, Mulr, Muli, Banr, Bani, Borr, Bori, Setr, Seti,
    Gtri, Gtir, Gtrr, Eqir, Eqri, Eqrr
}
use self::Opcode::*;

impl <'a> From<&'a str> for Opcode {
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

fn constraint_collapse(mut possibilities: Vec<HashSet<Opcode>>) -> Option<Vec<Opcode>> {
    // println!("Next: {:?}", possibilities.iter().map(HashSet::len).collect::<Vec<_>>());
    let mut done = HashSet::new();
    done.extend(possibilities.iter().filter(|p| p.len()==1).flat_map(|p| p.iter().next()));
    loop {
        if possibilities.iter().all(|p| p.len()==1) {
            break
        }
        let newly_done: Option<(usize, Opcode)> = possibilities.iter()
                .enumerate()
                .find(|(_,p)| p.len()==1 && !done.contains(p.iter().next().unwrap()))
                .map(|(i,p)| (i, *p.iter().next().unwrap()));
        if let Some((i, op)) = newly_done {
            for (_, p) in possibilities.iter_mut().enumerate().filter(|(i2,_)| *i2 != i) {
                p.remove(&op);
                if p.is_empty() {
                    // Contradiction, no options left for this value
                    return None
                }
            }
            done.insert(op);
        } else if let Some(i) = possibilities.iter()
                .enumerate()
                .filter(|(_, p)| p.len() > 1)
                .min_by_key(|(_, p)| p.len())
                .map(|(i,_)| i) {
            for op in possibilities[i].iter().filter(|op| !done.contains(op)) {
                let mut n = possibilities.clone();
                n[i].clear();
                n[i].insert(*op);
                if let Some(answer) = constraint_collapse(n) {
                    return Some(answer)
                }
            }
            return None
        }
    }
    Some(possibilities.into_iter().map(|p| p.into_iter().next().unwrap()).collect())
}

pub fn part2(input: String) -> String {
    let mut table = vec![Addr, Addi, Mulr, Muli, Banr, 
        Bani, Borr, Bori, Setr, Seti,
        Gtri, Gtir, Gtrr, Eqir, Eqri, Eqrr];
    let eg = parse_examples(&input);
    let possibilities = (0..16).map(|i| {
        eg.iter().filter(|e| e.i.op == i).flat_map(|e| get_working_ops_for_example(e)).collect()
    }).collect();
    // println!("Starting possibilities {:?}", possibilities);
    let correct = constraint_collapse(possibilities);
    println!("Final table: {:?}", correct);
    let prog = parse_program(&input);
    let mut c = Computer {table: &correct.unwrap(), reg: [0;4]};
    for i in &prog {
        c.run(i);
    }
    c.reg[0].to_string()
}