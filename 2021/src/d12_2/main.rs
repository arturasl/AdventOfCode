use ahash::AHashMap;
use anyhow::{Context, Ok, Result};
use itertools::Itertools;
use std::io::{self, BufRead};
use std::thread;

#[derive(Debug)]
struct Node {
    is_small: bool,
    children: Vec<usize>,
}

fn traverse(
    cur_idx: usize,
    graph: &Vec<Node>,
    visited: &mut Vec<bool>,
    had_small_twice: bool,
    start_idx: usize,
    end_idx: usize,
) -> Result<i64> {
    if cur_idx == end_idx {
        return Ok(1);
    }

    let mut cur_is_small_twice = false;
    if graph[cur_idx].is_small {
        if visited[cur_idx] {
            if had_small_twice || cur_idx == start_idx {
                return Ok(0);
            } else {
                cur_is_small_twice = true;
            }
        } else {
            visited[cur_idx] = true;
        }
    }

    let mut result = 0;
    for child in graph[cur_idx].children.iter() {
        result += traverse(
            *child,
            graph,
            visited,
            had_small_twice || cur_is_small_twice,
            start_idx,
            end_idx,
        )?;
    }

    if graph[cur_idx].is_small && !cur_is_small_twice {
        visited[cur_idx] = false;
    }

    return Ok(result);
}

fn run() -> Result<()> {
    let mut name_to_idx: AHashMap<String, usize> = AHashMap::new();
    let mut graph: Vec<Node> = Vec::new();

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
        let before_from_len = name_to_idx.len();
        let from_idx = name_to_idx
            .entry(from.clone())
            .or_insert(before_from_len)
            .clone();
        let before_to_len = name_to_idx.len();
        let to_idx = name_to_idx.entry(to).or_insert(before_to_len).clone();

        while graph.len() <= from_idx {
            graph.push(Node {
                is_small: false,
                children: vec![],
            });
        }
        graph[from_idx].children.push(to_idx);
        graph[from_idx].is_small = from.chars().all(|ch| ch.is_lowercase());
    }

    for node in graph.iter_mut() {
        node.children.sort_unstable();
        node.children.dedup();
    }

    let start_idx = name_to_idx.get("start").context("")?.clone();
    let end_idx = name_to_idx.get("end").context("")?.clone();
    let mut visited = vec![false; graph.len()];

    println!(
        "{:?}",
        traverse(start_idx, &graph, &mut visited, false, start_idx, end_idx)?
    );

    Ok(())
}

fn main() -> Result<()> {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)?
        .join()
        .unwrap()
}
