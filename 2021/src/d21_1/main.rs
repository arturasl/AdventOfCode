use anyhow::{bail, ensure, Context, Ok, Result};
use itertools::Itertools;
use regex::Regex;
use std::io::{self, BufRead};
use std::iter::successors;
use std::thread;

fn run() -> Result<()> {
    const DIE_SIDES: i64 = 100;
    const DIE_THROWS: usize = 3;
    const PLAYERS: usize = 2;
    const CYCLE: i64 = 10;

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

    let mut scores: [i64; PLAYERS] = [0; PLAYERS];
    for (i, roll) in successors(Some(1), |n| Some(n % DIE_SIDES + 1))
        .chunks(DIE_THROWS)
        .into_iter()
        .map(|c| c.sum::<i64>())
        .enumerate()
    {
        let player_idx = i % PLAYERS;
        let next_pos = (poses[player_idx] + roll - 1) % CYCLE + 1;
        poses[player_idx] = next_pos;
        scores[player_idx] += next_pos;
        if scores[player_idx] >= 1000 {
            let total_throws = ((i + 1) * DIE_THROWS) as i64;
            let other_score = scores[(player_idx + 1) % PLAYERS];
            println!(
                "{} = {} * {}",
                total_throws * other_score,
                total_throws,
                other_score
            );
            break;
        }
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
