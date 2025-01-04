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
        let (xmi, xma) = (
            captures["xmin"].parse::<i64>()?,
            captures["xmax"].parse::<i64>()?,
        );
        let (ymi, yma) = (
            captures["ymin"].parse::<i64>()?,
            captures["ymax"].parse::<i64>()?,
        );

        let mut by = 0;
        for vx in 0..xma + 1 {
            for vy in ymi..ymi.abs() + 1 {
                let (mut lvy, mut lvx) = (vy, vx);
                let (mut sx, mut sy) = (0, 0);
                let mut local_by = 0;
                while sy >= ymi && sx <= xma {
                    sx += lvx;
                    sy += lvy;
                    lvx = (lvx - 1).max(0);
                    lvy -= 1;
                    local_by = local_by.max(sy);
                    if xmi <= sx && sx <= xma && ymi <= sy && sy <= yma {
                        by = by.max(local_by);
                    }
                }
            }
        }

        println!("{}", by);
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
