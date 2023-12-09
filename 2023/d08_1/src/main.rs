use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let re_nodes = Regex::new(r"^(?<start>\w{3})\s*=\s*\((?<left>\w{3})\s*,\s*(?<right>\w{3})\)$")?;

    let mut line_iter = io::stdin().lock().lines();

    let instructions: String = line_iter.next().unwrap()?.trim().to_string();
    line_iter.next();

    let mut map: HashMap<String, (String, String)> = HashMap::new();
    for maybe_line in line_iter {
        let line: String = maybe_line?.trim().to_string();
        let captures = re_nodes.captures(&line).unwrap();
        map.insert(
            captures["start"].to_string(),
            (captures["left"].to_string(), captures["right"].to_string()),
        );
    }

    let mut pos: String = "AAA".to_string();
    let mut steps: usize = 0;
    while pos != "ZZZ" {
        for c in instructions.chars() {
            println!("At: {}", pos);
            pos = match c {
                'L' => map.get(&pos).unwrap().0.clone(),
                'R' => map.get(&pos).unwrap().1.clone(),
                _ => panic!(),
            };
            steps += 1;
            if pos == "ZZZ" {
                break;
            }
        }
    }

    println!("{:?}", steps);

    Ok(())
}
