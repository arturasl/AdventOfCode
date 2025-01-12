use anyhow::{bail, ensure, Context, Ok, Result};
use itertools::Itertools;
use memoize::memoize;
use regex::Regex;
use std::io::{self, BufRead};
use std::thread;

const MAX_SCORE: i64 = 21;
const THROWS: usize = 3;
const DICE_SIDES: i64 = 3;
const PLAYERS: usize = 2;
const CYLCE: i64 = 10;

#[memoize(Ignore: rolls)]
fn calc(
    mut poses: [i64; PLAYERS],
    mut scores: [i64; PLAYERS],
    player_idx: usize,
    rolls: &[i64],
) -> [i64; PLAYERS] {
    let mut result = [0; PLAYERS];

    let next_player_idx = (player_idx + 1) % PLAYERS;
    if scores[next_player_idx] >= MAX_SCORE {
        result[next_player_idx] += 1;
        return result;
    }

    for rolled in rolls {
        let prev_pos = poses[player_idx];
        poses[player_idx] = (poses[player_idx] + rolled - 1) % CYLCE + 1;
        scores[player_idx] += poses[player_idx];
        for (i, r) in calc(poses, scores, next_player_idx, rolls)
            .into_iter()
            .enumerate()
        {
            result[i] += r;
        }
        scores[player_idx] -= poses[player_idx];
        poses[player_idx] = prev_pos;
    }

    result
}

fn run() -> Result<()> {
    let re_player = Regex::new(r"^Player (?<player_idx>\d+) starting position: (?<init_pos>\d+)$")?;
    let mut poses: [i64; PLAYERS] = [0; PLAYERS];

    for maybe_line in io::stdin().lock().lines() {
        let line = maybe_line?.trim().to_owned();
        if line.is_empty() {
            continue;
        }

        let captures_player = re_player.captures(&line).context("")?;
        let player_idx = captures_player["player_idx"].parse::<usize>().context("")?;
        let init_pos = captures_player["init_pos"].parse::<i64>().context("")?;
        if (1..=PLAYERS).contains(&player_idx) {
            ensure!(poses[player_idx - 1] == 0);
            poses[player_idx - 1] = init_pos;
        } else {
            bail!("");
        }
    }
    let rolls: Vec<i64> = (0..THROWS)
        .map(|_| (1..=DICE_SIDES))
        .multi_cartesian_product()
        .map(|v| v.into_iter().sum::<i64>())
        .collect();

    println!(
        "{:?}",
        calc(poses, [0; PLAYERS], 0, &rolls)
            .into_iter()
            .max()
            .context("")?
    );

    Ok(())
}

fn main() -> Result<()> {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)?
        .join()
        .unwrap()
}
