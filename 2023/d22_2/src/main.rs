use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::io;
use std::thread;

#[derive(Debug)]
struct Coord {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug)]
struct Cube {
    lo: Coord,
    hi: Coord,
    idx: usize,
    above_idxs: HashSet<usize>,
    bellow_idxs: HashSet<usize>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct ZSortedIdx {
    idx: usize,
    z: i64,
}

impl Ord for ZSortedIdx {
    fn cmp(&self, other: &Self) -> Ordering {
        other.z.cmp(&self.z).then_with(|| self.idx.cmp(&other.idx))
    }
}

impl PartialOrd for ZSortedIdx {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn run() {
    let mut cubes: BTreeMap<usize, Cube> = io::stdin()
        .lines()
        .map(|x| x.unwrap().trim().to_string())
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split('~')
                .map(|part| {
                    let (x, y, z): (i64, i64, i64) = part
                        .split(',')
                        .map(|x| x.parse::<i64>().unwrap())
                        .collect_tuple()
                        .unwrap();
                    Coord { x, y, z }
                })
                .collect_tuple()
                .unwrap()
        })
        .map(|(lhs, rhs)| {
            (
                Coord {
                    x: lhs.x.min(rhs.x),
                    y: lhs.y.min(rhs.y),
                    z: lhs.z.min(rhs.z),
                },
                Coord {
                    x: lhs.x.max(rhs.x),
                    y: lhs.y.max(rhs.y),
                    z: lhs.z.max(rhs.z),
                },
            )
        })
        .sorted_by_key(|(lhs, _)| lhs.z)
        .enumerate()
        .map(|(idx, (lo, hi))| {
            (
                idx,
                Cube {
                    lo,
                    hi,
                    idx,
                    above_idxs: HashSet::new(),
                    bellow_idxs: HashSet::new(),
                },
            )
        })
        .collect();

    let (mut mi_x, mut ma_x) = (i64::MAX, i64::MIN);
    let (mut mi_y, mut ma_y) = (i64::MAX, i64::MIN);
    for cube in cubes.values() {
        mi_x = mi_x.min(cube.lo.x).min(cube.hi.x);
        ma_x = ma_x.max(cube.lo.x).max(cube.hi.x);
        mi_y = mi_y.min(cube.lo.y).min(cube.hi.y);
        ma_y = ma_y.max(cube.lo.y).max(cube.hi.y);
    }

    for cube in cubes.values_mut() {
        cube.lo.x -= mi_x;
        cube.lo.y -= mi_y;
        cube.hi.x -= mi_x;
        cube.hi.y -= mi_y;
    }

    let mut ground: Vec<Vec<i64>> =
        vec![vec![1; (ma_x - mi_x + 1) as usize]; (ma_y - mi_y + 1) as usize];

    for cube in cubes.values_mut() {
        let height = cube.hi.z - cube.lo.z + 1;
        let mut new_z = 0;
        for y in cube.lo.y..cube.hi.y + 1 {
            for x in cube.lo.x..cube.hi.x + 1 {
                new_z = new_z.max(ground[y as usize][x as usize]);
            }
        }
        for y in cube.lo.y..cube.hi.y + 1 {
            for x in cube.lo.x..cube.hi.x + 1 {
                ground[y as usize][x as usize] = new_z + height;
            }
        }

        cube.lo.z = new_z;
        cube.hi.z = new_z + height - 1;
    }

    let mut filled: HashMap<i64, HashMap<i64, HashMap<i64, usize>>> = HashMap::new();
    for cube in cubes.values() {
        for y in cube.lo.y..cube.hi.y + 1 {
            for x in cube.lo.x..cube.hi.x + 1 {
                for z in cube.lo.z..cube.hi.z + 1 {
                    assert!(filled
                        .entry(y)
                        .or_default()
                        .entry(x)
                        .or_default()
                        .insert(z, cube.idx)
                        .is_none());
                }
            }
        }
    }

    for cube in cubes.values_mut() {
        cube.above_idxs = (cube.lo.y..cube.hi.y + 1)
            .cartesian_product(cube.lo.x..cube.hi.x + 1)
            .filter_map(|(y, x)| {
                filled
                    .entry(y)
                    .or_default()
                    .entry(x)
                    .or_default()
                    .get(&(cube.hi.z + 1))
                    .cloned()
            })
            .collect();

        cube.bellow_idxs = (cube.lo.y..cube.hi.y + 1)
            .cartesian_product(cube.lo.x..cube.hi.x + 1)
            .filter_map(|(y, x)| {
                filled
                    .entry(y)
                    .or_default()
                    .entry(x)
                    .or_default()
                    .get(&(cube.lo.z - 1))
                    .cloned()
            })
            .collect();
    }

    let mut result: usize = 0;
    for cube in cubes.values() {
        let mut erased_idxs: HashSet<usize> = HashSet::new();
        let mut visited: HashSet<usize> = HashSet::new();

        let mut heap: BinaryHeap<ZSortedIdx> = BinaryHeap::from([ZSortedIdx {
            idx: cube.idx,
            z: cube.lo.z,
        }]);

        let mut erased: usize = 0;
        while let Some(ZSortedIdx { idx, z: _ }) = heap.pop() {
            if visited.contains(&idx) {
                continue;
            }
            visited.insert(idx);

            let cur = cubes.get(&idx).unwrap();
            if cur.bellow_idxs.difference(&erased_idxs).next().is_some() && idx != cube.idx {
                continue;
            }

            erased += 1;
            erased_idxs.insert(cur.idx);

            for above_idx in &cur.above_idxs {
                heap.push(ZSortedIdx {
                    idx: *above_idx,
                    z: cubes.get(above_idx).unwrap().lo.z,
                });
            }
        }

        result += erased - 1;
    }

    println!("{result}");
}

fn main() {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)
        .unwrap()
        .join()
        .unwrap();
}
