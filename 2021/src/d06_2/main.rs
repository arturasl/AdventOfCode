use anyhow::{Ok, Result};
use memoize::memoize;
use std::io::{self, BufRead};
use std::thread;

const WAIT_TIME: i64 = 2;
const RESET_TIME: i64 = 7;
const TOTAL_DAYS: i64 = 256;

#[memoize]
fn calc(cur: i64, days_left: i64) -> i64 {
    if days_left == 0 {
        return 1;
    }

    let mut result = 0;
    let mut next = cur - 1;
    if next == -1 {
        next = RESET_TIME - 1;
        result += calc(RESET_TIME + WAIT_TIME - 1, days_left - 1);
    }
    result + calc(next, days_left - 1)
}

fn run() -> Result<()> {
    for maybe_line in io::stdin().lock().lines() {
        let nums: Vec<i64> = maybe_line?
            .replace(" ", "")
            .split(",")
            .map(|x| Ok(x.parse::<i64>()?))
            .collect::<Result<_>>()?;
        if nums.is_empty() {
            continue;
        }

        println!(
            "{}",
            nums.into_iter().map(|x| calc(x, TOTAL_DAYS)).sum::<i64>()
        );
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
