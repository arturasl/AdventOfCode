use ahash::{AHashMap, AHashSet};
use anyhow::{Context, Ok, Result};
use itertools::Itertools;
use std::io::{self, BufRead};
use std::thread;

fn traverse(
    cur: &str,
    graph: &AHashMap<String, AHashSet<String>>,
    visited: &mut AHashSet<String>,
) -> Result<i64> {
    if cur == "end" {
        return Ok(1);
    }

    let is_small = cur.chars().all(|ch| ch.is_lowercase());
    if is_small {
        if visited.contains(cur) {
            return Ok(0);
        }
        visited.insert(cur.to_string());
    }

    let mut result = 0;
    for child in graph.get(cur).context("")? {
        result += traverse(child, graph, visited)?;
    }

    if is_small {
        visited.remove(cur);
    }

    Ok(result)
}

fn run() -> Result<()> {
    let mut graph: AHashMap<String, AHashSet<String>> = AHashMap::new();
    for (from, to) in io::stdin()
        .lock()
        .lines()
        .map(|maybe_line| {
            Ok(maybe_line?
                .trim()
                .split('-')
                .map(|part| part.to_owned())
                .collect::<Vec<String>>())
        })
        .filter_ok(|parts| !parts.is_empty())
        .map(|parts| {
            parts.and_then(|parts| {
                Ok(parts
                    .into_iter()
                    .collect_tuple::<(String, String)>()
                    .context("")?)
            })
        })
        .map_ok(|parts| {
            vec![
                (parts.0.clone(), parts.1.clone()),
                (parts.1.clone(), parts.0.clone()),
            ]
        })
        .flatten_ok()
        .collect::<Result<Vec<(String, String)>>>()?
        .into_iter()
    {
        graph.entry(from).or_default().insert(to);
    }

    let mut visited: AHashSet<String> = AHashSet::new();
    println!("{:?}", traverse("start", &graph, &mut visited)?);

    Ok(())
}

fn main() -> Result<()> {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)?
        .join()
        .unwrap()
}
