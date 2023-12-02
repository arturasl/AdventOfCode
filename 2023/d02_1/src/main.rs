use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let mut sum: i32 = 0;
    let re_line = Regex::new(r"^Game (\d+):(.*)$")?;
    let re_pick = Regex::new(r"^(\d+) (red|green|blue)$")?;
    for maybe_line in io::stdin().lock().lines() {
        let line: String = maybe_line?.trim().to_string();
        let line_captures = re_line.captures(&line).unwrap();
        let id: i32 = line_captures[1].parse()?;
        let hands: Vec<String> = line_captures[2]
            .trim()
            .split(";")
            .map(|x| x.trim().to_string())
            .collect();

        let mut ok: bool = true;
        for hand in hands {
            let mut overall: HashMap<String, i32> = HashMap::new();

            for pick in hand.split(",").map(|x| x.trim()) {
                let pick_captures = re_pick.captures(pick).unwrap();
                let amount: i32 = pick_captures[1].parse()?;
                let color: String = pick_captures[2].to_string();
                overall.insert(color.clone(), overall.get(&color).unwrap_or(&0) + amount);
            }

            ok = ok
                && *overall.get("red").unwrap_or(&0) <= 12
                && *overall.get("green").unwrap_or(&0) <= 13
                && *overall.get("blue").unwrap_or(&0) <= 14;
        }

        sum += if ok { id } else { 0 };
    }

    println!("sum: {}", sum);

    Ok(())
}
