const INPUT: usize = 312051;

// 1 W, 1 N
// 2 E, 2 S
// 3 W, 3 N

enum Dir {
  N, E, S, W,
}

use Dir::*;

fn part_one() {
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut side: i64 = 1;
    let mut n = side;
    let mut dir = E;
    for _ in 1..INPUT {
        n -= 1;
        match dir {
            E => {
                x += 1;
                if n == 0 {
                    dir = N;
                    n = side;
                }
            },
            N => {
                y += 1;
                if n == 0 {
                    dir = W;
                    side += 1;
                    n = side;
                }
            },
            W => {
                x -= 1;
                if n == 0 {
                    n = side;
                    dir = S;
                }
            },
            S => {
                y -= 1;
                if n == 0 {
                    side += 1;
                    n = side;
                    dir = E;
                }
            },
        };
    }
    println!("{}", x.abs() + y.abs());
}

fn part_two() {
    let g = 1_000;
    let mut grid = vec![vec![0; g]; g];
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut side: i64 = 1;
    let mut n = side;
    let mut dir = E;
    for _ in 1.. {
        let i = (x+g as i64/2) as usize;
        let j = (y+g as i64/2) as usize;
        if x == 0 && y == 0 {
            grid[i][j] = 1;
        } else {
            grid[i][j] =
                grid[i-1][j] + grid[i+1][j] + grid[i][j-1] + grid[i][j+1] +
                grid[i-1][j-1] + grid[i-1][j+1] + grid[i+1][j-1] + grid[i+1][j+1];
        }
        // println!("{},{} => {}", x, y, grid[i][j]);
        if grid[i][j] > INPUT {
            println!("{}", grid[i][j]);
            break
        }
        n -= 1;
        match dir {
            E => {
                x += 1;
                if n == 0 {
                    dir = N;
                    n = side;
                }
            },
            N => {
                y += 1;
                if n == 0 {
                    dir = W;
                    side += 1;
                    n = side;
                }
            },
            W => {
                x -= 1;
                if n == 0 {
                    n = side;
                    dir = S;
                }
            },
            S => {
                y -= 1;
                if n == 0 {
                    side += 1;
                    n = side;
                    dir = E;
                }
            },
        };
    }
}

fn main() {
  part_one();
  part_two();
}
