use aoc::utils::*;
use crate::intcode::*;
use itertools::Itertools;
use std::collections::VecDeque;

pub fn part1(s: String) -> String {
    let prog = gimme_nums(&s).into_iter().flatten().collect_vec();
    let mut comps = (0..50).map(|addr| {
        let mut c = Computer::new(prog.clone());
        c.run_and_return_output();
        c.input(addr);
        c
    })
    .collect_vec();
    let mut mailboxes = vec![VecDeque::new(); 50];
    loop {
        for (i, c) in comps.iter_mut().enumerate() {
            match c.run_and_return_output() {
                ProgYield::Input => if mailboxes[i].is_empty() {
                    c.input(-1);
                } else {
                    c.input(mailboxes[i].pop_front().unwrap());
                    c.input(mailboxes[i].pop_front().unwrap());
                },
                ProgYield::Output(addr) => {
                    let x = c.run_and_return_output().unwrap();
                    let y = c.run_and_return_output().unwrap();
                    if addr == 255 {
                        return y.to_string()
                    }
                    mailboxes[addr as usize].push_back(x);
                    mailboxes[addr as usize].push_back(y);
                },
                ProgYield::Halt => {},
            };
        }
    }
}


pub fn part2(s: String) -> String {
    let prog = gimme_nums(&s).into_iter().flatten().collect_vec();
    let mut comps = (0..50).map(|addr| {
        let mut c = Computer::new(prog.clone());
        c.run_and_return_output();
        c.input(addr);
        c
    })
    .collect_vec();
    let mut mailboxes = vec![VecDeque::new(); 50];
    let mut last = (0, 0);
    let mut ret = 0;
    loop {
        let mut idle_count = 0;
        for (i, c) in comps.iter_mut().enumerate() {
            match c.run_and_return_output() {
                ProgYield::Input => if mailboxes[i].is_empty() {
                    c.input(-1);
                    idle_count += 1;
                } else {
                    c.input(mailboxes[i].pop_front().unwrap());
                    c.input(mailboxes[i].pop_front().unwrap());
                },
                ProgYield::Output(addr) => {
                    let x = c.run_and_return_output().unwrap();
                    let y = c.run_and_return_output().unwrap();
                    if addr == 255 {
                        last = (x, y);
                        continue;
                    }
                    mailboxes[addr as usize].push_back(x);
                    mailboxes[addr as usize].push_back(y);
                },
                ProgYield::Halt => {},
            };
        }
        if idle_count == comps.len() {
            mailboxes[0].push_back(last.0);
            mailboxes[0].push_back(last.1);
            if ret != 0 && ret == last.1 {
                return ret.to_string();
            }
            ret = last.1;
        }
    }
}
