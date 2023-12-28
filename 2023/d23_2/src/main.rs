use fixedbitset::FixedBitSet;
use std::collections::{HashMap, HashSet};
use std::io;
use std::iter::Iterator;
use std::thread;

type Pos = (usize, usize);

#[derive(Debug)]
struct Edge {
    from_idx: usize,
    to_idx: usize,
    dist: usize,
}

impl Edge {
    fn other(&self, idx: usize) -> usize {
        if self.from_idx == idx {
            self.to_idx
        } else {
            assert!(self.to_idx == idx);
            self.from_idx
        }
    }
}

#[derive(Debug)]
struct Node {
    idx: usize,
    pos: Pos,
    edge_idxs: Vec<usize>,
}

struct Graph {
    nodes: Vec<Node>,
    pos_to_node_idx: HashMap<Pos, usize>,
    unique_edges: Vec<Edge>,
    start_node_idx: usize,
    end_node_idx: usize,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            nodes: Vec::new(),
            pos_to_node_idx: HashMap::new(),
            unique_edges: Vec::new(),
            start_node_idx: 0,
            end_node_idx: 0,
        }
    }

    fn upsert_node(&mut self, pos: &Pos) -> &mut Node {
        if let Some(prev_node_idx) = self.pos_to_node_idx.get(pos) {
            return &mut self.nodes[*prev_node_idx];
        }
        let len = self.nodes.len();
        self.nodes.push(Node {
            idx: len,
            pos: *pos,
            edge_idxs: Vec::new(),
        });
        self.pos_to_node_idx.insert(*pos, len);
        &mut self.nodes[len]
    }

    fn add_edge(&mut self, from_idx: usize, to_idx: usize, dist: usize) {
        self.unique_edges.push(Edge {
            from_idx: from_idx.min(to_idx),
            to_idx: from_idx.max(to_idx),
            dist,
        });

        self.nodes[from_idx]
            .edge_idxs
            .push(self.unique_edges.len() - 1);
        self.nodes[to_idx]
            .edge_idxs
            .push(self.unique_edges.len() - 1);
    }

    #[allow(dead_code)]
    fn print_graphviz(&self) {
        let node_name =
            |idx: usize| format!("n_{}_{}", self.nodes[idx].pos.0, self.nodes[idx].pos.1);

        println!("graph G{{");
        for node in &self.nodes {
            println!(
                "  {} [label=\"({}; {})\"];",
                node_name(node.idx),
                node.pos.0,
                node.pos.1
            );
        }

        println!();

        for edge in &self.unique_edges {
            println!(
                "  {} -- {} [label=\"{}\"];",
                node_name(edge.from_idx),
                node_name(edge.to_idx),
                edge.dist
            );
        }

        println!("}}");
    }
}

struct Map {
    map: Vec<Vec<char>>,
    start_pos: (usize, usize),
    end_pos: (usize, usize),
}

impl Map {
    fn expand(map: Vec<Vec<char>>, default: char) -> Vec<Vec<char>> {
        let cols: usize = map[0].len();

        vec![vec![default; cols + 2]]
            .into_iter()
            .chain(map.into_iter().map(|r| {
                vec![default]
                    .into_iter()
                    .chain(r)
                    .chain(vec![default])
                    .collect::<Vec<char>>()
            }))
            .chain(vec![vec![default; cols + 2]])
            .collect::<Vec<Vec<char>>>()
    }

    fn look_around(pos: &Pos) -> impl Iterator<Item = Pos> + '_ {
        [(-1, 0), (0, 1), (1, 0), (0, -1)].into_iter().map(|oft| {
            (
                ((pos.0 as i64) + oft.0) as usize,
                ((pos.1 as i64) + oft.1) as usize,
            )
        })
    }

    fn read() -> Map {
        let map: Vec<Vec<char>> = Map::expand(
            io::stdin()
                .lines()
                .map(|x| x.unwrap().trim().to_string())
                .filter(|line| !line.is_empty())
                .map(|line| {
                    line.chars()
                        .map(|ch| if "^>v<".contains(ch) { '.' } else { ch })
                        .collect::<Vec<char>>()
                })
                .collect(),
            '#',
        );
        let rows = map.len();
        let cols = map[0].len();
        assert!(map[1][1..4] == ['#', '.', '#']);
        assert!(map[rows - 2][cols - 4..] == ['#', '.', '#', '#']);

        Map {
            map,
            start_pos: (1, 2),
            end_pos: (rows - 2, cols - 3),
        }
    }

    fn walk(
        &mut self,
        pos: Pos,
        mut parent_idx: usize,
        mut dist_to_parent: usize,
        graph: &mut Graph,
    ) {
        if "#O".contains(self.map[pos.0][pos.1]) {
            return;
        }

        let at_junction: bool = Map::look_around(&pos)
            .map(|p| ".O".contains(self.map[p.0][p.1]) as usize)
            .sum::<usize>()
            >= 3;
        if at_junction || pos == self.end_pos {
            let cur_node_idx = graph.upsert_node(&pos).idx;
            graph.add_edge(cur_node_idx, parent_idx, dist_to_parent);

            dist_to_parent = 0;
            parent_idx = cur_node_idx;
        }

        let final_junction = Map::look_around(&pos)
            .map(|p| ".".contains(self.map[p.0][p.1]) as usize)
            .sum::<usize>()
            == 0;
        if at_junction && final_junction {
            return;
        }

        self.map[pos.0][pos.1] = if at_junction { 'J' } else { 'O' };

        for next_pos in Map::look_around(&pos) {
            if next_pos == graph.nodes[parent_idx].pos {
                continue;
            }
            self.walk(next_pos, parent_idx, dist_to_parent + 1, graph);
        }
    }

    fn destructively_to_graph(&mut self) -> Graph {
        let mut graph: Graph = Graph::new();
        graph.upsert_node(&self.start_pos);
        self.walk(self.start_pos, 0, 0, &mut graph);
        graph.start_node_idx = *graph.pos_to_node_idx.get(&self.start_pos).unwrap();
        graph.end_node_idx = *graph.pos_to_node_idx.get(&self.end_pos).unwrap();
        graph
    }

    #[allow(dead_code)]
    fn print(&self) {
        for r in &self.map {
            println!("{}", r.iter().collect::<String>());
        }
        println!();
    }
}

fn find_longest(
    cur_node_idx: usize,
    result: &mut usize,
    visited_edges: &mut FixedBitSet,
    enters: &mut usize,
    cache: &mut HashSet<FixedBitSet>,
    graph: &Graph,
) {
    if cur_node_idx == graph.end_node_idx {
        let cur_result = visited_edges
            .ones()
            .map(|p| graph.unique_edges[p].dist)
            .sum::<usize>();
        if *result < cur_result {
            *result = cur_result;
            println!("{cur_result}");
        }
        return;
    }

    *enters += 1;
    if cache.contains(visited_edges) {
        if *enters % 1000000 == 0 {
            println!("{visited_edges}");
        }
        return;
    }
    cache.insert(visited_edges.clone());

    for edge_idx in &graph.nodes[cur_node_idx].edge_idxs {
        if visited_edges[*edge_idx] {
            continue;
        }
        visited_edges.toggle(*edge_idx);
        find_longest(
            graph.unique_edges[*edge_idx].other(cur_node_idx),
            result,
            visited_edges,
            enters,
            cache,
            graph,
        );
        visited_edges.toggle(*edge_idx);
    }
}

fn run() {
    let mut map = Map::read();
    let graph = map.destructively_to_graph();
    graph.print_graphviz();

    let mut result: usize = 0;
    let mut visited_edges = FixedBitSet::with_capacity(graph.unique_edges.len());
    let mut cache: HashSet<FixedBitSet> = HashSet::new();
    let mut enters: usize = 0;
    find_longest(
        graph.start_node_idx,
        &mut result,
        &mut visited_edges,
        &mut enters,
        &mut cache,
        &graph,
    );
    println!("Result: {result}");
}

fn main() {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)
        .unwrap()
        .join()
        .unwrap();
}
