use anyhow::{bail, Context, Ok, Result};
use regex::Regex;
use std::io::{self, BufRead};
use std::thread;

#[derive(Debug)]
struct Pos {
    h_pos: i64,
    depth: i64,
    aim: i64,
}

fn run() -> Result<()> {
    let re_dir = Regex::new(r"^(?<name>(forward|down|up))\s+(?<amount>\d+)$")?;
    let pos: Pos = io::stdin()
        .lock()
        .lines()
        .map(|l| Ok(l?.trim().to_owned()))
        .map(|l| {
            l.and_then(|l| {
                let captures = re_dir.captures(&l).context("")?;
                Ok((
                    captures["name"].to_owned(),
                    captures["amount"].parse::<i64>()?,
                ))
            })
        })
        .fold(
            Ok(Pos {
                h_pos: 0,
                depth: 0,
                aim: 0,
            }),
            |prev, cur| {
                prev.and_then(|prev| cur.map(|cur| (prev, cur))).and_then(
                    |(mut prev, (cur_action, cur_amount))| {
                        match cur_action.as_str() {
                            "down" => prev.aim += cur_amount,
                            "up" => prev.aim -= cur_amount,
                            "forward" => {
                                prev.h_pos += cur_amount;
                                prev.depth += prev.aim * cur_amount;
                            }
                            _ => bail!(""),
                        };

                        Ok(prev)
                    },
                )
            },
        )?;

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
