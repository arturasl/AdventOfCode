use anyhow::{ensure, Context, Ok, Result};
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};
use std::thread;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    cost: i64,
    y: usize,
    x: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn expand(map: Vec<Vec<i64>>, default: i64) -> Vec<Vec<i64>> {
    let cols: usize = map[0].len();

    vec![vec![default; cols + 2]]
        .into_iter()
        .chain(map.into_iter().map(|r| {
            vec![default]
                .into_iter()
                .chain(r)
                .chain(vec![default])
                .collect::<Vec<i64>>()
        }))
        .chain(vec![vec![default; cols + 2]])
        .collect::<Vec<Vec<i64>>>()
}

fn run() -> Result<()> {
    let mut map = expand(
        io::stdin()
            .lock()
            .lines()
            .map(|l| Ok(l?.trim().to_owned()))
            .filter_ok(|l| !l.is_empty())
            .map(|l| {
                l.and_then(|l| {
                    Ok(l.chars()
                        .map(|c| Ok(c.to_digit(10).context("")? as i64))
                        .collect::<Result<Vec<i64>>>()?)
                })
            })
            .collect::<Result<Vec<Vec<i64>>>>()?,
        -1,
    );

    let mut heap = BinaryHeap::new();
    heap.push(State {
        cost: 0,
        y: 1,
        x: 1,
    });
    map[1][1] = -1;

    while let Some(cur_state) = heap.pop() {
        if (cur_state.y, cur_state.x) == (map.len() - 2, map[0].len() - 2) {
            println!("{}", cur_state.cost);
            return Ok(());
        }

        for dy in [-1i64, 0, 1] {
            for dx in [-1i64, 0, 1] {
                if !(dy == 0 || dx == 0) {
                    continue;
                }

                let (ny, nx) = (
                    (cur_state.y as i64 + dy) as usize,
                    (cur_state.x as i64 + dx) as usize,
                );

                if map[ny][nx] != -1 {
                    heap.push(State {
                        cost: map[ny][nx] + cur_state.cost,
                        y: ny,
                        x: nx,
                    });
                    map[ny][nx] = -1;
                }
            }
        }
    }

    ensure!(false);
    Ok(())
}

fn main() -> Result<()> {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)?
        .join()
        .unwrap()
}
