use anyhow::{bail, Context, Ok, Result};
use itertools::Itertools;
use regex::Regex;
use resiter::AndThen;
use std::io::{self, BufRead};
use std::thread;

#[derive(Debug)]
struct Pos {
    h_pos: i64,
    depth: i64,
}

fn run() -> Result<()> {
    let re_dir = Regex::new(r"^(?<name>(forward|down|up))\s+(?<amount>\d+)$")?;
    let pos: Pos = io::stdin()
        .lock()
        .lines()
        .map(|l| Ok(l?.trim().to_owned()))
        .and_then_ok(|l| {
            let captures = re_dir.captures(&l).context("")?;
            let amount: i64 = captures["amount"].parse()?;
            Ok(match &captures["name"] {
                "forward" => Pos {
                    h_pos: amount,
                    depth: 0,
                },
                "down" => Pos {
                    h_pos: 0,
                    depth: amount,
                },
                "up" => Pos {
                    h_pos: 0,
                    depth: -amount,
                },
                _ => bail!(""),
            })
        })
        .fold_ok(Pos { h_pos: 0, depth: 0 }, |prev, cur| Pos {
            h_pos: prev.h_pos + cur.h_pos,
            depth: prev.depth + cur.depth,
        })?;

    println!("{}", pos.h_pos * pos.depth);

    Ok(())
}

fn main() -> Result<()> {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)?
        .join()
        .unwrap()
}
