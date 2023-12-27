use std::collections::{BTreeMap, HashSet};
use std::io;
use std::iter::Iterator;
use std::thread;

type Pos = (usize, usize);

#[derive(Debug)]
struct Edge {
    dist: usize,
    to: Pos,
}

#[derive(Debug)]
struct Node {
    name: Pos,
    children: Vec<Edge>,
}

impl Node {
    fn new(pos: Pos) -> Node {
        Node {
            name: pos,
            children: Vec::new(),
        }
    }
}

fn look_around(pos: &Pos) -> impl Iterator<Item = Pos> + '_ {
    [(-1, 0), (0, 1), (1, 0), (0, -1)].into_iter().map(|oft| {
        (
            ((pos.0 as i64) + oft.0) as usize,
            ((pos.1 as i64) + oft.1) as usize,
        )
    })
}

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

fn walk(
    pos: Pos,
    mut parent_pos: Pos,
    mut dist_to_parent: usize,
    graph: &mut BTreeMap<Pos, Node>,
    map: &mut Vec<Vec<char>>,
) {
    if "#OF".contains(map[pos.0][pos.1]) {
        return;
    }

    let at_junction: bool = look_around(&pos)
        .map(|p| ".JO".contains(map[p.0][p.1]) as usize)
        .sum::<usize>()
        >= 3
        || pos == (map.len() - 2, map[0].len() - 3);

    if at_junction {
        if map[pos.0][pos.1] == '.' {
            graph.insert(pos, Node::new(pos));
        }
        let cur_node = graph.get_mut(&pos).unwrap();
        cur_node.children.push(Edge {
            dist: dist_to_parent,
            to: parent_pos,
        });
        let parent_node = graph.get_mut(&parent_pos).unwrap();
        parent_node.children.push(Edge {
            dist: dist_to_parent,
            to: pos,
        });

        dist_to_parent = 0;
        parent_pos = pos;
    }

    let juntion_finished = look_around(&pos)
        .map(|p| ".".contains(map[p.0][p.1]) as usize)
        .sum::<usize>()
        == 0;

    map[pos.0][pos.1] = {
        if at_junction {
            if juntion_finished {
                'F'
            } else {
                'J'
            }
        } else {
            'O'
        }
    };

    for next_pos in look_around(&pos) {
        if next_pos == parent_pos {
            continue;
        }
        walk(next_pos, parent_pos, dist_to_parent + 1, graph, map);
    }
}

fn print_map(map: &Vec<Vec<char>>) {
    for r in map {
        println!("{}", r.iter().collect::<String>());
    }
    println!();
}

fn print_graphviz(graph: &BTreeMap<Pos, Node>) {
    println!("graph G{{");

    let node_name = |node: &Pos| format!("n_{}_{}", node.0, node.1);

    for node in graph.values() {
        println!(
            "{} [label=\"({}; {})\"];",
            node_name(&node.name),
            node.name.0,
            node.name.1
        );
    }

    let mut printed: HashSet<(Pos, Pos)> = HashSet::new();
    for node in graph.values() {
        for edge in &node.children {
            if printed.contains(&(node.name, edge.to)) {
                continue;
            }
            printed.insert((node.name, edge.to));
            printed.insert((edge.to, node.name));
            println!(
                "{} -- {} [label=\"{}\"];",
                node_name(&node.name),
                node_name(&edge.to),
                edge.dist
            );
        }
    }

    println!("}}");
}

fn run() {
    let mut map: Vec<Vec<char>> = expand(
        io::stdin()
            .lines()
            .map(|x| {
                x.unwrap()
                    .trim()
                    .replace("^", ".")
                    .replace(">", ".")
                    .replace("v", ".")
                    .replace("<", ".")
                    .to_string()
            })
            .filter(|line| !line.is_empty())
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect(),
        '#',
    );
    assert!(map[1][1..4] == ['#', '.', '#']);
    assert!(map[map.len() - 2][map[0].len() - 4..] == ['#', '.', '#', '#']);

    let start_pos = (1, 2);

    let mut graph: BTreeMap<Pos, Node> = BTreeMap::new();
    graph.insert(start_pos, Node::new(start_pos));
    walk(start_pos, start_pos, 0, &mut graph, &mut map);

    // print_map(&map);
    print_graphviz(&graph);
}

fn main() {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)
        .unwrap()
        .join()
        .unwrap();
}
