use ahash::AHashMap;
use anyhow::{Context, Ok, Result};
use std::io::{self, BufRead};
use std::thread;

fn run() -> Result<()> {
    let close_to_points: AHashMap<char, i64> = [(')', 3), (']', 57), ('}', 1197), ('>', 25137)]
        .into_iter()
        .collect();
    let close_to_open: AHashMap<char, char> = [(')', '('), (']', '['), ('}', '{'), ('>', '<')]
        .into_iter()
        .collect();

    let mut result = 0;
    for maybe_line in io::stdin().lock().lines() {
        let line = maybe_line?.trim().to_owned();
        if line.is_empty() {
            continue;
        }

        let mut stack: Vec<char> = vec![];
        for c in line.chars() {
            if close_to_open.contains_key(&c) {
                let top = stack.pop().unwrap_or('z');
                if *close_to_open.get(&c).context("")? != top {
                    result += close_to_points.get(&c).context("")?;
                    break;
                }
            } else {
                stack.push(c);
            }
        }
    }
    println!("{}", result);

    Ok(())
}

fn main() -> Result<()> {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)?
        .join()
        .unwrap()
}
