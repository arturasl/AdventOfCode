use regex::Regex;
use std::error::Error;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let re_space = Regex::new(r"\s+")?;

    let mut games: Vec<f64> = vec![];
    for maybe_line in io::stdin().lock().lines() {
        let line: String = maybe_line?
            .trim()
            .split(':')
            .nth(1)
            .unwrap()
            .trim()
            .to_string();
        games.push(re_space.replace_all(&line, "").parse().unwrap());
    }
    assert_eq!(games.len(), 2);

    let t = games[0];
    let d = games[1];

    let x: f64 = (t * t - 4. * d).sqrt();
    println!("{:?}", ((t + x) / 2.).floor() - ((t - x) / 2.).ceil() + 1.);

    Ok(())
}
