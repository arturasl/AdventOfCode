use std::collections::HashSet;
use std::io;
use std::iter::zip;
use std::thread;

fn run() {
    let mut final_result: usize = 0;
    let mut map: Vec<Vec<char>> = vec![];
    for line in io::stdin()
        .lines()
        .map(|x| x.unwrap().trim().chars().collect::<Vec<char>>())
        .chain(vec![vec![]].into_iter())
    {
        if !line.is_empty() {
            map.push(line);
            continue;
        }
        if map.is_empty() {
            continue;
        }

        for rep in 0..2 {
            let mut result: HashSet<usize> = (1..map[0].len()).collect();
            for row in 0..map.len() {
                let mut local_result: HashSet<usize> = HashSet::new();
                for col in 0..map[row].len() + 1 {
                    let left = (&map[row][..col]).into_iter().rev();
                    let right = (&map[row][col..]).into_iter();
                    if zip(left, right).all(|(x, y)| x == y) {
                        local_result.insert(col);
                    }
                }
                result = result.intersection(&local_result).cloned().collect();
            }

            assert!(result.len() <= 1);
            if !result.is_empty() {
                final_result += result.into_iter().nth(0).unwrap() * if rep == 0 { 1 } else { 100 }
            }

            map = (0..map[0].len())
                .map(|col| (0..map.len()).map(|row| map[row][col]).collect())
                .collect();
        }

        map = vec![];
    }

    println!("{}", final_result);
}

fn main() {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)
        .unwrap()
        .join()
        .unwrap();
}
