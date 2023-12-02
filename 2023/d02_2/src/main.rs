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
        let hands: Vec<String> = line_captures[2]
            .trim()
            .split(";")
            .map(|x| x.trim().to_string())
            .collect();

        let mut overall: HashMap<String, i32> = HashMap::new();
        for hand in hands {
            for pick in hand.split(",").map(|x| x.trim()) {
                let pick_captures = re_pick.captures(pick).unwrap();
                let amount: i32 = pick_captures[1].parse()?;
                let color: String = pick_captures[2].to_string();
                overall.insert(
                    color.clone(),
                    *overall.get(&color).unwrap_or(&0).max(&amount),
                );
            }
        }

        let pow = *overall.get("red").unwrap_or(&0)
            * *overall.get("green").unwrap_or(&0)
            * *overall.get("blue").unwrap_or(&0);
        println!("pow: {}", pow);
        assert!(pow >= 0);
        sum += pow;
    }

    println!("sum: {}", sum);

    Ok(())
}
