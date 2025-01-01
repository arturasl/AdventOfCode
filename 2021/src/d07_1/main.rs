use anyhow::{Ok, Result};
use std::io::{self, BufRead};
use std::thread;

fn run() -> Result<()> {
    for maybe_line in io::stdin().lock().lines() {
        let mut nums: Vec<i64> = maybe_line?
            .replace(" ", "")
            .split(",")
            .map(|x| Ok(x.parse::<i64>()?))
            .collect::<Result<_>>()?;
        if nums.is_empty() {
            continue;
        }
        nums.sort_unstable();

        let mut rhs_sum: i64 = nums.iter().sum();
        let mut lhs_sum: i64 = 0;
        let mut best = i64::MAX;

        for (i, num) in nums.iter().enumerate() {
            rhs_sum -= num;
            best = best
                .min(rhs_sum - num * ((nums.len() - i - 1) as i64) + num * (i as i64) - lhs_sum);
            lhs_sum += num;
        }

        println!("{}", best);
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
