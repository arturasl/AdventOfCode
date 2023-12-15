use std::io;
use std::iter::zip;
use std::thread;

fn transpose(map: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    (0..map[0].len())
        .map(|col| (0..map.len()).map(|row| map[row][col]).collect())
        .collect()
}

fn find_hist(map: &Vec<Vec<bool>>, expected: i64) -> Vec<usize> {
    let mut result: Vec<i64> = vec![0; map[0].len() + 1];
    for col in 1..map[0].len() {
        result[col] = -((map.len() * col.min(map[0].len() - col)) as i64);
    }
    for row in 0..map.len() {
        for col in 1..map[row].len() {
            let left = (&map[row][..col]).into_iter().rev();
            let right = (&map[row][col..]).into_iter();
            result[col] += zip(left, right).filter(|(x, y)| x == y).count() as i64;
        }
    }

    result
        .into_iter()
        .enumerate()
        .filter(|(_, x)| *x == expected)
        .map(|(i, _)| i)
        .collect()
}

fn run() {
    let mut final_result: usize = 0;
    let mut map: Vec<Vec<bool>> = vec![];
    for line in io::stdin()
        .lines()
        .map(|x| {
            x.unwrap()
                .trim()
                .chars()
                .map(|x| x == '#')
                .collect::<Vec<bool>>()
        })
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
            let hist: Vec<usize> = find_hist(&map, -1);
            assert!(hist.len() <= 1);
            if !hist.is_empty() {
                final_result += hist.into_iter().next().unwrap() * if rep == 0 { 1 } else { 100 }
            }

            map = transpose(&map);
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
