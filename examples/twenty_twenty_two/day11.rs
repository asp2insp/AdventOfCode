use std::mem;

use aoc::utils::{gimme_chunks, gimme_usizes_once};
use itertools::Itertools;

struct Monkey {
    items: Vec<usize>,
    operation: Box<dyn Fn(usize) -> usize>,
    test_divisor: usize,
    true_target: usize,
    false_target: usize,
    item_count: usize,
}

fn make_op(s: &str) -> Box<dyn Fn(usize) -> usize> {
    if s.contains("old * old") {
        return Box::new(|old| old * old);
    }
    let other = gimme_usizes_once(s)[0];
    if s.contains('+') {
        Box::new(move |old| old + other)
    } else if s.contains('*') {
        Box::new(move |old| old * other)
    } else {
        unreachable!("Bad operator")
    }
}

fn parse(s: &str) -> Vec<Monkey> {
    gimme_chunks(s)
        .into_iter()
        // .map(|ss| { println!("{:?}", ss); ss })
        .map(|ss| Monkey {
            items: gimme_usizes_once(ss[1]),
            operation: make_op(ss[2]),
            test_divisor: gimme_usizes_once(ss[3])[0],
            true_target: gimme_usizes_once(ss[4])[0],
            false_target: gimme_usizes_once(ss[5])[0],
            item_count: 0,
        })
        .collect_vec()
}

pub fn part1(input: String) -> String {
    let mut monkeys = parse(&input);
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let mut items = vec![];
            mem::swap(&mut monkeys[i].items, &mut items);
			monkeys[i].item_count += items.len();
			let op = &monkeys[i].operation;
            let mailbox = items
                .into_iter()
                .map(|mut item| {
                    item = op(item);
                    item /= 3;
                    if item % monkeys[i].test_divisor == 0 {
                        (monkeys[i].true_target, item)
                    } else {
                        (monkeys[i].false_target, item)
                    }
                })
                .collect_vec();
            mailbox.into_iter().for_each(|(t, item)| {
                monkeys[t].items.push(item);
            });
        }
    }
    monkeys.sort_by_key(|m| m.item_count);
	monkeys.reverse();
	(monkeys[0].item_count * monkeys[1].item_count).to_string()
}

pub fn part2(input: String) -> String {
    let mut monkeys = parse(&input);
	let mod_by: usize = monkeys.iter().map(|m| m.test_divisor).product();
    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            let mut items = vec![];
            mem::swap(&mut monkeys[i].items, &mut items);
			monkeys[i].item_count += items.len();
			let op = &monkeys[i].operation;
            let mailbox = items
                .into_iter()
                .map(|mut item| {
                    item = op(item);
                    item %= mod_by;
                    if item % monkeys[i].test_divisor == 0 {
                        (monkeys[i].true_target, item)
                    } else {
                        (monkeys[i].false_target, item)
                    }
                })
                .collect_vec();
            mailbox.into_iter().for_each(|(t, item)| {
                monkeys[t].items.push(item);
            });
        }
    }
    monkeys.sort_by_key(|m| m.item_count);
	monkeys.reverse();
	(monkeys[0].item_count * monkeys[1].item_count).to_string()
}
