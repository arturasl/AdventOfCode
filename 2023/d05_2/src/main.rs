use itertools::sorted;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{self, BufRead};

type Range = (i64, i64);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Rule {
    from: Range,
    to: Range,
}

const MAP_NAMES: &'static [&str] = &[
    "seed-to-soil",
    "soil-to-fertilizer",
    "fertilizer-to-water",
    "water-to-light",
    "light-to-temperature",
    "temperature-to-humidity",
    "humidity-to-location",
];

fn split(idx: &mut usize, all_points: &Vec<i64>, cur: &Range) -> Vec<Range> {
    let mut after: Vec<Range> = vec![];

    while all_points[*idx] <= cur.0 {
        *idx += 1;
    }
    while (*idx as usize) < all_points.len() && all_points[*idx] <= cur.1 {
        after.push((all_points[*idx - 1], all_points[*idx]));
        *idx += 1;
    }

    after
}

fn apply(seed_ranges: &Vec<Range>, rules: &Vec<Rule>) -> Vec<Range> {
    let all_points: Vec<i64> = {
        let mut all_points: HashSet<i64> = HashSet::new();
        for seed in seed_ranges {
            all_points.insert(seed.0);
            all_points.insert(seed.1);
        }
        for rule in rules {
            all_points.insert(rule.from.0);
            all_points.insert(rule.from.1);
        }
        sorted(all_points).collect()
    };

    let mapping_rules: HashMap<Range, Range> = {
        let mut idx: usize = 0;
        let mut mapping_rules: HashMap<Range, Range> = HashMap::new();
        for rule in rules {
            for split_from in split(&mut idx, &all_points, &rule.from) {
                mapping_rules.insert(
                    split_from,
                    (
                        rule.to.0 + split_from.0 - rule.from.0,
                        rule.to.0 + split_from.1 - rule.from.0,
                    ),
                );
            }
        }
        mapping_rules
    };

    let mut result: Vec<Range> = vec![];

    let mut idx: usize = 0;
    for seed_range in seed_ranges {
        for split_seed in split(&mut idx, &all_points, seed_range) {
            if let Some(to) = mapping_rules.get(&split_seed) {
                result.push(*to);
            } else {
                result.push(split_seed);
            }
        }
    }

    result.sort();
    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let re_seeds = Regex::new(r"^seeds:(?<seeds>(?: \d+)+)")?;
    let re_nums = Regex::new(r"\d+")?;

    let mut seeds: Vec<Range> = vec![];
    let mut reading_map: String = "".to_string();
    let mut grouped_rules: HashMap<String, Vec<Rule>> = HashMap::new();

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
                .map(|x| x.parse::<i64>().unwrap())
                .chunks(2)
                .into_iter()
                .map(|x| {
                    let vals: Vec<i64> = x.collect();
                    (vals[0], vals[0] + vals[1])
                })
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

        grouped_rules
            .entry(reading_map.clone())
            .or_insert(vec![])
            .push(Rule {
                from: (nums[1], nums[1] + nums[2]),
                to: (nums[0], nums[0] + nums[2]),
            });
    }

    for rules in grouped_rules.values_mut() {
        rules.sort();
    }

    let mut m: i64 = i64::MAX;
    for seed in seeds {
        let mut result: Vec<Range> = vec![seed];
        for map in MAP_NAMES {
            result = apply(&result, grouped_rules.get(*map).unwrap());
        }
        m = m.min(result[0].0);
    }

    println!("Min location: {:?}", m);

    Ok(())
}
