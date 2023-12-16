use std::collections::HashMap;
use std::io;
use std::thread;

fn hash(line: &str) -> usize {
    line.chars()
        .fold(0usize, |acc, c| ((acc + (c as usize)) * 17) % 256)
}

fn run() {
    let rules: Vec<String> = io::stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split(',')
        .map(|x| x.to_string())
        .collect();

    let mut hash_map: Vec<HashMap<String, (usize, i64)>> = vec![HashMap::new(); 256];

    for (i, rule) in rules.into_iter().enumerate() {
        if rule.contains('=') {
            let parts: Vec<&str> = rule.split('=').collect();
            let label: &str = parts[0];
            let lens: i64 = parts[1].parse().unwrap();
            hash_map[hash(label)]
                .entry(label.to_string())
                .and_modify(|x| x.1 = lens)
                .or_insert((i, lens));
        } else {
            assert!(rule.chars().rev().next().unwrap() == '-');
            let label = &rule[..rule.len() - 1];
            hash_map[hash(label)].remove(label);
        }
    }

    let mut result: i64 = 0;
    for bx in 0..hash_map.len() {
        if hash_map[bx].is_empty() {
            continue;
        }
        let mut vec = hash_map[bx].iter().collect::<Vec<_>>();
        vec.sort_by_key(|x| x.1 .0);
        let local_result = vec
            .iter()
            .enumerate()
            .map(|(j, x)| (x.1 .1) * ((bx as i64) + 1) * ((j as i64) + 1))
            .sum::<i64>();
        println!("{bx}: {:?} = {local_result}", vec);
        result += local_result;
    }

    println!("Result: {result}");
}

fn main() {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)
        .unwrap()
        .join()
        .unwrap();
}
