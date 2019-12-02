use std::collections::HashSet;

pub fn part1(input: String) -> String {
    format!("{}", 
        input.lines()
            .map(|l| l.parse::<i32>().unwrap())
            .fold(0, |s, n| s + n)
    )
}

pub fn part2(input: String) -> String {
    let mut set: HashSet<i32> = HashSet::new();
    let mut s = 0;
    set.insert(0);
    let values: Vec<i32> = input.lines().map(|l| l.parse::<i32>().unwrap()).collect();
    for i in 0.. {
        let n = values[i%values.len()];
        s += n;
        if set.contains(&s) {
            return format!("{}",  s)
        }
        set.insert(s);
    }
    return "No Answer".to_owned()
}