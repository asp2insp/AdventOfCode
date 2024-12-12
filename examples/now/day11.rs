use aoc::utils::gimme_usizes_once;
use cached::proc_macro::cached;

fn round(v: Vec<usize>) -> Vec<usize> {
    v.into_iter()
        .flat_map(|i| match i {
            0 => vec![1],
            x if len(x) % 2 == 0 => {
                let s = x.to_string();
                let l = s.len();
                let sc1 = s.chars();
                let sc2 = s.chars();
                let first = sc1
                    .take(l / 2)
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap();
                let second = sc2
                    .skip(l / 2)
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap();
                vec![first, second]
            }
            x => vec![x * 2024],
        })
        .collect()
}

pub fn part1(input: String) -> String {
    let mut nums = gimme_usizes_once(&input);
    for _ in 0..25 {
        nums = round(nums);
    }
    nums.len().to_string()
}

// const BOUNDARY: usize = 4940711462450593;

fn len(mut u: usize) -> usize {
    let mut l = 0;
    while u > 0 {
        u /= 10;
        l += 1;
    }
    l
}

#[cached]
fn count_children(start: usize, gen: usize) -> usize {
    if gen == 0 {
		// print!("{}, ", start);
        return 1;
    } else if start == 0 {
		return count_children(1, gen - 1);
	}
    let l = len(start) as u32;
    if l % 2 == 1 {
        return count_children(start * 2024, gen - 1);
    } else {
        let div = 10u64.pow(l / 2);
		// println!("{} -> {}, {}", start, start / (div as usize), start % (div as usize));
        return count_children(start / (div as usize), gen - 1)
            + count_children(start % (div as usize), gen - 1);
    }
}

fn count_children_for_all(nums: &[usize], gen: usize) -> usize {
	let mut size = 0;
    for &num in nums {
        size += count_children(num, gen)
    }
	// println!("");
	size
}

pub fn part2(input: String) -> String {
    let nums = gimme_usizes_once(&input);
    count_children_for_all(&nums, 75).to_string()
}


#[test]
fn test() {
	assert_eq!(1, count_children(0, 1));
	assert_eq!(1, count_children(1, 1));
	assert_eq!(1, count_children(231, 1));
	assert_eq!(2, count_children(22, 1));
}

#[test]
fn test2() {
	let v = vec![125, 17];
	assert_eq!(3, count_children_for_all(&v, 1));
	assert_eq!(4, count_children_for_all(&v, 2));
	assert_eq!(5, count_children_for_all(&v, 3));
	assert_eq!(9, count_children_for_all(&v, 4));
	assert_eq!(13, count_children_for_all(&v, 5));
	assert_eq!(22, count_children_for_all(&v, 6));
	assert_eq!(55312, count_children_for_all(&v, 25));
}
