use anyhow::{Context, Ok, Result};
use std::io::{self, BufRead};
use std::thread;

fn run() -> Result<()> {
    let mut lookup: Vec<char> = Vec::new();
    let mut image: Vec<Vec<char>> = Vec::new();
    for maybe_line in io::stdin().lock().lines() {
        let line: Vec<char> = maybe_line?.trim().chars().collect();
        if line.is_empty() {
            continue;
        }

        if lookup.is_empty() {
            lookup = line;
        } else {
            image.push(line);
        }
    }

    for it in 0..2 {
        let height = image.len() as i64;
        let width = image[0].len() as i64;
        let mut next_image: Vec<Vec<char>> = Vec::new();

        for y in -1..(height + 1) {
            next_image.push(Vec::new());
            for x in -1..(width + 1) {
                let mut num = 0;
                for dy in -1i64..2 {
                    for dx in -1i64..2 {
                        let (ny, nx) = (y + dy, x + dx);
                        let bit = if !(0 <= ny
                            && ny < (image.len() as i64)
                            && 0 <= nx
                            && nx < (image[ny as usize].len() as i64))
                        {
                            if it == 0 {
                                0
                            } else if it % 2 == 1 {
                                i64::from(lookup[0] == '#')
                            } else {
                                i64::from(lookup[(1 << 9) - 1] == '#')
                            }
                        } else {
                            i64::from(image[ny as usize][nx as usize] == '#')
                        };
                        num = (num << 1) | bit;
                    }
                }
                next_image
                    .last_mut()
                    .context("")?
                    .push(lookup[num as usize]);
            }
        }
        image = next_image;
    }

    for row in image.iter() {
        for c in row.iter() {
            print!("{}", c);
        }
        println!();
    }

    let result = image
        .iter()
        .map(|row| row.iter().filter(|c| **c == '#').count() as i64)
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
