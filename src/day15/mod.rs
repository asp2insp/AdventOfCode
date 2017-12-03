// Disc #1 has 13 positions; at time=0, it is at position 11.
// Disc #2 has 5 positions; at time=0, it is at position 0.
// Disc #3 has 17 positions; at time=0, it is at position 11.
// Disc #4 has 3 positions; at time=0, it is at position 0.
// Disc #5 has 7 positions; at time=0, it is at position 2.
// Disc #6 has 19 positions; at time=0, it is at position 17.
// Disc #7 has 11 positions; at time=0, it is at position 0.

fn simulate(discs: &Vec<(usize, usize)>) -> bool {
    let mut discs = discs.clone();
    for i in 0..discs.len() {
        discs = discs.into_iter().map(|(i,d)| ( (i+1)%d, d)).collect();
        if discs[i].0 != 0 {
            return false
        }
    }
    true
}

fn part_one() {
    let mut discs = vec![(11,13), (0,5), (11,17), (0,3), (2,7), (17,19)];
    for n in 0.. {
        if simulate(&discs) {
            println!("{}", n);
            return
        }
        discs = discs.into_iter().map(|(i,d)| ( (i+1)%d, d)).collect();
    }
}

fn part_two() {
    let mut discs = vec![(11,13), (0,5), (11,17), (0,3), (2,7), (17,19), (0,11)];
    for n in 0.. {
        if simulate(&discs) {
            println!("{}", n);
            return
        }
        discs = discs.into_iter().map(|(i,d)| ( (i+1)%d, d)).collect();
    }
}

fn main() {
  part_one();
  part_two();
}
