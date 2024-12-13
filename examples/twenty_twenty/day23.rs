
fn clockwise_of(v: &Vec<usize>, idx: usize) -> usize {
    if idx + 1 < v.len() {
        idx + 1
    } else {
        0
    }
}

fn do_move(v: &mut Vec<usize>, mut current_idx: usize) -> usize {
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

pub fn part2(_s: String) -> String {
    "2".to_string()
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