use anyhow::{ensure, Context, Ok, Result};
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};
use std::thread;

const MAX_REPEATS: usize = 4;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    cost: i64,
    ry: usize,
    y: usize,
    rx: usize,
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

fn run() -> Result<()> {
    let init_map = io::stdin()
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
        .collect::<Result<Vec<Vec<i64>>>>()?;
    let (height, width) = (init_map.len(), init_map[0].len());

    let mut map: Vec<Vec<Vec<Vec<i64>>>> = vec![];
    for ry in 0..MAX_REPEATS + 1 {
        map.push(Vec::default());
        for (y, row) in init_map.iter().enumerate() {
            map[ry].push(Vec::default());
            for rx in 0..MAX_REPEATS + 1 {
                map[ry][y].push(Vec::default());
                for cell in row.iter() {
                    map[ry][y][rx].push((cell - 1 + (ry as i64) + (rx as i64)) % 9 + 1);
                }
            }
        }
    }

    let mut heap = BinaryHeap::new();
    heap.push(State {
        cost: 0,
        ry: 0,
        y: 0,
        rx: 0,
        x: 0,
    });
    map[0][0][0][0] = -1;

    while let Some(cur_state) = heap.pop() {
        if (cur_state.ry, cur_state.y, cur_state.rx, cur_state.x)
            == (MAX_REPEATS, height - 1, MAX_REPEATS, width - 1)
        {
            println!("{:?}", cur_state);
            println!("{}", cur_state.cost);
            return Ok(());
        }

        for dy in [-1i64, 0, 1] {
            for dx in [-1i64, 0, 1] {
                if !(dy == 0 || dx == 0) {
                    continue;
                }

                let (mut nry, ny, mut nrx, nx) = (
                    cur_state.ry,
                    (cur_state.y as i64 + dy),
                    cur_state.rx,
                    (cur_state.x as i64 + dx),
                );
                if ny < 0 || nx < 0 {
                    continue;
                }
                let (mut ny, mut nx) = (ny as usize, nx as usize);

                if ny == height {
                    ny = 0;
                    nry += 1;
                }
                if nx == width {
                    nx = 0;
                    nrx += 1;
                }

                if nry > MAX_REPEATS || nrx > MAX_REPEATS {
                    continue;
                }

                if map[nry][ny][nrx][nx] == -1 {
                    continue;
                }

                heap.push(State {
                    cost: map[nry][ny][nrx][nx] + cur_state.cost,
                    ry: nry,
                    y: ny,
                    rx: nrx,
                    x: nx,
                });
                map[nry][ny][nrx][nx] = -1;
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
