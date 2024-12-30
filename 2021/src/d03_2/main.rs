use anyhow::{ensure, Context, Ok, Result};
use std::io::{self, BufRead};
use std::thread;

struct Edge {
    cnt: i64,
    other: Node,
}

struct Node {
    edges: Vec<Edge>,
}

impl Edge {
    fn new() -> Edge {
        Edge {
            cnt: 0,
            other: Node::new(),
        }
    }
}

impl Node {
    fn new() -> Node {
        Node { edges: vec![] }
    }
}

fn add(node: &mut Node, bits: &[usize], i: usize) -> Result<()> {
    if i == bits.len() {
        return Ok(());
    }

    if node.edges.is_empty() {
        node.edges.push(Edge::new());
        node.edges.push(Edge::new());
    }

    node.edges[bits[i]].cnt += 1;
    add(&mut node.edges[bits[i]].other, bits, i + 1)
}

fn best(node: &Node, depth: i64, f: &dyn Fn(i64, i64) -> bool) -> Result<i64> {
    if node.edges.is_empty() {
        ensure!(depth == 0);
        return Ok(0);
    }
    ensure!(depth > 0);

    let idx = usize::from(f(node.edges[1].cnt, node.edges[0].cnt));
    Ok(((idx as i64) << (depth - 1)) | best(&node.edges[idx].other, depth - 1, f)?)
}

fn run() -> Result<()> {
    let mut graph = Node::new();

    let mut depth: i64 = 0;
    for maybe_line in io::stdin().lock().lines() {
        let bits: Vec<usize> = maybe_line?
            .trim()
            .chars()
            .map(|c| Ok(c.to_digit(10).context("")? as usize))
            .collect::<Result<_>>()?;
        if bits.is_empty() {
            continue;
        }

        depth = bits.len() as i64;
        add(&mut graph, &bits, 0)?
    }

    let max = best(&graph, depth, &|lhs, rhs| lhs >= rhs)?;
    let min = best(&graph, depth, &|lhs, rhs| {
        (lhs < rhs && lhs != 0) || rhs == 0
    })?;

    println!("{:b} * {:b} = {}", max, min, max * min);

    Ok(())
}

fn main() -> Result<()> {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)?
        .join()
        .unwrap()
}
