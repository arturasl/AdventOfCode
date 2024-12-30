use anyhow::{ensure, Ok, Result};
use std::io::{self, BufRead};
use std::thread;

fn run() -> Result<()> {
    let mut hist: Vec<i64> = vec![];
    let mut total: i64 = 0;
    for maybe_line in io::stdin().lock().lines() {
        let chars: Vec<char> = maybe_line?.trim().chars().collect();
        if chars.is_empty() {
            continue;
        }

        hist.resize(chars.len(), 0);
        total += 1;
        for (i, char) in chars.into_iter().enumerate() {
            hist[i] += i64::from(char == '1');
        }
    }

    let mut max: i64 = 0;
    for ones in &hist {
        ensure!(*ones != total - ones);
        max = (max << 1) | i64::from(*ones > total - ones);
    }
    let min = ((1 << hist.len()) - 1) ^ max;

    println!("{}", max * min);

    Ok(())
}

fn main() -> Result<()> {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)?
        .join()
        .unwrap()
}
