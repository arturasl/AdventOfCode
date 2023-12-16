use std::collections::HashMap;
use std::io;
use std::thread;

fn drop_up(map: &mut Vec<Vec<char>>) {
    for col_idx in 0..map[0].len() {
        let mut swap_to: usize = 0;
        for row_idx in 0..map.len() {
            match map[row_idx][col_idx] {
                '#' => swap_to = row_idx + 1,
                '.' => {}
                'O' => {
                    map[row_idx][col_idx] = '.';
                    map[swap_to][col_idx] = 'O';
                    swap_to += 1;
                }
                _ => panic!(),
            }
        }
    }
}

fn rot(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    (0..map[0].len())
        .map(|col| (0..map.len()).rev().map(|row| map[row][col]).collect())
        .collect()
}

fn calc_result(map: &Vec<Vec<char>>) -> usize {
    map.iter()
        .enumerate()
        .map(|(i, row)| row.into_iter().filter(|x| **x == 'O').count() * (map.len() - i))
        .sum()
}

fn run() {
    let map: Vec<Vec<char>> = io::stdin()
        .lines()
        .map(|x| x.unwrap().trim().chars().collect::<Vec<char>>())
        .filter(|x| !x.is_empty())
        .collect();

    let mut path_map: HashMap<Vec<Vec<char>>, usize> = HashMap::new();
    let mut paths: Vec<Vec<Vec<char>>> = Vec::new();
    let mut after_cycle = map;
    let mut cycle_oft: usize = 0;

    loop {
        if path_map.contains_key(&after_cycle) {
            cycle_oft = *path_map.get(&after_cycle).unwrap();
            break;
        }
        path_map.insert(after_cycle.clone(), path_map.len());
        paths.push(after_cycle.clone());
        for _ in 0..4 {
            drop_up(&mut after_cycle);
            after_cycle = rot(after_cycle);
        }
    }

    let walk: usize = 1_000_000_000;
    let cycle_len = paths.len() - cycle_oft;
    let cycles = (walk - cycle_oft) / cycle_len;
    let walk_cycle_oft = (walk - cycle_oft) - cycles * cycle_len;
    let pos = cycle_oft + walk_cycle_oft;
    println!("{}", calc_result(&paths[pos]));
}

fn main() {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)
        .unwrap()
        .join()
        .unwrap();
}
