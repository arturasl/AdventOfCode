use anyhow::{Context, Ok, Result};
use itertools::Itertools;
use std::io::{self, BufRead};
use std::thread;

fn print_map(map: &Vec<Vec<i64>>) {
    for line in map.iter() {
        for char in line.iter() {
            print!("{}", char);
        }
        println!("");
    }
}

fn next(mut map: Vec<Vec<i64>>) -> (i64, Vec<Vec<i64>>) {
    let mut flashed: Vec<Vec<bool>> = vec![];
    for y in 0..map.len() {
        flashed.push(vec![false; map[y].len()]);
        for x in 0..map[y].len() {
            map[y][x] += 1;
        }
    }

    let mut changed = true;
    while changed {
        changed = false;

        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if map[y][x] < 10 || flashed[y][x] {
                    continue;
                }
                flashed[y][x] = true;
                changed = true;

                for dy in -1i64..1 + 1 {
                    for dx in -1i64..1 + 1 {
                        if dy == 0 && dx == 0 {
                            continue;
                        }
                        let (ny, nx) = ((y as i64) + dy, (x as i64) + dx);
                        if !(0 <= ny
                            && ny < (map.len() as i64)
                            && 0 <= nx
                            && nx < (map[y].len() as i64))
                        {
                            continue;
                        }

                        map[ny as usize][nx as usize] += 1;
                    }
                }
            }
        }
    }

    let mut num_flashes = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if flashed[y][x] {
                map[y][x] = 0;
                num_flashes += 1;
            }
        }
    }

    (num_flashes, map)
}

fn run() -> Result<()> {
    let mut map: Vec<Vec<i64>> = io::stdin()
        .lock()
        .lines()
        .map(|maybe_line| {
            Ok(maybe_line?
                .trim()
                .chars()
                .map(|ch| Ok(ch.to_digit(10).context("")? as i64))
                .collect::<Result<Vec<i64>>>()?)
        })
        .filter_ok(|chars| !chars.is_empty())
        .collect::<Result<_>>()?;

    for i in 0..1000 {
        let local_flashes;
        (local_flashes, map) = next(map);
        if local_flashes == 100 {
            println!("{}", i + 1);
            break;
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)?
        .join()
        .unwrap()
}
