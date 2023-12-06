use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, BufRead};

fn find(start: i64, ranges: &HashMap<String, Vec<(i64, i64, i64)>>, map: &str) -> i64 {
    let rng: &Vec<(i64, i64, i64)> = ranges.get(map).unwrap();
    let mut result: Option<i64> = None;
    for (dest_start, source_start, len) in rng {
        if *source_start <= start && start < *source_start + *len {
            assert!(result.is_none());
            result = Some(dest_start + start - source_start);
        }
    }
    result.unwrap_or(start)
}

fn main() -> Result<(), Box<dyn Error>> {
    let re_seeds = Regex::new(r"^seeds:(?<seeds>(?: \d+)+)")?;
    let re_nums = Regex::new(r"\d+")?;

    let mut seeds: Vec<i64> = vec![];
    let mut reading_map: String = "".to_string();
    let mut ranges: HashMap<String, Vec<(i64, i64, i64)>> = HashMap::new();

    for maybe_line in io::stdin().lock().lines() {
        let line: String = maybe_line?.trim().to_string();

        if line.is_empty() {
            continue;
        }

        if seeds.is_empty() {
            let line_capture = re_seeds.captures(&line).unwrap();
            seeds = line_capture["seeds"]
                .trim()
                .split(" ")
                .map(|x| x.parse().unwrap())
                .collect();
            continue;
        }

        let nums: Vec<i64> = re_nums
            .find_iter(&line)
            .map(|m| m.as_str().parse().unwrap())
            .collect();
        if nums.is_empty() {
            reading_map = line.split(" ").nth(0).unwrap().to_string();
            continue;
        }
        assert!(nums.len() == 3);

        ranges
            .entry(reading_map.clone())
            .or_insert(vec![])
            .push((nums[0], nums[1], nums[2]));
    }

    let mut final_poses: Vec<i64> = vec![];
    for seed in seeds {
        let mut moves: i64 = seed;
        for map in [
            "seed-to-soil",
            "soil-to-fertilizer",
            "fertilizer-to-water",
            "water-to-light",
            "light-to-temperature",
            "temperature-to-humidity",
            "humidity-to-location",
        ] {
            moves = find(moves, &ranges, map);
        }
        final_poses.push(moves);
    }

    println!("{}", final_poses.iter().min().unwrap());

    Ok(())
}
