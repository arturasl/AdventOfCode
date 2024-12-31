use anyhow::{Context, Ok, Result};
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::thread;

fn run() -> Result<()> {
    let re_path = Regex::new(r"^(\d+),(\d+)->(\d+),(\d+)$").unwrap();
    let mut hits: HashMap<(i64, i64), i64> = HashMap::new();
    for maybe_line in io::stdin().lock().lines() {
        let line = maybe_line?.replace(" ", "");
        if line.is_empty() {
            continue;
        }
        let captures = re_path.captures(&line).context("matching regex")?;
        let (x1, y1, x2, y2) = captures
            .iter()
            .skip(1)
            .filter(Option::is_some)
            .map(|c| c.unwrap().as_str().parse::<i64>().unwrap())
            .collect_tuple()
            .context("Should have had 4 numbers")?;

        if !(y1 == y2 || x1 == x2) {
            continue;
        }

        for y in y1.min(y2)..y1.max(y2) + 1 {
            for x in x1.min(x2)..x1.max(x2) + 1 {
                *hits.entry((y, x)).or_insert(0) += 1;
            }
        }
    }

    println!("{}", hits.into_values().filter(|v| *v >= 2).count());

    Ok(())
}

fn main() -> Result<()> {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)?
        .join()
        .unwrap()
}
