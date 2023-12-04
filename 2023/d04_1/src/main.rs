use regex::Regex;
use std::collections::HashSet;
use std::error::Error;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let re_line = Regex::new(r"^Card\s+\d+:(?<lhs>(?: +\d+)*)\s*\|(?<rhs>(?: +\d+)*)$")?;
    let re_nums = Regex::new(r"\d+")?;
    let mut result: i64 = 0;
    for maybe_line in io::stdin().lock().lines() {
        let line: String = maybe_line?.trim().to_string();
        let line_capture = re_line.captures(&line).unwrap();

        let lhs_nums: HashSet<String> = re_nums
            .find_iter(&line_capture["lhs"])
            .map(|m| m.as_str().to_string())
            .collect();
        let rhs_nums: HashSet<String> = re_nums
            .find_iter(&line_capture["rhs"].to_string())
            .map(|m| m.as_str().to_string())
            .collect();
        let cnt = rhs_nums.iter().filter(|x| lhs_nums.contains(*x)).count();
        if cnt > 0 {
            result += 1i64 << (cnt - 1);
        }
    }
    println!("{:?}", result);

    Ok(())
}
