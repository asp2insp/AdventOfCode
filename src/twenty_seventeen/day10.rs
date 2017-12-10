#![allow(unused)]
extern crate itertools;
use itertools::*;

const L: usize = 256;

fn part_one() {
    let mut v = vec![0; L];
    for i in 0..L {
        v[i] = i;
    }
    let lengths: Vec<_> = INPUT.split_terminator(',').flat_map(|s| s.parse::<usize>()).collect();
    let mut skip_size = 0;
    let mut offset = 0;
    for length in lengths {
        if offset + length < L {
            v[offset..offset+length].reverse();
        } else {
            let leftover = (offset + length) - L;
            let mut subset = vec![0; length];
            subset[0..L-offset].copy_from_slice(&v[offset..L]);
            subset[L-offset..length].copy_from_slice(&v[0..leftover]);
            subset.reverse();
            v[offset..L].copy_from_slice(&subset[0..L-offset]);
            v[0..leftover].copy_from_slice(&subset[L-offset..length]);
        }
        
        // println!("{:?}", v);
        offset = (offset + length + skip_size) % L;
        skip_size += 1;
    }
    println!("{}", v[0] * v[1]);
}


fn part_two() {
    let mut standard = vec![17, 31, 73, 47, 23];
    let mut v = vec![0; L];
    for i in 0..L {
        v[i] = i;
    }
    let mut lengths: Vec<_> = INPUT.chars().map(|c| (c as u8) as usize).collect();
    lengths.append(&mut standard);
    let mut skip_size = 0;
    let mut offset = 0;
    for _ in 0..64 {
        for length in lengths.clone() {
            if offset + length < L {
                v[offset..offset+length].reverse();
            } else {
                let leftover = (offset + length) - L;
                let mut subset = vec![0; length];
                subset[0..L-offset].copy_from_slice(&v[offset..L]);
                subset[L-offset..length].copy_from_slice(&v[0..leftover]);
                subset.reverse();
                v[offset..L].copy_from_slice(&subset[0..L-offset]);
                v[0..leftover].copy_from_slice(&subset[L-offset..length]);
            }
            
            // println!("{:?}", v);
            offset = (offset + length + skip_size) % L;
            skip_size += 1;
        }
    }
    let hash = v.chunks(16)
        .map(|c| c.iter().fold(0, |a,b| a ^ b))
        .map(|n| format!("{:x}", n))
        .join("");
    println!("{}", hash);
}


fn main() {
    part_one();
    part_two();
}

const INPUT: &str = "88,88,211,106,141,1,78,254,2,111,77,255,90,0,54,205";
// const INPUT: &str = "3,4,1,5";
