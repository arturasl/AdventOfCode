use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rayon::prelude::*;
use std::collections::{BTreeMap, HashMap};
use std::io;
use std::iter::once;
use std::thread;

#[derive(Debug)]
struct Node {
    children: Vec<usize>,
}

#[derive(Debug, Clone)]
struct State {
    mi: usize,
    ma: usize,
    in_stack: bool,
}

fn read_graph() -> Vec<Node> {
    let map: BTreeMap<String, Vec<String>> = io::stdin()
        .lines()
        .map(|x| x.unwrap().trim().to_string())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (name, children) = line.split(':').collect_tuple().unwrap();
            (
                name.to_string(),
                children.split_whitespace().map(|x| x.to_string()).collect(),
            )
        })
        .collect();

    let mut nodes: Vec<Node> = Vec::new();
    let mut name_to_idx: HashMap<String, usize> = HashMap::new();

    for name in map.iter().flat_map(|(n, c)| c.iter().chain(once(n))) {
        if name_to_idx.contains_key(name) {
            continue;
        }
        let len = nodes.len();
        name_to_idx.insert(name.clone(), len);
        nodes.push(Node {
            children: Vec::new(),
        });
    }

    for (name, child_names) in &map {
        let node_idx = *name_to_idx.get(name).unwrap();
        for child_name in child_names {
            let child_idx = *name_to_idx.get(child_name).unwrap();
            nodes[node_idx].children.push(child_idx);
            nodes[child_idx].children.push(node_idx);
        }
    }

    nodes
}

fn walk(
    cur_idx: usize,
    parent_idx: usize,
    ma: usize,
    states: &mut Vec<State>,
    nodes: &Vec<Node>,
    taken_edges: &Vec<(usize, usize)>,
) -> usize {
    if states[cur_idx].in_stack {
        return states[cur_idx].mi;
    }
    if states[cur_idx].ma != 0 {
        return usize::MAX;
    }

    states[cur_idx].in_stack = true;
    states[cur_idx].mi = ma;
    states[cur_idx].ma = ma;

    for child_idx in &nodes[cur_idx].children {
        if taken_edges.contains(&(cur_idx, *child_idx)) || *child_idx == parent_idx {
            continue;
        }

        states[cur_idx].mi = states[cur_idx].mi.min(walk(
            *child_idx,
            cur_idx,
            ma + 1,
            states,
            nodes,
            taken_edges,
        ));
    }

    states[cur_idx].in_stack = false;
    states[cur_idx].mi
}

fn do_walk(
    nodes: &Vec<Node>,
    taken_edges: &Vec<(usize, usize)>,
) -> (Option<Vec<(usize, usize)>>, usize) {
    let mut states: Vec<State> = vec![
        State {
            mi: usize::MAX,
            ma: 0,
            in_stack: false,
        };
        nodes.len()
    ];
    walk(0, 0, 1, &mut states, nodes, taken_edges);

    let mut visited: usize = 0;
    for node_idx in 0..nodes.len() {
        visited += (states[node_idx].ma != 0) as usize;
        for child_idx in &nodes[node_idx].children {
            if states[node_idx].ma + 1 == states[*child_idx].mi
                && !taken_edges.contains(&(node_idx, *child_idx))
            {
                return (
                    Some(vec![(node_idx, *child_idx), taken_edges[0], taken_edges[2]]),
                    visited,
                );
            }
        }
    }

    (None, visited)
}

fn run() {
    let nodes: Vec<Node> = read_graph();

    let mut unique_edges: Vec<(usize, usize)> = (0..nodes.len())
        .flat_map(|lhs_idx| {
            nodes[lhs_idx]
                .children
                .iter()
                .map(move |rhs_idx| (lhs_idx, *rhs_idx))
                .filter(|(lhs_idx, rhs_idx)| lhs_idx < rhs_idx)
        })
        .collect();
    unique_edges.shuffle(&mut thread_rng());

    let taken_edges: Vec<(usize, usize)> = unique_edges
        .iter()
        .combinations(2)
        .map(|t| {
            t.into_iter()
                .map(|(l, r)| vec![(*l, *r), (*r, *l)])
                .concat()
        })
        .collect::<Vec<Vec<(usize, usize)>>>()
        .par_iter()
        .map(|taken_edges| do_walk(&nodes, taken_edges).0)
        .find_any(|f| f.is_some())
        .map(|f| f.unwrap())
        .unwrap();

    println!("Edges: {taken_edges:?}");
    let num_visited = do_walk(
        &nodes,
        &taken_edges
            .into_iter()
            .map(|(l, r)| vec![(l, r), (r, l)])
            .concat(),
    )
    .1;
    println!("Result: {}", num_visited * (nodes.len() - num_visited));
}

fn main() {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)
        .unwrap()
        .join()
        .unwrap();
}
