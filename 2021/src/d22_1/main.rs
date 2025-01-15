use anyhow::{Context, Ok, Result};
use regex::Regex;
use std::io::{self, BufRead};
use std::thread;

#[derive(Debug)]
struct Action {
    action: bool,
    xrng: (i64, i64),
    yrng: (i64, i64),
    zrng: (i64, i64),
}

fn run() -> Result<()> {
    let re_action = Regex::new(
        r"^(?<action>on|off) x=(?<xmin>-?\d+)..(?<xmax>-?\d+),y=(?<ymin>-?\d+)..(?<ymax>-?\d+),z=(?<zmin>-?\d+)..(?<zmax>-?\d+)$",
    )?;

    let mut actions: Vec<Action> = Vec::new();
    for maybe_line in io::stdin().lock().lines() {
        let line = maybe_line?.trim().to_owned();
        if line.is_empty() {
            continue;
        }

        let caps = re_action.captures(&line).context("")?;
        actions.push(Action {
            action: &caps["action"] == "on",
            xrng: (caps["xmin"].parse()?, caps["xmax"].parse()?),
            yrng: (caps["ymin"].parse()?, caps["ymax"].parse()?),
            zrng: (caps["zmin"].parse()?, caps["zmax"].parse()?),
        });
    }

    let mut lit: i64 = 0;
    for x in -50i64..=50 {
        for y in -50i64..=50 {
            for z in -50i64..=50 {
                for action in actions.iter().rev() {
                    if !(action.xrng.0..=action.xrng.1).contains(&x) {
                        continue;
                    }
                    if !(action.yrng.0..=action.yrng.1).contains(&y) {
                        continue;
                    }
                    if !(action.zrng.0..=action.zrng.1).contains(&z) {
                        continue;
                    }

                    lit += i64::from(action.action);
                    break;
                }
            }
        }
    }

    println!("{}", lit);

    Ok(())
}

fn main() -> Result<()> {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)?
        .join()
        .unwrap()
}
