use ahash::{AHashMap, AHashSet};
use anyhow::{ensure, Context, Ok, Result};
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::collections::{BinaryHeap, VecDeque};
use std::io::{self, BufRead};
use std::thread;

const DIRS: [(i64, i64); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
struct Pos {
    y: i64,
    x: i64,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct State {
    cost: i64,
    lizards: BTreeMap<char, BTreeSet<Pos>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct PosWDistance {
    dist: i64,
    pos: Pos,
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

fn get(pos: &Pos, map: &[Vec<char>]) -> char {
    if !(0..map.len()).contains(&(pos.y as usize)) {
        return ' ';
    }
    if !(0..map[0].len()).contains(&(pos.x as usize)) {
        return ' ';
    }
    map[pos.y as usize][pos.x as usize]
}

fn get_mut<'a>(pos: &Pos, map: &'a mut [Vec<char>]) -> &'a mut char {
    &mut map[pos.y as usize][pos.x as usize]
}

fn is_room(pos: &Pos, map: &[Vec<char>]) -> bool {
    let left = Pos {
        y: pos.y,
        x: pos.x - 1,
    };
    let right = Pos {
        y: pos.y,
        x: pos.x + 1,
    };
    get(pos, map) != '#' && get(&left, map) == '#' && get(&right, map) == '#'
}

fn is_entrance(pos: &Pos, map: &[Vec<char>]) -> bool {
    let down = Pos {
        y: pos.y + 1,
        x: pos.x,
    };
    let up = Pos {
        y: pos.y - 1,
        x: pos.x,
    };
    get(pos, map) == '.' && get(&down, map) != '#' && get(&up, map) == '#'
}

fn is_hallway(pos: &Pos, map: &[Vec<char>]) -> bool {
    let up = Pos {
        y: pos.y - 1,
        x: pos.x,
    };
    get(pos, map) == '.' && get(&up, map) == '#'
}

fn find_lizzards(map: &[Vec<char>]) -> BTreeMap<char, BTreeSet<Pos>> {
    let mut poses: BTreeMap<char, BTreeSet<Pos>> = BTreeMap::new();
    for (y, row) in map.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if !['#', '.', ' '].contains(col) {
                poses.entry(*col).or_default().insert(Pos {
                    y: y as i64,
                    x: x as i64,
                });
            }
        }
    }
    poses
}

fn find_reachable(
    start: &Pos,
    taken: &AHashSet<Pos>,
    map: &[Vec<char>],
) -> Result<Vec<PosWDistance>> {
    let mut visited: AHashSet<Pos> = taken.clone();
    ensure!(visited.contains(start));
    let mut found: Vec<PosWDistance> = Vec::with_capacity(map[0].len());
    let mut queue: VecDeque<PosWDistance> = VecDeque::with_capacity(map[0].len());
    queue.push_back(PosWDistance {
        dist: 0,
        pos: *start,
    });

    while let Some(cur) = queue.pop_front() {
        for (dy, dx) in DIRS {
            let next = PosWDistance {
                dist: cur.dist + 1,
                pos: Pos {
                    y: cur.pos.y + dy,
                    x: cur.pos.x + dx,
                },
            };
            let ch = get(&next.pos, map);

            if ch != '.' {
                continue;
            }
            if visited.contains(&next.pos) {
                continue;
            }
            visited.insert(next.pos);

            found.push(next.clone());
            queue.push_back(next);
        }
    }

    Ok(found)
}

fn find_final_poses(map: &[Vec<char>]) -> BTreeMap<char, BTreeSet<Pos>> {
    let poses_by_x: Vec<(i64, Pos)> = (0..map.len())
        .cartesian_product(0..map[0].len())
        .map(|(y, x)| Pos {
            y: y as i64,
            x: x as i64,
        })
        .filter(|pos| is_room(pos, map))
        .map(|pos| (pos.x, pos))
        .sorted_unstable()
        .collect();

    let mut result: BTreeMap<char, BTreeSet<Pos>> = BTreeMap::new();
    let mut cur_ch = b'A' - 1;
    let mut prev_x: i64 = -1;

    for pos in poses_by_x.into_iter().map(|x| x.1) {
        if prev_x != pos.x {
            prev_x = pos.x;
            cur_ch += 1;
        }
        result.entry(cur_ch as char).or_default().insert(pos);
    }

    result
}

fn run() -> Result<()> {
    let mut map: Vec<Vec<char>> = Vec::new();
    for maybe_line in io::stdin().lock().lines() {
        let mut line: Vec<char> = maybe_line?.chars().collect();
        if line.is_empty() {
            continue;
        }

        while !map.is_empty() && line.len() < map[0].len() {
            line.push(' ');
        }
        map.push(line);
    }

    let init_poses = find_lizzards(&map);
    for poses in init_poses.values() {
        for pos in poses {
            *get_mut(pos, &mut map) = '.';
        }
    }

    let final_poses = find_final_poses(&map);

    let lizard_costs: AHashMap<char, i64> = [('A', 1), ('B', 10), ('C', 100), ('D', 1000)]
        .into_iter()
        .collect();

    let mut queue = BinaryHeap::new();
    queue.push(State {
        cost: 0,
        lizards: init_poses,
    });
    let mut visited: BTreeSet<BTreeMap<char, BTreeSet<Pos>>> = BTreeSet::new();

    while let Some(cur_state) = queue.pop() {
        if visited.contains(&cur_state.lizards) {
            continue;
        }
        visited.insert(cur_state.lizards.clone());

        let next_room_pos_per_lizzard: AHashMap<char, Pos> = cur_state
            .lizards
            .iter()
            .flat_map(|(lizzard, poses)| {
                final_poses
                    .get(lizzard)?
                    .iter()
                    .filter(|p| !poses.contains(p))
                    .max()
                    .map(|p| (*lizzard, *p))
            })
            .collect();
        if next_room_pos_per_lizzard.is_empty() {
            println!("{}", cur_state.cost);
            break;
        }

        let taken: AHashSet<Pos> = cur_state
            .lizards
            .values()
            .flat_map(|posses| posses.clone().into_iter())
            .collect();

        for (lizzard, poses) in cur_state.lizards.iter() {
            if let Some(next_room_pos) = next_room_pos_per_lizzard.get(lizzard) {
                for pos in poses {
                    let reachable: Vec<PosWDistance> = find_reachable(pos, &taken, &map)?
                        .into_iter()
                        .filter(|pwd| !is_entrance(&pwd.pos, &map))
                        .filter(|pwd| is_hallway(pos, &map) ^ is_hallway(&pwd.pos, &map))
                        .filter(|pwd| is_hallway(&pwd.pos, &map) || pwd.pos == *next_room_pos)
                        .collect();

                    for pwd in reachable.into_iter() {
                        let mut next_lizzards = cur_state.lizards.clone();
                        next_lizzards.get_mut(lizzard).context("")?.remove(pos);
                        next_lizzards.get_mut(lizzard).context("")?.insert(pwd.pos);
                        queue.push(State {
                            cost: cur_state.cost
                                + pwd.dist * lizard_costs.get(lizzard).context("")?,
                            lizards: next_lizzards,
                        })
                    }
                }
            }
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
