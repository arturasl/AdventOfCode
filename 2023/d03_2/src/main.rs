use regex::Regex;
use std::error::Error;
use std::io::{self, BufRead};

const DIGITS: &str = "0123456789";

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}

fn expand(
    grid: &mut Vec<Vec<char>>,
    initial: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    bx: &mut Vec<usize>,
) {
    if grid[i][j] == initial[i][j] {
        return;
    }
    grid[i][j] = initial[i][j];
    bx[0] = bx[0].min(i);
    bx[1] = bx[1].max(i);
    bx[2] = bx[2].min(j);
    bx[3] = bx[3].max(j);

    let ii = i as i64;
    let jj = j as i64;

    for i_oft in -1i64..1 + 1 {
        for j_oft in -1i64..1 + 1 {
            if ii + i_oft < 0 || jj + j_oft < 0 {
                continue;
            }
            if ii + i_oft >= (grid.len() as i64) || jj + j_oft >= (grid[i].len() as i64) {
                continue;
            }
            if !DIGITS.contains(initial[(ii + i_oft) as usize][(jj + j_oft) as usize]) {
                continue;
            }
            expand(
                grid,
                initial,
                (ii + i_oft) as usize,
                (jj + j_oft) as usize,
                bx,
            );
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut initial: Vec<Vec<char>> = vec![];
    for maybe_line in io::stdin().lock().lines() {
        initial.push(maybe_line?.trim().chars().collect());
    }

    let mut visited: Vec<Vec<char>> = vec![];
    {
        let mut tmp: Vec<char> = vec![];
        tmp.resize(initial[0].len(), '.');
        visited.resize(initial.len(), tmp);
    }

    let mut result: i64 = 0;

    for i in 0..initial.len() {
        for j in 0..initial[i].len() {
            if initial[i][j] != '*' {
                continue;
            }

            let mut bx: Vec<usize> = vec![i, i, j, j];
            expand(&mut visited, &initial, i, j, &mut bx);

            let mut slice: Vec<Vec<char>> = vec![];
            for ii in bx[0]..bx[1] + 1 {
                slice.push(vec![]);
                for jj in bx[2]..bx[3] + 1 {
                    slice[ii - bx[0]].push(visited[ii][jj]);
                    visited[ii][jj] = '.';
                }
            }

            println!("# Group");
            print_grid(&slice);

            let single: String = slice
                .iter()
                .map(|x| x.iter().collect::<String>())
                .collect::<Vec<String>>()
                .join(" ");

            println!("single: {}", single);

            let numbers: Vec<i64> = Regex::new(r"\d+")?
                .find_iter(&single)
                .map(|x| x.as_str().parse::<i64>().unwrap())
                .collect();

            println!("numbers: {:?}", numbers);

            if numbers.len() != 2 {
                continue;
            }

            result += numbers[0] * numbers[1];
        }
    }

    println!("{}", result);

    Ok(())
}
