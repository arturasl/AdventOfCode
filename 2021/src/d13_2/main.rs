use ahash::AHashSet;
use anyhow::{ensure, Context, Ok, Result};
use regex::Regex;
use std::io::{self, BufRead};
use std::thread;

fn swap_if_x((a, b): (i64, i64), ch: char) -> (i64, i64) {
    if ch == 'x' {
        (b, a)
    } else {
        (a, b)
    }
}

fn run() -> Result<()> {
    let re_coord = Regex::new(r"^(?<x>\d+),(?<y>\d+)$").unwrap();
    let re_fold = Regex::new(r"^fold along (?<coord>[xy])=(?<pos>\d+)$").unwrap();

    let mut coords: Vec<(i64, i64)> = Vec::new();
    let mut folds: Vec<(char, i64)> = Vec::new();

    for maybe_line in io::stdin().lock().lines() {
        let line = maybe_line?.trim().to_owned();
        if line.is_empty() {
            continue;
        }

        if let Some(cap_coord) = re_coord.captures(&line) {
            coords.push((
                cap_coord["y"].parse::<i64>().context("")?,
                cap_coord["x"].parse::<i64>().context("")?,
            ));
        } else if let Some(cap_fold) = re_fold.captures(&line) {
            folds.push((
                cap_fold["coord"].to_string().chars().nth(0).context("")?,
                cap_fold["pos"].parse::<i64>().context("")?,
            ));
        } else {
            ensure!(false);
        }
    }

    let mut map: AHashSet<(i64, i64)> = coords.into_iter().collect();

    for (char, bend) in folds {
        let mut new_map: AHashSet<(i64, i64)> = AHashSet::new();
        for mut pos in map {
            pos = swap_if_x(pos, char);

            if pos.0 <= bend {
                pos = swap_if_x(pos, char);
                new_map.insert(pos);
                continue;
            }

            let mut n = (2 * bend - pos.0, pos.1);
            ensure!(n.0 >= 0 && n.0 < bend);

            n = swap_if_x(n, char);
            new_map.insert(n);
        }
        map = new_map;
    }

    let my = map.iter().map(|(y, _)| y).max().context("")?;
    let mx = map.iter().map(|(_, x)| x).max().context("")?;
    for y in 0..my + 1 {
        for x in 0..mx + 1 {
            print!("{}", if map.contains(&(y, x)) { '#' } else { '.' });
        }
        println!("");
    }

    println!("{}", map.len());
    Ok(())
}

fn main() -> Result<()> {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)?
        .join()
        .unwrap()
}
