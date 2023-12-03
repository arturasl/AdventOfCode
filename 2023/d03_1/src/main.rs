use regex::Regex;
use std::error::Error;
use std::io::{self, BufRead};

const DIGITS: &str = "0123456789";

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}

fn expand(grid: &mut Vec<Vec<char>>, initial: &Vec<Vec<char>>, i: usize, j: usize) {
    if grid[i][j] == initial[i][j] {
        return;
    }
    grid[i][j] = initial[i][j];

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
            if initial[(ii + i_oft) as usize][(jj + j_oft) as usize] == '.' {
                continue;
            }
            expand(grid, initial, (ii + i_oft) as usize, (jj + j_oft) as usize);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut initial: Vec<Vec<char>> = vec![];
    for maybe_line in io::stdin().lock().lines() {
        initial.push(maybe_line?.trim().chars().collect());
    }

    print_grid(&initial);

    let mut visited: Vec<Vec<char>> = vec![];
    {
        let mut tmp: Vec<char> = vec![];
        tmp.resize(initial[0].len(), '.');
        visited.resize(initial.len(), tmp);
    }

    for i in 0..initial.len() {
        for j in 0..initial[i].len() {
            if initial[i][j] != '.' && !DIGITS.contains(initial[i][j]) {
                expand(&mut visited, &initial, i, j);
            }
        }
    }

    print_grid(&visited);

    let single: String = visited
        .iter()
        .map(|x| x.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ");

    println!("{}", single);

    let result: i64 = Regex::new(r"\d+")?
        .find_iter(&single)
        .map(|x| x.as_str().parse::<i64>().unwrap())
        .sum();

    println!("{}", result);

    Ok(())
}
