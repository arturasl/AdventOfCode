use cached::proc_macro::cached;
use itertools::Itertools;
use std::collections::HashMap;
use std::io;
use std::iter::repeat;
use std::thread;

#[cached(
    type = "HashMap<(usize, usize), i64>",
    create = "{ HashMap::new() }",
    convert = r#"{ (solve_idx, expected_idx) }"#
)]
fn rec(solve_idx: usize, solve: &[char], expected_idx: usize, expected: &[usize]) -> i64 {
    if solve_idx >= solve.len() {
        if expected_idx == expected.len() {
            return 1;
        }
        return 0;
    }

    if solve[solve_idx] == '.' {
        return rec(solve_idx + 1, solve, expected_idx, expected);
    }

    let mut result: i64 = 0;

    if solve[solve_idx] == '?' {
        result += rec(solve_idx + 1, solve, expected_idx, expected);
    }

    if expected_idx < expected.len() {
        let mut can_take = true;
        for i in 0..expected[expected_idx] {
            can_take =
                can_take && solve_idx + i < solve.len() && "?#".contains(solve[solve_idx + i]);
        }
        can_take = can_take
            && (solve_idx + expected[expected_idx] == solve.len()
                || ".?".contains(solve[solve_idx + expected[expected_idx]]));
        if can_take {
            result += rec(
                solve_idx + expected[expected_idx] + 1,
                solve,
                expected_idx + 1,
                expected,
            );
        }
    }

    result
}

fn run() {
    let mut result: i64 = 0;
    for line in io::stdin().lines().map(|x| x.unwrap()) {
        let (solve_tmp, expected_tmp): (&str, &str) =
            line.trim().split_whitespace().collect_tuple().unwrap();
        let mut solve: Vec<char> = repeat(solve_tmp.chars().chain(['?'].into_iter()))
            .take(5)
            .flatten()
            .collect();
        solve.pop();
        let expected: Vec<usize> = repeat(expected_tmp.split(',').map(|x| x.parse().unwrap()))
            .take(5)
            .flatten()
            .collect();
        (*REC).lock().unwrap().clear();
        let local_result: i64 = rec(0, &solve, 0, &expected);
        result += local_result;
    }
    println!("{}", result);
}

fn main() {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)
        .unwrap()
        .join()
        .unwrap();
}
