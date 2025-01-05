use anyhow::{Context, Ok, Result};
use regex::Regex;
use std::io::{self, BufRead};
use std::thread;
fn run() -> Result<()> {
    let re_inp = Regex::new(
        r"^target area: x=(?<xmin>-?\d+)..(?<xmax>-?\d+), y=(?<ymin>-?\d+)..(?<ymax>-?\d+$)",
    )?;
    for maybe_line in io::stdin().lock().lines() {
        let line = maybe_line?.trim().to_owned();
        if line.is_empty() {
            continue;
        }

        let captures = re_inp.captures(&line).context("")?;
        let ymi = captures["ymin"].parse::<i64>()?;
        println!("{}", (ymi * (ymi + 1)) / 2);
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
