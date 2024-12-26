enum Schematic {
    Lock([usize; 5]),
    Key([usize; 5]),
}

fn parse(s: &str) -> Schematic {
    let mut nums = [0; 5];
    for l in s.lines().skip(1).take(5) {
        for (i, c) in l.chars().enumerate() {
            if c == '#' {
                nums[i] += 1;
            }
        }
    }
    if s.lines().next().unwrap() == "#####" {
        Schematic::Lock(nums)
    } else {
        Schematic::Key(nums)
    }
}

pub fn part1(input: String) -> String {
    let schemes = input.split("\n\n").map(parse).collect::<Vec<_>>();
    // println!("{}, {} keys, {} locks", schemes.len(), schemes.iter().filter(|s| if let Schematic::Key(_) = s { true } else { false }).count(), schemes.iter().filter(|s| if let Schematic::Lock(_) = s { true } else { false }).count());
    let mut count = 0;
    for s in &schemes {
        if let Schematic::Key(k) = s {
            for l in &schemes {
                if let Schematic::Lock(l) = l {
                    if k.iter().zip(l).all(|(a, b)| a + b <= 5) {
                        count += 1;
                    }
                }
            }
        }
    }
    count.to_string()
}

pub fn part2(_input: String) -> String {
    "part2".to_string()
}
