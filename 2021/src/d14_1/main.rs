use ahash::HashMapExt;
use anyhow::{ensure, Context, Ok, Result};
use itertools::Itertools;
use memoize::memoize;
use std::io::{self, BufRead};
use std::thread;

const DEPTH: usize = 10;

fn ch_to_idx(ch: char) -> usize {
    ch as usize - 'A' as usize
}

fn add(lhs: &Vec<usize>, rhs: &Vec<usize>) -> Vec<usize> {
    lhs.iter().zip_eq(rhs).map(|(a, b)| a + b).collect()
}

#[memoize(Ignore: conv, CustomHasher: ahash::HashMap)]
fn count(lhs: usize, rhs: usize, depth: usize, conv: &Vec<Vec<usize>>) -> Vec<usize> {
    let mid = conv[lhs][rhs];

    if depth == 0 || mid == usize::MAX {
        let mut result = vec![0; 26];
        result[lhs] += 1;
        return result;
    }

    add(
        &count(lhs, mid, depth - 1, conv),
        &count(mid, rhs, depth - 1, conv),
    )
}

fn run() -> Result<()> {
    let mut inp: Vec<usize> = Vec::new();
    let mut conv: Vec<Vec<usize>> = vec![vec![usize::MAX; 26]; 26];

    for maybe_line in io::stdin().lock().lines() {
        let line = maybe_line?.trim().to_owned();
        if line.is_empty() {
            continue;
        }

        if inp.is_empty() {
            inp = line.chars().map(ch_to_idx).collect();
        } else {
            let (lhs, rhs) = line
                .replace(" ", "")
                .split("->")
                .map(|s| s.chars().collect::<Vec<char>>())
                .collect_tuple()
                .context("")?;
            ensure!(lhs.len() == 2);
            ensure!(rhs.len() == 1);

            conv[ch_to_idx(lhs[0])][ch_to_idx(lhs[1])] = ch_to_idx(rhs[0]);
        }
    }

    let mut result = vec![0; 26];
    for i in 0..inp.len() - 1 {
        result = add(&result, &count(inp[i], inp[i + 1], DEPTH, &conv))
    }
    result[inp[inp.len() - 1]] += 1;

    result = result.into_iter().filter(|cnt| *cnt > 0).collect();
    let (mi, ma) = result.iter().minmax().into_option().context("")?;
    println!("{}", ma - mi);

    Ok(())
}

fn main() -> Result<()> {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)?
        .join()
        .unwrap()
}
