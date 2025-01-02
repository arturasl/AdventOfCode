use anyhow::{Context, Ok, Result};
use itertools::Itertools;
use std::io::{self, BufRead};
use std::thread;

const AROUND: [(i64, i64); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

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

fn calc_size(cur: (usize, usize), map: &mut Vec<Vec<i64>>) -> i64 {
    let val = map[cur.0][cur.1];
    map[cur.0][cur.1] = 10;

    if val >= 9 {
        return 0;
    }

    let mut result = 1;
    for (dy, dx) in AROUND {
        let (ny, nx) = (
            ((cur.0) as i64 + dy) as usize,
            ((cur.1 as i64) + dx) as usize,
        );
        if map[ny][nx] > val {
            result += calc_size((ny, nx), map);
        }
    }

    return result;
}

fn run() -> Result<()> {
    let mut map = expand(
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

    let mut sizes: Vec<i64> = vec![];
    for y in 1..map.len() - 1 {
        for x in 1..map[y].len() - 1 {
            if AROUND
                .iter()
                .map(|(dy, dx)| ((y as i64) + dy, (x as i64) + dx))
                .map(|(ny, nx)| map[ny as usize][nx as usize])
                .all(|num| num > map[y][x])
            {
                sizes.push(calc_size((y, x), &mut map));
            }
        }
    }

    sizes.select_nth_unstable_by(3, |x, y| y.cmp(x));
    println!(
        "{}",
        sizes.into_iter().take(3).fold(1, |prev, cur| prev * cur)
    );

    Ok(())
}

fn main() -> Result<()> {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)?
        .join()
        .unwrap()
}
