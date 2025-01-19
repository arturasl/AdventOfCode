use anyhow::{Ok, Result};
use std::io::{self, BufRead};
use std::thread;

fn get(map: &mut [Vec<char>], y: i64, x: i64) -> &mut char {
    let (height, width) = (map.len() as i64, map[0].len() as i64);
    &mut map[(y % height) as usize][(x % width) as usize]
}

fn apply(ch: char, dy: i64, dx: i64, map: &mut [Vec<char>]) -> i64 {
    let (height, width) = (map.len() as i64, map[0].len() as i64);
    let mut moves: Vec<(i64, i64)> = Vec::new();
    for y in 0..height {
        for x in 0..width {
            if *get(map, y, x) == ch && *get(map, y + dy, x + dx) == '.' {
                moves.push((y, x));
            }
        }
    }
    for (y, x) in moves.iter() {
        *get(map, *y, *x) = '.';
        *get(map, *y + dy, *x + dx) = ch;
    }

    moves.len() as i64
}

fn run() -> Result<()> {
    let mut map: Vec<Vec<char>> = Vec::new();
    for maybe_line in io::stdin().lock().lines() {
        let line: Vec<char> = maybe_line?.replace(" ", "").chars().collect();
        if line.is_empty() {
            continue;
        }

        map.push(line);
    }

    let mut days: i64 = 0;

    loop {
        let moved = apply('>', 0, 1, &mut map) + apply('v', 1, 0, &mut map);
        if moved == 0 {
            break;
        }
        days += 1;
    }

    println!("{}", days + 1);

    Ok(())
}

fn main() -> Result<()> {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)?
        .join()
        .unwrap()
}
