use anyhow::{Context, Ok, Result};
use itertools::Itertools;
use std::io::{self, BufRead};
use std::thread;

fn expand(map: Vec<Vec<i64>>, default: i64) -> Vec<Vec<i64>> {
    let cols: usize = map[0].len();

    vec![vec![default; cols + 2]]
        .into_iter()
        .chain(map.into_iter().map(|r| {
            vec![default]
                .into_iter()
                .chain(r)
                .chain(vec![default])
                .collect::<Vec<i64>>()
        }))
        .chain(vec![vec![default; cols + 2]])
        .collect::<Vec<Vec<i64>>>()
}

fn run() -> Result<()> {
    let map = expand(
        io::stdin()
            .lock()
            .lines()
            .map(|l| Ok(l?.trim().to_owned()))
            .filter_ok(|l| !l.is_empty())
            .map(|l| {
                l.and_then(|l| {
                    Ok(l.chars()
                        .map(|c| Ok(c.to_digit(10).context("")? as i64))
                        .collect::<Result<Vec<i64>>>()?)
                })
            })
            .collect::<Result<Vec<Vec<i64>>>>()?,
        10,
    );

    let around: Vec<(i64, i64)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut result = 0;

    for y in 1..map.len() - 1 {
        for x in 1..map[y].len() - 1 {
            if around
                .iter()
                .map(|(dy, dx)| ((y as i64) + dy, (x as i64) + dx))
                .map(|(ny, nx)| map[ny as usize][nx as usize])
                .all(|num| num > map[y][x])
            {
                result += map[y][x] + 1;
            }
        }
    }

    println!("{}", result);

    Ok(())
}

fn main() -> Result<()> {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)?
        .join()
        .unwrap()
}
