use ahash::AHashMap;
use anyhow::{Ok, Result};
use itertools::Itertools;
use rayon::prelude::*;
use std::io::{self, BufRead};
use std::thread;

fn map(part: &String, mapping: &AHashMap<char, char>) -> String {
    part.chars()
        .map(|p| *mapping.get(&p).unwrap())
        .sorted_unstable()
        .collect()
}

fn run() -> Result<()> {
    let all_letters: Vec<char> = "abcdefg".chars().collect();
    let nums: AHashMap<String, i64> = vec![
        "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
    ]
    .into_iter()
    .enumerate()
    .map(|(i, s)| (s.to_owned(), i as i64))
    .collect();

    let mappings: Vec<AHashMap<char, char>> = all_letters
        .iter()
        .permutations(all_letters.len())
        .map(|perm| {
            all_letters
                .iter()
                .zip_eq(perm.into_iter())
                .map(|(a, b)| (*a, *b))
                .collect()
        })
        .collect();

    let result = io::stdin()
        .lock()
        .lines()
        .map(|maybe_line| maybe_line.unwrap())
        .collect::<Vec<_>>()
        .par_iter()
        .map(|line| {
            let parts: Vec<String> = line
                .trim()
                .replace(" | ", " ")
                .split(" ")
                .map(|s| s.to_owned())
                .collect();
            assert!(parts.len() == 10 + 4);

            for mapping in mappings.iter() {
                if parts
                    .iter()
                    .take(10)
                    .all(|part| nums.contains_key(&map(part, &mapping)))
                {
                    return parts
                        .iter()
                        .rev()
                        .take(4)
                        .rev()
                        .map(|part| nums.get(&map(part, &mapping)).unwrap())
                        .fold(0, |cur, digit| cur * 10 + digit);
                }
            }
            assert!(false);
            0
        })
        .sum::<i64>();

    println!("{}", result);

    Ok(())
}

fn main() -> Result<()> {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)?
        .join()
        .unwrap()
}
