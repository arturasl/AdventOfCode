use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::io;
use std::thread;

fn expand(map: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let cols: usize = map[0].len();

    vec![vec![0i64; cols + 2]]
        .into_iter()
        .chain(map.into_iter().map(|r| {
            vec![0i64]
                .into_iter()
                .chain(r.into_iter())
                .chain(vec![0i64])
                .collect::<Vec<i64>>()
        }))
        .chain(vec![vec![0i64; cols + 2]])
        .collect::<Vec<Vec<i64>>>()
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct State {
    pos: (i64, i64),
    prev_pos: (i64, i64),
    same_dir: i64,
}

#[derive(Copy, Clone, Debug)]
struct StateWCost {
    state: State,
    cost: i64,
}

impl Ord for StateWCost {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for StateWCost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for StateWCost {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl Eq for StateWCost {}

fn run() {
    let map: Vec<Vec<i64>> = expand(
        io::stdin()
            .lines()
            .map(|x| {
                x.unwrap()
                    .trim()
                    .chars()
                    .map(|x| x.to_digit(10).unwrap() as i64)
                    .collect::<Vec<i64>>()
            })
            .filter(|x| !x.is_empty())
            .collect(),
    );
    let rows = map.len() as i64;
    let cols = map[0].len() as i64;

    let mut heap: BinaryHeap<StateWCost> = BinaryHeap::new();
    heap.push(StateWCost {
        state: State {
            pos: (1, 1),
            prev_pos: (-1, -1),
            same_dir: 5,
        },
        cost: 0,
    });

    let mut visited: HashSet<State> = HashSet::new();

    while let Some(cur) = heap.pop() {
        for dir in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let next_pos = (cur.state.pos.0 + dir.0, cur.state.pos.1 + dir.1);
            if next_pos.0 == 0
                || next_pos.1 == 0
                || next_pos.0 == rows - 1
                || next_pos.1 == cols - 1
                || next_pos == cur.state.prev_pos
            {
                continue;
            }
            let next_state = StateWCost {
                state: State {
                    pos: next_pos,
                    prev_pos: cur.state.pos,
                    same_dir: if (
                        cur.state.pos.0 - cur.state.prev_pos.0,
                        cur.state.pos.1 - cur.state.prev_pos.1,
                    ) == dir
                    {
                        cur.state.same_dir + 1
                    } else {
                        1
                    },
                },
                cost: cur.cost + map[next_pos.0 as usize][next_pos.1 as usize],
            };
            if next_state.state.same_dir == 1 && cur.state.same_dir <= 3 {
                continue;
            }
            if next_state.state.same_dir > 10 {
                continue;
            }
            if visited.contains(&next_state.state) {
                continue;
            }
            visited.insert(next_state.state.clone());
            if next_state.state.pos == (rows - 2, cols - 2) && next_state.state.same_dir >= 4 {
                println!("Result: {}", next_state.cost);
                heap.clear();
                break;
            }

            heap.push(next_state);
        }
    }
}

fn main() {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)
        .unwrap()
        .join()
        .unwrap();
}
