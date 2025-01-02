use ahash::AHashMap;
use anyhow::{ensure, Context, Ok, Result};
use std::io::{self, BufRead};
use std::thread;

fn run() -> Result<()> {
    let open_to_points: AHashMap<char, i64> = [('(', 1), ('[', 2), ('{', 3), ('<', 4)]
        .into_iter()
        .collect();
    let close_to_open: AHashMap<char, char> = [(')', '('), (']', '['), ('}', '{'), ('>', '<')]
        .into_iter()
        .collect();

    let mut results: Vec<i64> = vec![];
    for maybe_line in io::stdin().lock().lines() {
        let line = maybe_line?.trim().to_owned();
        if line.is_empty() {
            continue;
        }

        let mut stack: Vec<char> = vec![];
        let mut ok = true;
        for c in line.chars() {
            if close_to_open.contains_key(&c) {
                let top = stack.pop().unwrap_or('z');
                if *close_to_open.get(&c).context("")? != top {
                    ok = false;
                    break;
                }
            } else {
                stack.push(c);
            }
        }

        if ok {
            results.push(stack.into_iter().rev().try_fold(0, |prev, char| {
                Ok(prev * 5 + open_to_points.get(&char).context(format!("{}", char))?)
            })?);
        }
    }

    ensure!(results.len() % 2 == 1);
    let middle = results.len() / 2;
    results.select_nth_unstable(middle);
    println!("{:?}", results[middle]);

    Ok(())
}

fn main() -> Result<()> {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)?
        .join()
        .unwrap()
}
