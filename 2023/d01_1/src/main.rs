use std::io::{self, BufRead};

fn main() {
    let nums = "0123456789";
    let mut sum: i32 = 0;
    for maybe_line in io::stdin().lock().lines() {
        let chars: String = maybe_line.unwrap().trim().chars().collect();
        let lhs: char = chars.chars().find(|&x| nums.contains(x)).unwrap();
        let rhs: char = chars.chars().rev().find(|&x| nums.contains(x)).unwrap();
        let num: i32 = format!("{}{}", lhs, rhs).parse().unwrap();
        sum += num;
    }
    println!("{}", sum);
}
