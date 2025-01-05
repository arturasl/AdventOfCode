use ahash::AHashSet;
use anyhow::{Context, Ok, Result};
use regex::Regex;
use std::collections::BTreeMap;
use std::io::{self, BufRead};
use std::ops::Bound::{Included, Unbounded};
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

        let mut ok_vys: BTreeMap<i64, Vec<i64>> = BTreeMap::new();
        for vy in ymi..ymi.abs() {
            let (mut sy, mut lvy) = (0, vy);
            let mut time = 0;
            while sy >= ymi {
                sy += lvy;
                lvy -= 1;
                if ymi <= sy && sy <= yma {
                    ok_vys.entry(time).or_default().push(vy);
                }
                time += 1;
            }
        }

        let mut result = 0;
        let mut xstart = 0;
        while xstart * (1 + xstart) < 2 * xmi {
            xstart += 1;
        }

        for vx in xstart..xma + 1 {
            let (mut sx, mut lvx) = (0, vx);
            let mut time = 0;
            let mut hit_ys: AHashSet<i64> = AHashSet::new();
            while sx <= xma && lvx != 0 {
                sx += lvx;
                lvx = (lvx - 1).max(0);
                if xmi <= sx && sx <= xma {
                    if lvx == 0 {
                        for (_, vys) in ok_vys.range((Included(time), Unbounded)) {
                            hit_ys.extend(vys);
                        }
                    } else if let Some(vys) = ok_vys.get(&time) {
                        hit_ys.extend(vys);
                    }
                }
                time += 1;
            }

            result += hit_ys.len();
        }

        println!("{}", result);
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
