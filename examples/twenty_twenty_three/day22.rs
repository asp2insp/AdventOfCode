use std::collections::{HashMap, VecDeque};

use aoc::{
    makeset,
    utils::{drop_by_idx, BetterRange},
};

const FLOOR: usize = 999999;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Block {
    id: usize,
    xrange: BetterRange,
    yrange: BetterRange,
    zrange: BetterRange,
    supported_by: Vec<usize>,
    supports: Vec<usize>,
}

impl Block {
    fn in_same_column(&self, other: &Block) -> bool {
        self.xrange.overlaps(&other.xrange) && self.yrange.overlaps(&other.yrange)
    }

    fn shift_z(&mut self, offset: isize) {
        self.zrange.shift_mut(offset);
    }

    fn is_below(&self, other: &Block) -> bool {
        self.in_same_column(other) && self.zrange.top <= other.zrange.bottom
    }

    fn is_right_below(&self, other: &Block) -> bool {
        self.in_same_column(other) && self.zrange.top == other.zrange.bottom
    }
}

fn parse(s: &str) -> Vec<Block> {
    s.lines()
        .map(|l| {
            l.split('~')
                .map(|b| {
                    b.split(',')
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .enumerate()
        .map(|(i, v)| Block {
            id: i,
            xrange: BetterRange::new_unordered_inclusive(v[0][0], v[1][0]),
            yrange: BetterRange::new_unordered_inclusive(v[0][1], v[1][1]),
            zrange: BetterRange::new_unordered_inclusive(v[0][2], v[1][2]),
            supported_by: vec![],
            supports: vec![],
        })
        .collect()
}

fn settle(mut blocks: Vec<Block>) -> Vec<Block> {
    blocks.sort_by_key(|b| b.zrange.bottom);
    let mut output = vec![];
    // Find the ones that can fall all the way to the ground
    let mut to_remove = vec![];
    for (i, b) in blocks.iter().enumerate() {
        if b.zrange.bottom == 0 {
            let mut b2 = b.clone();
            b2.supported_by.push(FLOOR);
            output.push(b2);
            to_remove.push(i);
        } else if !blocks.iter().any(|o| o != b && o.is_below(&b)) {
            let offset = b.zrange.bottom as isize * -1;
            let mut b2 = b.clone();
            b2.shift_z(offset);
            b2.supported_by.push(FLOOR);
            output.push(b2);
            to_remove.push(i);
        }
    }
    blocks = drop_by_idx(blocks, to_remove.as_slice());

    // Now we have a list of blocks that are resting on the ground or on other blocks
    // Iterate through, find blocks that are not at rest yet, and move them to be at rest
    while !to_remove.is_empty() {
        to_remove.clear();
        for (i, b) in blocks.iter().enumerate() {
            if !(0..blocks.len()).any(|j| j != i && blocks[j].is_below(b)) {
                // This block is on the bottom of the blocks that can still fall, find
                // the stable blocks in output that are in the same column and move this
                // block to rest on top of them
                let resting_place = output
                    .iter()
                    .filter(|o| o.is_below(b))
                    .map(|o| o.zrange.top as isize)
                    .max()
                    .unwrap();
                let offset = resting_place - b.zrange.bottom as isize;
                let mut b2 = b.clone();
                b2.shift_z(offset);
                for o in &mut output {
                    if o.is_right_below(&b2) {
                        b2.supported_by.push(o.id);
                        o.supports.push(b2.id);
                    }
                }
                output.push(b2);
                to_remove.push(i);
            }
        }
        blocks = drop_by_idx(blocks, to_remove.as_slice());
    }
    output
}

pub fn part1(input: String) -> String {
    let blocks = parse(&input);
    let settled = settle(blocks);
    settled
        .iter()
        .filter(|b| {
            // find if any blocks are only supported by this block
            settled
                .iter()
                .filter(|o| o.supported_by.len() == 1 && o.supported_by[0] == b.id)
                .count()
                == 0
        })
        // .inspect(|b| println!("{:?} is safe", b))
        .count()
        .to_string()
}

pub fn part2(input: String) -> String {
    let blocks = parse(&input);
    let settled = settle(blocks);
    let by_id = settled.iter().map(|b| (b.id, b)).collect::<HashMap<_, _>>();
    settled
        .iter()
        .map(|b| {
            let mut dis = makeset!(b.id);
            let mut queue = VecDeque::new();
            b.supports.iter().for_each(|s| queue.push_back(&by_id[s]));
            while !queue.is_empty() {
                let nb = queue.pop_front().unwrap();
                if nb.supported_by.iter().all(|s| dis.contains(s)) {
                    dis.insert(nb.id);
                    for s in &nb.supports {
                        if !dis.contains(s) {
                            queue.push_back(&by_id[s]);
                        }
                    }
                }
            }
            dis.len() - 1
        })
        .sum::<usize>()
        .to_string()
}

#[test]
fn test() {
    let i = r"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
    assert_eq!(part1(i.to_string()), "5");
    assert_eq!(part2(i.to_string()), "7");
}
