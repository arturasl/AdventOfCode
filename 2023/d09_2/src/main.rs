use std::error::Error;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let mut result: i64 = 0;
    for maybe_line in io::stdin().lock().lines() {
        let mut nums: Vec<i64> = maybe_line?
            .trim()
            .split(" ")
            .map(|x| x.parse().unwrap())
            .collect();

        let mut path: Vec<i64> = vec![nums[0]];
        while !nums.iter().all(|x| *x == 0) {
            for i in 0..nums.len() - 1 {
                nums[i] = nums[i + 1] - nums[i];
            }
            path.push(nums[0]);
            nums.pop().unwrap();
        }
        result += path.iter().rev().fold(0i64, |acc, x| x - acc);
    }
    println!("result: {}", result);

    Ok(())
}
