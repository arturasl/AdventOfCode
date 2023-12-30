use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::{BTreeMap, HashMap};
use std::io;
use std::iter::once;
use std::thread;

#[derive(Debug)]
struct Node {
    children: Vec<usize>,
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
            mi: 0,
            ma: 0,
            in_stack: false,
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
    nodes: &mut Vec<Node>,
    taken_edges: &Vec<(usize, usize)>,
) -> usize {
    if nodes[cur_idx].in_stack {
        return nodes[cur_idx].mi;
    }
    if nodes[cur_idx].ma != 0 {
        return usize::MAX;
    }

    nodes[cur_idx].in_stack = true;
    nodes[cur_idx].mi = ma;
    nodes[cur_idx].ma = ma;

    for child_idx in nodes[cur_idx].children.clone() {
        if taken_edges.contains(&(cur_idx, child_idx)) || child_idx == parent_idx {
            continue;
        }

        nodes[cur_idx].mi =
            nodes[cur_idx]
                .mi
                .min(walk(child_idx, cur_idx, ma + 1, nodes, taken_edges));
    }

    nodes[cur_idx].in_stack = false;
    nodes[cur_idx].mi
}

fn do_walk(nodes: &mut Vec<Node>, taken_edges: &Vec<(usize, usize)>) {
    for node in nodes.iter_mut() {
        node.mi = usize::MAX;
        node.ma = 0;
        node.in_stack = false;
    }
    walk(0, 0, 1, nodes, taken_edges);
}

fn run() {
    let mut nodes: Vec<Node> = read_graph();

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

    let mut found: Vec<(usize, usize)> = Vec::new();

    for taken_edges in unique_edges.iter().combinations(2).map(|t| {
        t.into_iter()
            .map(|(l, r)| vec![(*l, *r), (*r, *l)])
            .concat()
    }) {
        if !found.is_empty() {
            break;
        }

        do_walk(&mut nodes, &taken_edges);

        for node_idx in 0..nodes.len() {
            for child_idx in &nodes[node_idx].children {
                if nodes[node_idx].ma + 1 == nodes[*child_idx].mi
                    && !taken_edges.contains(&(node_idx, *child_idx))
                {
                    found = vec![(node_idx, *child_idx), taken_edges[0], taken_edges[2]];
                }
            }
        }
    }

    println!("Edges: {found:?}");
    let taken_edges = found
        .into_iter()
        .map(|(l, r)| vec![(l, r), (r, l)])
        .concat();
    do_walk(&mut nodes, &taken_edges);

    let num_visited: usize = nodes.iter().filter(|n| n.ma != 0).count();
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
