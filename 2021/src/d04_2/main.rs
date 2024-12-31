use anyhow::{ensure, Context, Ok, Result};
use regex::Regex;
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::thread;

fn read() -> Result<(HashMap<i64, i64>, Vec<Vec<Vec<i64>>>)> {
    let re_num = Regex::new(r"\d+").unwrap();
    let mut draw_to_first_pos: HashMap<i64, i64> = HashMap::new();
    let mut tables: Vec<Vec<Vec<i64>>> = vec![];

    for maybe_line in io::stdin().lock().lines() {
        let nums: Vec<i64> = re_num
            .find_iter(&maybe_line?)
            .map(|m| Ok(m.as_str().parse::<i64>()?))
            .collect::<Result<_>>()?;

        if draw_to_first_pos.is_empty() && !nums.is_empty() {
            for (idx, num) in nums.into_iter().enumerate() {
                ensure!(draw_to_first_pos.insert(num, idx as i64) == None);
            }
        } else if nums.is_empty() {
            if tables.last().iter().all(|x| !x.is_empty()) {
                tables.push(vec![])
            }
        } else {
            tables.last_mut().context("No previous")?.push(nums);
        }
    }
    while tables.last().iter().any(|x| x.is_empty()) {
        tables.pop();
    }

    Ok((draw_to_first_pos, tables))
}

fn run() -> Result<()> {
    let (draw_to_first_pos, tables) = read()?;

    let mut best: Option<(i64, i64)> = None;
    for table in tables.into_iter() {
        let draw_times_y_to_row: Vec<Vec<Option<i64>>> = table
            .iter()
            .map(|row| {
                row.iter()
                    .map(|v| draw_to_first_pos.get(v).cloned())
                    .collect()
            })
            .collect();
        let draw_times_x_to_col: Vec<Vec<Option<i64>>> = (0..table.len())
            .map(|x| {
                (0..table.len())
                    .map(|y| draw_times_y_to_row[y][x])
                    .collect()
            })
            .collect();

        let maybe_local_draw_time = draw_times_y_to_row
            .iter()
            .chain(draw_times_x_to_col.iter())
            .filter(|maybe_draw_times| maybe_draw_times.iter().all(Option::is_some))
            .map(|maybe_draw_times| {
                maybe_draw_times
                    .into_iter()
                    .map(|x| x.unwrap())
                    .max()
                    .unwrap()
            })
            .min();

        if let Some(local_draw_time) = maybe_local_draw_time {
            let mut unvisited_sum = 0;
            let mut val = -1;
            for y in 0..table.len() {
                for x in 0..table[y].len() {
                    if draw_times_y_to_row[y][x].is_none()
                        || draw_times_y_to_row[y][x].unwrap() > local_draw_time
                    {
                        unvisited_sum += table[y][x];
                    }
                    if Some(local_draw_time) == draw_times_y_to_row[y][x] {
                        ensure!(val == -1);
                        val = table[y][x];
                    }
                }
            }
            let local_result = (local_draw_time, unvisited_sum * val);
            best = Some(best.unwrap_or(local_result).max(local_result));
        }
    }

    println!("{}", best.context("")?.1);

    Ok(())
}

fn main() -> Result<()> {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)?
        .join()
        .unwrap()
}
