use aoc::utils::*;
use cached::proc_macro::cached;

const A_COST: isize = 3;
const B_COST: isize = 1;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct Game {
    a_offsets: Point,
    b_offsets: Point,
    target: Point,
}

fn parse(s: &String) -> Vec<Game> {
    gimme_nums(s)
        .chunks(4)
        .map(|c| Game {
            a_offsets: Point::new(c[0][0], c[0][1]),
            b_offsets: Point::new(c[1][0], c[1][1]),
            target: Point::new(c[2][0], c[2][1]),
        })
        .collect()
}

#[cached]
fn solve_recursive(g: Game, curr: Point, moves_left: usize) -> Option<usize> {
    if moves_left == 0 && curr != g.target || curr.x > g.target.x || curr.y > g.target.y {
        return None;
    }
    if curr == g.target {
        return Some(0);
    }
    match (
        solve_recursive(
            g,
            curr.offset((g.a_offsets.x, g.a_offsets.y)),
            moves_left - 1,
        )
        .map(|c| c + A_COST as usize),
        solve_recursive(
            g,
            curr.offset((g.b_offsets.x, g.b_offsets.y)),
            moves_left - 1,
        )
        .map(|c| c + B_COST as usize),
    ) {
        (Some(a), Some(b)) => Some(a.min(b)),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        _ => None,
    }
}

pub fn part1(input: String) -> String {
    parse(&input)
        .into_iter()
        // .take(10)
        .filter_map(|g| solve_recursive(g, Point::new(0, 0), 200))
        .sum::<usize>()
        .to_string()
}

fn determinant_2x2(matrix: &[[isize; 2]; 2]) -> isize {
    matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0]
}

fn cramers_rule(matrix: &[[isize; 2]; 2], vector: &[isize; 2]) -> Option<[f64; 2]> {
    let det = determinant_2x2(matrix);
    if det == 0 {
        return None;
    }
    let mut result = [0.0; 2];
    for i in 0..2 {
        let mut m = matrix.clone();
        for j in 0..2 {
            m[j][i] = vector[j];
        }
        result[i] = determinant_2x2(&m) as f64 / det as f64;
    }
    // println!("{:?}", result);
    Some(result)
}

pub fn part2(input: String) -> String {
    let mut games = parse(&input);
    for g in &mut games {
        g.target.x += 10000000000000;
        g.target.y += 10000000000000;
    }
    games
        .iter()
        .filter_map(|g| {
            let matrix = [
                [g.a_offsets.x, g.b_offsets.x],
                [g.a_offsets.y, g.b_offsets.y],
            ];
            let vector = [g.target.x, g.target.y];
            cramers_rule(&matrix, &vector)
                .filter(|solution| {
                    solution[0].ceil() == solution[0].floor()
                        && solution[1].ceil() == solution[1].floor()
                })
                .map(|solution| solution[0] as isize * A_COST + solution[1] as isize * B_COST)
        })
        .sum::<isize>()
        .to_string()
}
