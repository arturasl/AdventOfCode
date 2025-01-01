use ahash::AHashMap;
use anyhow::{Context, Ok, Result};
use itertools::Itertools;
use std::io::{self, BufRead};
use std::thread;

fn find_distances(nums: &Vec<i64>) -> Result<Vec<i64>> {
    let (mi, ma) = (
        nums.iter().min().context("")?,
        nums.iter().max().context("")?,
    );

    let mut hist: AHashMap<i64, i64> = AHashMap::new();
    for num in nums.iter() {
        *hist.entry(*num).or_insert(0) += 1;
    }

    let mut result: Vec<i64> = vec![];
    let mut lhs_sum = 0;
    let mut lhs_cnt = 0;
    let mut lhs_result = 0;
    for i in *mi..(ma + 1) {
        lhs_result += i * lhs_cnt - lhs_sum;
        result.push(lhs_result);
        if let Some(cnt) = hist.get(&i) {
            lhs_sum += cnt * i;
            lhs_cnt += cnt;
        }
    }

    Ok(result)
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

        let distances = find_distances(&nums)?;

        let ma = nums.iter().max().context("")?;
        let rev_nums: Vec<i64> = nums.iter().map(|x| ma - x).collect();
        let rev_distances: Vec<i64> = find_distances(&rev_nums)?.into_iter().rev().collect();

        let total_distances: Vec<i64> = distances
            .into_iter()
            .zip_eq(rev_distances.into_iter())
            .map(|(x, y)| x + y)
            .collect();

        println!("{}", total_distances.iter().min().context("")?);
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
