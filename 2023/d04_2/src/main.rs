use regex::Regex;
use std::collections::HashSet;
use std::error::Error;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let re_line = Regex::new(r"^Card\s+(?<id>\d+):(?<lhs>(?: +\d+)*)\s*\|(?<rhs>(?: +\d+)*)$")?;
    let re_nums = Regex::new(r"\d+")?;

    let mut counts: Vec<i64> = vec![];

    let mut expected_idx: usize = 1;
    for maybe_line in io::stdin().lock().lines() {
        let line: String = maybe_line?.trim().to_string();
        let line_capture = re_line.captures(&line).unwrap();

        let lhs_nums: Vec<i64> = re_nums
            .find_iter(&line_capture["lhs"])
            .map(|m| m.as_str().parse().unwrap())
            .collect();
        let lhs: HashSet<i64> = lhs_nums.iter().map(|x| *x).collect();

        let rhs_nums: Vec<i64> = re_nums
            .find_iter(&line_capture["rhs"])
            .map(|m| m.as_str().parse().unwrap())
            .collect();
        let rhs: HashSet<i64> = rhs_nums.iter().map(|x| *x).collect();
        assert!(rhs.len() == rhs_nums.len());

        let idx: usize = line_capture["id"].parse().unwrap();
        assert!(expected_idx == idx);
        expected_idx += 1;
        let cnt: usize = rhs.iter().filter(|x| lhs.contains(*x)).count();

        for i in idx + 1..idx + cnt + 1 {
            while counts.len() <= i {
                counts.push(1);
            }

            counts[i] += counts[idx];
        }
    }

    while counts.len() <= expected_idx - 1 {
        counts.push(1);
    }

    println!(
        "{}",
        counts.iter().skip(1).take(expected_idx - 1).sum::<i64>()
    );

    Ok(())
}
