

fn clockwise_of(v: &Vec<usize>, idx: usize) -> usize {
    if idx + 1 < v.len() {
        idx + 1
    } else {
        0
    }
}

fn do_move(v: &mut Vec<usize>, current_idx: usize) -> usize {
    let mut label = v[current_idx] - 1;
    let curr_item = v[current_idx];
    let a = v.remove(clockwise_of(v, current_idx));
    let b = v.remove(clockwise_of(v, current_idx));
    let c = v.remove(clockwise_of(v, current_idx));
    let min = *v.iter().min().unwrap();
    let max = *v.iter().max().unwrap();
    // println!("{} in {:?}", label, v);
    while !v.contains(&label) {
        if label <= min {
            label = max;
        } else {
            label -= 1;
        }
    }
    let loc = clockwise_of(&v, v.iter().position(|&x| x == label).unwrap());
    v.insert(loc, c);
    v.insert(loc, b);
    v.insert(loc, a);
    clockwise_of(v, v.iter().position(|&x| x == curr_item).unwrap())
}

pub fn part1(_s: String) -> String {
    let mut ring = vec![5,8,3,9,7,6,2,4,1];
    let mut cidx = 0;
    for _ in 0..100 {
        cidx = do_move(&mut ring, cidx);
    }
    let mut s = String::new();
    let mut i = ring.iter().position(|&x| x == 1).unwrap();
    for _ in 0..ring.len()-1 {
        i = clockwise_of(&ring, i);
        s.push_str(&ring[i].to_string());
    }
    s
}

fn do_move_ll(ring: &mut Vec<usize>, cidx: usize, minmax: (usize, usize)) -> usize {
    assert!(cidx != 0);
    // Ring represents a large circular linked list. Each slot
    // A contains a label B which is also the item pointed to by A->B
    // Thus creating a long linked list stored in a single vec.
    // Do a move. First we remove the 3 elements clockwise of cidx:
    let mut removed = vec![];
    for _ in 0..3 {
        removed.push(ring[cidx]);
        ring[cidx] = ring[ring[cidx]];
    }
    // Then we find a destination cup
    let mut dest = cidx - 1;
    while removed.contains(&dest) || dest == 0 {
        if dest <= minmax.0 {
            dest = minmax.1;
        } else {
            dest -= 1;
        }
    }
    // Finally, insert removed clockwise of the destination cup
    for _ in 0..3 {
        let n = removed.pop().unwrap();
        ring[n] = ring[dest];
        ring[dest] = n;
    }
    assert!(cidx != 0 && ring[cidx] != 0);
    ring[cidx]
}

pub fn part2(_s: String) -> String {
    let mut v = vec![0; 1_000_001];
    let mut prev = 1;
    for n in [5,8,3,9,7,6,2,4,1].into_iter() {
        v[prev] = n;
        prev = n;
    }
    // println!("{:?}", &v[..15]);
    for i in 10..=1000000 {
        v[prev] = i;
        prev = i;
    }
    let mut cidx = 5; // this was the first item
    v[prev] = cidx; // Close the loop
    let minmax = (1, 1000000);
    for _ in 0..10000000 {
        cidx = do_move_ll(&mut v, cidx, minmax);

    }
    // println!("{}, {}, {}", v[1], v[v[1]], v[v[v[1]]]);
    let a = v[1];
    let b = v[a];
    (a*b).to_string()
}

#[test]
fn test() {
    let mut v = vec![3,8,9,1,2,5,4,6,7];
    let mut cidx = 0;
    cidx = do_move(&mut v, cidx);
    assert_eq!(vec![3,2,8,9,1,5,4,6,7], v);
    assert_eq!(2, v[cidx]);
    cidx = do_move(&mut v, cidx);
    assert_eq!(vec![8, 9, 1, 3, 2, 5, 4, 6, 7], v);
    assert_eq!(5, v[cidx]);
    cidx = do_move(&mut v, cidx);
    assert_eq!(vec![8, 9, 1, 3, 4, 6, 7, 2, 5], v);
    assert_eq!(8, v[cidx]);
    for _ in 3..10 {
        cidx = do_move(&mut v, cidx);
    }
    assert_eq!(vec![6, 5, 8, 3, 7, 4, 1, 9, 2], v);
    assert_eq!(8, v[cidx]);
    for _ in 10..100 {
        cidx =  do_move(&mut v, cidx);
    }
    assert_eq!(vec![1,6,7,3,8,4,5,2,9], v);
}



#[test]
fn test_ll() {
    let mut v: Vec<usize> = vec![0; 10];
    let mut prev = 1;
    for d in "389125467".chars().filter_map(|c| c.to_digit(10)) {
        v[prev] = d as usize;
        prev = d as usize;
    }
    let mut cidx = 3; // this was the start
    v[prev] = cidx;
    let minmax = (1,9);
    assert_eq!(vec![0, 2, 5, 8, 6, 4, 7, 3, 9, 1], v);
    cidx = do_move_ll(&mut v, cidx, minmax);
    assert_eq!(vec![3,2,8,9,1,5,4,6,7], in_order_from(&v, 3));
    cidx = do_move_ll(&mut v, cidx, minmax);
    assert_eq!(vec![3,2,5,4,6,7,8,9,1], in_order_from(&v, 3));
    cidx = do_move_ll(&mut v, cidx, minmax);
    assert_eq!(vec![7,2,5,8,9,1,3,4,6], in_order_from(&v, 7));
    cidx = do_move_ll(&mut v, cidx, minmax);
    assert_eq!(vec![3,2,5,8,4,6,7,9,1], in_order_from(&v, 3));
    cidx = do_move_ll(&mut v, cidx, minmax);
    assert_eq!(vec![9,2,5,8,4,1,3,6,7], in_order_from(&v, 9));
    println!("{:?}", (0..10).collect_vec());
    println!("{:?}, {}", v, cidx);
    cidx = do_move_ll(&mut v, cidx, minmax);
    println!("{:?}, {}", v, cidx);
    assert_eq!(vec![7,2,5,8,4,1,9,3,6], in_order_from(&v, 7));
    for _ in 6..10 {
        cidx = do_move_ll(&mut v, cidx, minmax);
    }
    assert_eq!(vec![1,9,2,6,5,8,3,7,4], in_order_from(&v, 1));
}

#[cfg(test)]
fn in_order_from(v: &Vec<usize>, mut cidx: usize) -> Vec<usize> {
    let mut ret = Vec::new();
    for _ in 0..v.len()-1 {
        ret.push(cidx);
        cidx = v[cidx];
    }
    ret
}