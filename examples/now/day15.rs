use aoc::utils::*;
use aoc::*;
use fnv::FnvHashSet;
use itertools::Itertools;

pub fn part1(input: String) -> String {
    let parts = input.split("\n\n").collect_vec();
    let mut g = Grid::new(parts[0], ()).with_wall('#');
    let dirs = parts[1]
        .chars()
        .filter_map(|c| Direction::from_char(c).ok())
        .collect_vec();
    let mut robot = g.find('@').unwrap();
    g.set(robot, '.', ());
    for d in dirs {
        if let Some(target) = g.drive(robot, d) {
            if g.read_pt(&target) == '.' {
                robot = target;
            } else {
                let mut t2 = target;
                while g.read_pt(&t2) == 'O' {
                    if let Some(t3) = g.drive(t2, d) {
                        t2 = t3;
                    } else {
                        break;
                    }
                }
                if g.read_pt(&t2) == '.' {
                    g.set(t2, 'O', ());
                    g.set(target, '.', ());
                    robot = target;
                }
            }
        }
        // g.set(robot, '@', ());
        // println!("{}\n", g);
        // g.set(robot, '.', ());
    }
    g.find_all('O')
        .into_iter()
        .map(|p| 100 * (g.top_bound - p.y) + p.x)
        .sum::<isize>()
        .to_string()
}

pub fn part2(input: String) -> String {
    let parts = input.split("\n\n").collect_vec();
    let doubled = parts[0]
        .chars()
        .flat_map(|c| match c {
            '.' => "..".chars(),
            '#' => "##".chars(),
            '@' => "@.".chars(),
            'O' => "[]".chars(),
            '\n' => "\n".chars(),
            c => panic!("Unknown char {}", c),
        })
        .collect::<String>();
    let mut g = Grid::new(&doubled, ()).with_wall('#');
    let dirs = parts[1]
        .chars()
        .filter_map(|c| Direction::from_char(c).ok())
        .collect_vec();
    let mut robot = g.find('@').unwrap();
    // println!("{}\n", g);
    g.set(robot, '.', ());
    for d in dirs {
        if let Some(target) = g.drive(robot, d) {
            if g.read_pt(&target) == '.' {
                robot = target;
            } else {
                if let Some(pushable) = find_pushable_boxes(&g, robot, d) {
                    for p in pushable.into_iter().sorted_by_key(|p| match d {
                        Direction::N => -p.y,
                        Direction::S => p.y,
                        Direction::E => -p.x,
                        Direction::W => p.x,
                    }) {
                        let boxchar = g.read_pt(&p);
                        g.set(p.offset_dir(d), boxchar, ());
                        g.set(p, '.', ());
                    }
                    robot = target;
                }
            }
        }
        // g.set(robot, '@', ());
        // println!("{}\n", g);
        // g.set(robot, '.', ());
    }
    g.find_all('[')
        .into_iter()
        .map(|p| 100 * (g.top_bound - p.y) + p.x)
        .sum::<isize>()
        .to_string()
}

fn find_pushable_boxes(g: &Grid<()>, robot: Point, d: Direction) -> Option<FnvHashSet<Point>> {
    let mut boxes = makeset!(robot.offset_dir(d));
    let mut prev_size = 0;
    while prev_size != boxes.len() {
        prev_size = boxes.len();
        let mut b2 = makeset!();
        for b in boxes {
            let mut v = vec![b];
            if g.read_pt(&b) == ']' {
                v.push(b.offset_dir(Direction::W));
            } else if g.read_pt(&b) == '[' {
                v.push(b.offset_dir(Direction::E));
            }
            if let Some(c) = g.drive(b, d).map(|p| g.read_pt(&p)) {
                if c == ']' || c == '[' {
                    v.push(g.drive(b, d).unwrap());
                }
            } else {
                return None;
            }
            b2.extend(v);
        }
        boxes = b2;
    }
    Some(boxes)
}

#[test]
fn test_large() {
    let s = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;
    assert_eq!("10092", part1(s.to_string()));
    assert_eq!("9021", part2(s.to_string()));
}

#[test]
fn test_small() {
    let s = r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"#;
    assert_eq!("2028", part1(s.to_string()));
}

#[test]
fn test_wide_small() {
    let s = r#"#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^"#;
    assert_eq!("618", part2(s.to_string()));
}
