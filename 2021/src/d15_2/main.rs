use ahash::AHashMap;
use anyhow::{ensure, Context, Ok, Result};
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};
use std::thread;

const MAX_REPEATS: i64 = 4;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    cost: i64,
    ry: i64,
    y: i64,
    rx: i64,
    x: i64,
    side: u8,
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

fn update_visited(
    state: &State,
    visited: &mut AHashMap<(i64, i64, i64, i64), (i64, u8)>,
) -> Option<(i64, u8)> {
    let entry = visited.entry((state.ry, state.y, state.rx, state.x));
    if let Entry::Occupied(prev) = entry {
        Some(*prev.get())
    } else {
        entry.or_insert((state.cost, state.side));
        None
    }
}

fn run() -> Result<()> {
    let map = io::stdin()
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
    let (height, width) = (map.len() as i64, map[0].len() as i64);

    let mut heap = BinaryHeap::new();
    let mut visited: AHashMap<(i64, i64, i64, i64), (i64, u8)> =
        AHashMap::with_capacity((height * width * MAX_REPEATS) as usize);

    let start_state = State {
        cost: 0,
        ry: 0,
        y: 0,
        rx: 0,
        x: 0,
        side: 0,
    };
    update_visited(&start_state, &mut visited);
    heap.push(start_state);

    let end_state = State {
        cost: map[(height - 1) as usize][(width - 1) as usize],
        ry: MAX_REPEATS,
        y: height - 1,
        rx: MAX_REPEATS,
        x: width - 1,
        side: 1,
    };
    update_visited(&end_state, &mut visited);
    //heap.push(end_state);

    let dirs: Vec<(i64, i64)> = [-1i64, 0, 1]
        .into_iter()
        .cartesian_product([-1i64, 0, 1])
        .filter(|(dy, dx)| (*dy == 0) ^ (*dx == 0))
        .collect();

    while let Some(cur_state) = heap.pop() {
        for (dy, dx) in dirs.iter() {
            let mut next_state = State {
                cost: cur_state.cost,
                ry: cur_state.ry,
                y: cur_state.y + dy,
                rx: cur_state.rx,
                x: cur_state.x + dx,
                side: cur_state.side,
            };

            if next_state.y == -1 {
                next_state.y = height - 1;
                next_state.ry -= 1;
            } else if next_state.y == height {
                next_state.y = 0;
                next_state.ry += 1;
            } else if next_state.x == -1 {
                next_state.x = width - 1;
                next_state.rx -= 1;
            } else if next_state.x == width {
                next_state.x = 0;
                next_state.rx += 1;
            }

            if !(0 <= next_state.ry
                && next_state.ry <= MAX_REPEATS
                && 0 <= next_state.rx
                && next_state.rx <= MAX_REPEATS)
            {
                continue;
            }

            let val = (map[next_state.y as usize][next_state.x as usize] - 1
                + next_state.ry
                + next_state.rx)
                % 9
                + 1;
            next_state.cost += val;

            if let Some((prev_cost, prev_side)) = update_visited(&next_state, &mut visited) {
                if prev_side != next_state.side {
                    println!("{}", prev_cost + cur_state.cost);
                    return Ok(());
                }
                continue;
            }

            heap.push(next_state);
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
