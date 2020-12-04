use std::collections::HashSet;


pub fn part1(input: String) -> String {
    let nums: HashSet<usize> = input
        .lines()
        .flat_map(str::parse::<usize>)
        .collect();
    for n in &nums {
        if nums.contains(&(2020-n)) {
            return format!("{} x {} = {}", n, 2020-n, n * (2020-n))
        }
    }
    "Not found".to_owned()
}


pub fn part2(input: String) -> String {
    let nums: HashSet<usize> = input
        .lines()
        .flat_map(str::parse::<usize>)
        .collect();
    for n in &nums {
        for m in &nums {
            if n + m < 2020 && nums.contains(& (2020 - n - m)) {
                return format!("{} x {} x {} = {}", n, m, 2020 - n - m, n * m * (2020 - n - m))
            }
        }
    }
    "Not found".to_owned()
}
