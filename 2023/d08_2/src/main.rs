use num::integer::lcm;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, BufRead};

type Node = (String, usize);

fn main() -> Result<(), Box<dyn Error>> {
    let re_nodes = Regex::new(r"^(?<start>\w{3})\s*=\s*\((?<left>\w{3})\s*,\s*(?<right>\w{3})\)$")?;

    let mut line_iter = io::stdin().lock().lines();

    let dirs: Vec<char> = line_iter.next().unwrap()?.trim().chars().collect();
    line_iter.next();

    let mut map: HashMap<String, (String, String)> = HashMap::new();
    for maybe_line in line_iter {
        let line: String = maybe_line?.trim().to_string();
        let captures = re_nodes.captures(&line).unwrap();
        assert!(map
            .insert(
                captures["start"].to_string(),
                (captures["left"].to_string(), captures["right"].to_string()),
            )
            .is_none());
    }

    let mut skip_paths: HashMap<String, HashMap<Node, (Node, usize)>> = HashMap::new();
    for k in map.keys() {
        if k.chars().last().unwrap() != 'A' {
            continue;
        }

        let mut cur_skip: Node = (k.to_string(), 0usize);
        let mut cur: Node = (k.to_string(), 0usize);
        let mut skips: HashMap<Node, (Node, usize)> = HashMap::new();
        let mut distance: usize = 0;

        loop {
            if cur.0.chars().last().unwrap() == 'Z' {
                if skips.insert(cur_skip, (cur.clone(), distance)).is_some() {
                    break;
                }
                cur_skip = cur.clone();
                distance = 0;
            }

            let next_pos = match dirs[cur.1] {
                'L' => map.get(&cur.0).unwrap().0.clone(),
                'R' => map.get(&cur.0).unwrap().1.clone(),
                _ => panic!(),
            };

            cur = (next_pos, (cur.1 + 1) % dirs.len());
            distance += 1;
        }

        skip_paths.insert(k.to_string(), skips);
    }

    // *Z is visited only once per path, cycle size and offsets are the same.
    let mut result: usize = 1;
    for (k, v) in skip_paths {
        println!("{:?}: {:?}", k, v);
        result = lcm(result, v.values().nth(0).unwrap().1);
    }
    println!("Result: {}", result);

    // let mut state: HashMap<String, (Node, usize)> = HashMap::new();
    // for k in skip_paths.keys() {
    //     state.insert(k.clone(), ((k.clone(), 0), 0));
    // }
    //
    // let mut iterations: usize = 0;
    // loop {
    //     let first: usize = state.iter().nth(0).unwrap().1 .1;
    //     if first != 0 && state.iter().all(|x| x.1 .1 == first) {
    //         break;
    //     }
    //
    //     let to_move: String = state
    //         .iter()
    //         .min_by_key(|x| x.1 .1)
    //         .map(|x| x.0.clone())
    //         .unwrap();
    //
    //     state.entry(to_move.clone()).and_modify(|x| {
    //         let (next_node, distance) = skip_paths.get(&to_move).unwrap().get(&x.0).unwrap();
    //         *x = (next_node.clone(), x.1 + distance);
    //     });
    //
    //     iterations += 1;
    //
    //     if iterations % 100000000 == 0 {
    //         println!("State: {:?}", state);
    //     }
    // }
    //
    // println!("Result: {}", state.iter().nth(0).unwrap().1 .1);

    Ok(())
}
