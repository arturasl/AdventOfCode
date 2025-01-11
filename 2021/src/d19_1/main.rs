use ahash::{AHashSet, HashMapExt};
use anyhow::{bail, ensure, Context, Ok, Result};
use itertools::Itertools;
use memoize::memoize;
use regex::Regex;
use std::collections::BTreeSet;
use std::io::{self, BufRead};
use std::thread;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Pos {
    y: i64,
    x: i64,
    z: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Scanner {
    idx: usize,
    pos: Pos,
    beacons: BTreeSet<Pos>,
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Clone)]
struct Rot {
    matrix: [[i64; 3]; 3],
}

fn read_scanners() -> Result<Vec<Scanner>> {
    let mut scanners: Vec<Scanner> = Vec::new();

    let re_scanner_header = Regex::new(r"^-+ scanner (?<id>\d+) -+$")?;
    let re_pos = Regex::new(r"^(?<x>-?\d+),\s*(?<y>-?\d+),\s*(?<z>-?\d+)$")?;
    for maybe_line in io::stdin().lock().lines() {
        let line = maybe_line?.trim().to_owned();
        if line.is_empty() {
            continue;
        }

        if let Some(captures_scanner_header) = re_scanner_header.captures(&line) {
            let len = scanners.len();
            ensure!(len == captures_scanner_header["id"].parse()?);
            scanners.push(Scanner {
                idx: len,
                pos: Pos { x: 0, y: 0, z: 0 },
                beacons: BTreeSet::new(),
            });
        } else if let Some(captures_pos) = re_pos.captures(&line) {
            scanners.last_mut().context("")?.beacons.insert(Pos {
                x: captures_pos["x"].parse()?,
                y: captures_pos["y"].parse()?,
                z: captures_pos["z"].parse()?,
            });
        } else {
            bail!("");
        }
    }

    Ok(scanners)
}

fn mul_p(pos: &Pos, matrix: &[[i64; 3]; 3]) -> Pos {
    Pos {
        x: pos.x * matrix[0][0] + pos.y * matrix[0][1] + pos.z * matrix[0][2],
        y: pos.x * matrix[1][0] + pos.y * matrix[1][1] + pos.z * matrix[1][2],
        z: pos.x * matrix[2][0] + pos.y * matrix[2][1] + pos.z * matrix[2][2],
    }
}

fn mul_m(lhs: &[[i64; 3]; 3], rhs: &[[i64; 3]; 3]) -> [[i64; 3]; 3] {
    let mut r: [[i64; 3]; 3] = [[0; 3]; 3];

    for y in 0..3 {
        for x in 0..3 {
            for (i, rr) in rhs.iter().enumerate() {
                r[y][x] += lhs[y][i] * rr[x];
            }
        }
    }

    r
}

fn create_rots() -> Result<Vec<Rot>> {
    let mut rots: AHashSet<Rot> = AHashSet::new();

    let rx: [[i64; 3]; 3] = [[1, 0, 0], [0, 0, -1], [0, 1, 0]];
    let ry: [[i64; 3]; 3] = [[0, 0, 1], [0, 1, 0], [-1, 0, 0]];
    let rz: [[i64; 3]; 3] = [[0, -1, 0], [1, 0, 0], [0, 0, 1]];

    let mut nx = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];
    for _ in 0..4 {
        let mut ny = nx;
        for _ in 0..4 {
            let mut nz = ny;
            for _ in 0..4 {
                rots.insert(Rot { matrix: nz });
                nz = mul_m(&nz, &rz);
            }
            ny = mul_m(&ny, &ry);
        }
        nx = mul_m(&nx, &rx);
    }

    ensure!(rots.len() == 24);
    Ok(rots.into_iter().collect())
}

fn apply_rot(scanner: &Scanner, rot: &Rot) -> Scanner {
    let mut result = Scanner {
        idx: scanner.idx,
        pos: mul_p(&scanner.pos, &rot.matrix),
        beacons: BTreeSet::new(),
    };

    for pos in scanner.beacons.iter() {
        result.beacons.insert(mul_p(pos, &rot.matrix));
    }

    result
}

fn find_compatible(lhs: &Scanner, rhs: &Scanner, rots: &[Rot]) -> BTreeSet<Scanner> {
    let mut result: BTreeSet<Scanner> = BTreeSet::new();

    for lhs_pos in lhs.beacons.iter() {
        for rot in rots {
            let rhs_rotated = apply_rot(rhs, rot);
            for rhs_pos in rhs_rotated.beacons.iter() {
                let scanner_pos = Pos {
                    x: lhs_pos.x - rhs_pos.x,
                    y: lhs_pos.y - rhs_pos.y,
                    z: lhs_pos.z - rhs_pos.z,
                };

                let rhs_new_beacons: BTreeSet<Pos> = rhs_rotated
                    .beacons
                    .iter()
                    .map(|p| Pos {
                        x: p.x + scanner_pos.x,
                        y: p.y + scanner_pos.y,
                        z: p.z + scanner_pos.z,
                    })
                    .collect();

                if lhs.beacons.intersection(&rhs_new_beacons).count() >= 12 {
                    result.insert(Scanner {
                        idx: rhs_rotated.idx,
                        pos: scanner_pos,
                        beacons: rhs_new_beacons,
                    });
                }
            }
        }
    }

    result
}

#[memoize(Ignore: scanners, Ignore: rots, CustomHasher: ahash::HashMap)]
fn find(finished_scanners: BTreeSet<Scanner>, scanners: &[Scanner], rots: &[Rot]) -> bool {
    let finished_idxes: BTreeSet<usize> = finished_scanners.iter().map(|s| s.idx).collect();
    println!("{:?}", finished_idxes);
    if finished_idxes.len() == scanners.len() {
        let unique_beacons = finished_scanners
            .iter()
            .flat_map(|s| s.beacons.clone())
            .collect::<BTreeSet<Pos>>()
            .len();

        let max_dist: i64 = finished_scanners
            .iter()
            .combinations(2)
            .map(|c| {
                (c[0].pos.x - c[1].pos.x).abs()
                    + (c[0].pos.y - c[1].pos.y).abs()
                    + (c[0].pos.z - c[1].pos.z).abs()
            })
            .max()
            .unwrap_or(0);
        println!("{}", max_dist);
        println!("{}", unique_beacons);
        return true;
    }

    for finished_scanner in finished_scanners.iter() {
        for rhs_scanner in scanners.iter() {
            if finished_idxes.contains(&rhs_scanner.idx) {
                continue;
            }

            for potential_scanner in find_compatible(finished_scanner, rhs_scanner, rots) {
                let mut new_finished_scanners = finished_scanners.clone();
                new_finished_scanners.insert(potential_scanner.clone());
                if find(new_finished_scanners, scanners, rots) {
                    return true;
                }
            }
        }
    }

    false
}

fn run() -> Result<()> {
    let scanners = read_scanners()?;
    let rots = create_rots()?;

    let mut finsihed_scanners: BTreeSet<Scanner> = BTreeSet::new();
    finsihed_scanners.insert(scanners[0].clone());
    find(finsihed_scanners, &scanners, &rots);

    Ok(())
}

fn main() -> Result<()> {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)?
        .join()
        .unwrap()
}
