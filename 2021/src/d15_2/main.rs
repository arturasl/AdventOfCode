use anyhow::{ensure, Context, Ok, Result};
use itertools::Itertools;
use std::cmp::Ordering;
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
    side: i8,
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

type Visited = Vec<Vec<Vec<Vec<(i64, i8)>>>>;

fn get_visited<'a>(state: &State, visited: &'a mut Visited) -> &'a mut (i64, i8) {
    &mut visited[state.ry as usize][state.y as usize][state.rx as usize][state.x as usize]
}

fn update_visited(state: &State, visited: &mut Visited) -> bool {
    let prev = get_visited(state, visited);
    if prev.1 & state.side == state.side {
        return false;
    }

    prev.0 += state.cost;
    prev.1 |= state.side;
    true
}

fn get_map_val(state: &State, map: &[Vec<i64>]) -> i64 {
    (map[state.y as usize][state.x as usize] - 1 + state.ry + state.rx) % 9 + 1
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
    let mut visited: Visited =
        vec![
            vec![vec![vec![(0, 0); width as usize]; MAX_REPEATS as usize + 1]; height as usize];
            MAX_REPEATS as usize + 1
        ];

    let start_state = State {
        cost: 0,
        ry: 0,
        y: 0,
        rx: 0,
        x: 0,
        side: 1,
    };
    update_visited(&start_state, &mut visited);
    heap.push(start_state);

    let mut end_state = State {
        cost: 0,
        ry: MAX_REPEATS,
        y: height - 1,
        rx: MAX_REPEATS,
        x: width - 1,
        side: 2,
    };
    end_state.cost = get_map_val(&end_state, &map);
    update_visited(&end_state, &mut visited);
    heap.push(end_state);

    let dirs: Vec<(i64, i64)> = [-1i64, 0, 1]
        .into_iter()
        .cartesian_product([-1i64, 0, 1])
        .filter(|(dy, dx)| (*dy == 0) ^ (*dx == 0))
        .collect();

    let mut its = 0;
    while let Some(cur_state) = heap.pop() {
        its += 1;
        let cur_val = get_visited(&cur_state, &mut visited);
        if cur_val.1 == 3 {
            println!("Its: {}", its);
            println!("{}", cur_val.0 - get_map_val(&cur_state, &map));
            return Ok(());
        }

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

            next_state.cost += get_map_val(&next_state, &map);
            if !update_visited(&next_state, &mut visited) {
                continue;
            }

            //println!();
            //println!("#####################");
            //for ry in 0..MAX_REPEATS + 1 {
            //    for y in 0..height {
            //        for rx in 0..MAX_REPEATS + 1 {
            //            for x in 0..width {
            //                print!(
            //                    "{:02} ",
            //                    visited[ry as usize][y as usize][rx as usize][x as usize].0
            //                );
            //            }
            //        }
            //        println!();
            //    }
            //}

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
