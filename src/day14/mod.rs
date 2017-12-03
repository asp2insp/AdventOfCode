//! This is a regular crate doc comment, but it also contains a partial
//! Cargo manifest.  Note the use of a *fenced* code block, and the
//! `cargo` "language".
//!
//! ```cargo
//! [dependencies]
//! rust-crypto = "0.2"
//! itertools = "0.7"
//! rayon = "0.9"
//! ```

extern crate crypto;
extern crate rayon;

use crypto::md5::*;
use crypto::digest::*;
use rayon::prelude::*;

pub fn part1(input: String) -> String {
    let mut n = 64;
    let hashes = (0..50_000)
        .map(|i| {
            let mut sh = Md5::new();
            sh.input(format!("{}{}", input, i).as_bytes());
            sh.result_str()
        })
        .collect::<Vec<_>>();
    for i in 0..hashes.len() {
        let t = match hashes[i].as_bytes()
            .windows(3)
            .find(|b| b[0] == b[1] && b[1] == b[2]) {
                None => continue,
                Some(b) => b[0],
        };
        for j in i+1..i+1000 {
            if hashes[j].as_bytes()
                .windows(5)
                .any(|s| s.iter().all(|c| *c == t)) {
                    n -= 1;
                    if n == 0 {
                        return format!("{}", i)
                    }
                break
            }
        }
    }
    panic!("Not found!")
}


pub fn part2(input: String) -> String {
    let mut n = 64;
    let nums = (0..50_000).collect::<Vec<_>>();
    let hashes = nums.par_iter()
        .map(|i| {
            let mut sh = Md5::new();
            sh.input_str(&format!("{}{}", input, i));
            let mut key = sh.result_str();
            for _ in 0..2016 {
                sh.reset();
                sh.input_str(&key);
                key = sh.result_str();
            }
            key
        })
        .collect::<Vec<_>>();
    for i in 0..hashes.len() {
        let t = match hashes[i].as_bytes()
            .windows(3)
            .find(|b| b[0] == b[1] && b[1] == b[2]) {
                None => continue,
                Some(b) => b[0],
        };
        for j in i+1..i+1000 {
            if hashes[j].as_bytes()
                .windows(5)
                .any(|s| s.iter().all(|c| *c == t)) {
                    n -= 1;
                    if n == 0 {
                        return format!("{} ({})", i, j)
                    }
                break
            }
        }
    }
    panic!("Not found!")
}

fn main() {
    println!("{}", part1("cuanljph".to_owned()));
    println!("{}", part2("cuanljph".to_owned()));
}
