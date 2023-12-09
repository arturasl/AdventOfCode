use regex::Regex;
use std::error::Error;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let re_space = Regex::new(r"\s+")?;

    let mut games: Vec<Vec<i64>> = vec![];
    for maybe_line in io::stdin().lock().lines() {
        let line: String = maybe_line?
            .trim()
            .split(':')
            .nth(1)
            .unwrap()
            .trim()
            .to_string();
        games.push(re_space.split(&line).map(|x| x.parse().unwrap()).collect());
    }
    assert_eq!(games.len(), 2);

    let mut result: i64 = 1;
    for i in 0..games[0].len() {
        let t = games[0][i];
        let d = games[1][i];

        let mut mi: i64 = i64::MAX;
        let mut ma: i64 = i64::MIN;
        for h in 0..t {
            if h * (t - h) > d {
                mi = mi.min(h);
                ma = ma.max(h);
            }
        }
        assert!(mi != i64::MAX);
        result *= ma - mi + 1;
    }

    println!("{:?}", result);

    Ok(())
}
