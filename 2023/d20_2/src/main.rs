use itertools::sorted;
use num::integer::lcm;
use regex::Regex;
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::io;
use std::thread;

const NODE_INIT: &str = "broadcaster";
const NODE_FINAL: &str = "rx";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Clone)]
enum Node {
    Passthrough(),
    FlipFlop(bool),
    Conjunction(HashMap<String, Pulse>),
}

impl Node {
    fn to_string(&self) -> String {
        match self {
            Node::Passthrough() => "-".to_string(),
            Node::FlipFlop(state) => {
                if *state {
                    "1".to_string()
                } else {
                    "0".to_string()
                }
            }
            Node::Conjunction(parents) => format!(
                "({})",
                sorted(
                    parents
                        .iter()
                        .map(|(k, p)| (k, if *p == Pulse::High { 'H' } else { 'L' }))
                )
                .map(|(_, p)| p)
                .collect::<String>()
            ),
        }
    }
}

#[derive(Debug, Clone)]
struct Module {
    name: String,
    node: Node,
    destinations: Vec<String>,
    parents: Vec<String>,
}

#[derive(Debug, Clone)]
struct Action {
    from: String,
    to: String,
    pulse: Pulse,
}

#[derive(Debug, Clone)]
struct Cycle {
    path_len: usize,
    len: usize,
    start_at: usize,
    high: (usize, usize),
    high_presses: usize,
}

fn read_modules() -> BTreeMap<String, Module> {
    let re_module =
        Regex::new(r"^\s*(?<modifier>(:?[%&])?)(?<name>\w+)\s*->\s*(?<dest>(:?\w|\s|,)+)\s*$")
            .unwrap();
    let mut modules: BTreeMap<String, Module> = io::stdin()
        .lines()
        .map(|x| x.unwrap().trim().to_string())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let captures = re_module.captures(&line).unwrap();
            (
                captures["name"].to_string(),
                Module {
                    name: captures["name"].to_string(),
                    node: match &captures["modifier"] {
                        "" => Node::Passthrough(),
                        "%" => Node::FlipFlop(false),
                        "&" => Node::Conjunction(HashMap::new()),
                        _ => panic!(),
                    },
                    destinations: captures["dest"]
                        .replace(" ", "")
                        .split(",")
                        .map(|dest| dest.to_string())
                        .collect(),
                    parents: vec![],
                },
            )
        })
        .collect();

    // Ensure all strings have an entry.
    let mut all_names: HashSet<String> = HashSet::new();
    for module in modules.values() {
        all_names.insert(module.name.clone());
        for dest in &module.destinations {
            all_names.insert(dest.to_string());
        }
    }
    for name in &all_names {
        if !modules.contains_key(name) {
            modules.insert(
                name.to_string(),
                Module {
                    name: name.to_string(),
                    node: Node::Passthrough(),
                    destinations: vec![],
                    parents: vec![],
                },
            );
        }
    }

    // Parents.
    let mut parents_per_module: HashMap<String, Vec<String>> =
        all_names.into_iter().map(|n| (n, vec![])).collect();
    for (name, module) in &modules {
        for dest in &module.destinations {
            parents_per_module.get_mut(dest).unwrap().push(name.clone());
        }
    }

    for module in modules.values_mut() {
        let parents = parents_per_module.get(&module.name).unwrap();
        if let Node::Conjunction(paren) = &mut module.node {
            for parent in parents {
                paren.insert(parent.clone(), Pulse::Low);
            }
        }
        module.parents = parents.clone();
    }

    modules
}

fn to_graphviz(modules: &HashMap<String, Module>) {
    println!("digraph G {{");

    for module in modules.values() {
        println!(
            "\"{}\" [label=\"{} / {}\"];",
            module.name,
            module.name,
            module.node.to_string()
        );
    }

    for module in modules.values() {
        for dest in &module.destinations {
            println!("\"{}\" -> \"{}\"", module.name, dest);
        }
    }

    println!("}}");
}

fn print_cycles(cycles: &[Cycle]) {
    print!(
        "{}",
        cycles
            .iter()
            .map(|c| format!("  {:?}\n", c))
            .collect::<String>()
    );
}

fn to_string(modules: &BTreeMap<String, Module>) -> String {
    modules
        .values()
        .map(|module| module.node.to_string())
        .collect::<String>()
}

fn find_cycle(mut modules: BTreeMap<String, Module>, init_action: Action) -> Cycle {
    let mut queue: VecDeque<Action> = VecDeque::new();
    let mut presses: usize = 0;
    let mut visited: HashMap<String, usize> = HashMap::new();

    let mut conjunction_high: (Option<usize>, Option<usize>) = (None, None);
    let mut high_presses: Option<usize> = None;

    loop {
        let str_modules = to_string(&modules);
        if let Some(start_at) = visited.get(&str_modules) {
            return Cycle {
                path_len: visited.len(),
                len: visited.len() - *start_at,
                start_at: *start_at,
                high: (conjunction_high.0.unwrap(), conjunction_high.1.unwrap() - 1),
                high_presses: high_presses.unwrap(),
            };
        }
        visited.insert(str_modules, presses);

        presses += 1;
        queue.push_back(init_action.clone());

        let mut steps: usize = 0;

        while let Some(action) = queue.pop_front() {
            if let Node::Conjunction(parents) = &modules.get("zh").unwrap().node {
                if parents.iter().any(|(_, p)| *p == Pulse::High) {
                    if conjunction_high.0.is_none() {
                        assert!(conjunction_high.1.is_none());
                        assert!(high_presses.is_none());
                        conjunction_high.0 = Some(steps);
                        high_presses = Some(presses);
                    }
                } else if conjunction_high.1.is_none() && conjunction_high.0.is_some() {
                    conjunction_high.1 = Some(steps);
                }
            }

            steps += 1;
            let module = modules.get_mut(&action.to).unwrap();

            if action.to == NODE_FINAL && action.pulse == Pulse::Low {
                panic!();
            }

            let maybe_next_pulse: Option<Pulse> = match &mut module.node {
                Node::Passthrough() => Some(action.pulse.clone()),
                Node::FlipFlop(state) => {
                    if action.pulse == Pulse::Low {
                        *state = !*state;
                        Some(if *state { Pulse::High } else { Pulse::Low })
                    } else {
                        None
                    }
                }
                Node::Conjunction(parents) => {
                    *parents.get_mut(&action.from).unwrap() = action.pulse.clone();
                    Some(if parents.values().all(|p| *p == Pulse::High) {
                        Pulse::Low
                    } else {
                        Pulse::High
                    })
                }
            };

            if let Some(next_pulse) = maybe_next_pulse {
                for dest in &module.destinations {
                    queue.push_back(Action {
                        from: module.name.to_string(),
                        to: dest.to_string(),
                        pulse: next_pulse.clone(),
                    });
                }
            }
        }
    }
}

fn run() {
    let modules = read_modules();
    assert!(modules.contains_key(NODE_INIT));
    assert!(modules.get(NODE_INIT).unwrap().parents.is_empty());
    assert!(modules.contains_key(NODE_FINAL));
    assert_eq!(modules.get(NODE_FINAL).unwrap().parents.len(), 1);
    assert!(modules.get(NODE_FINAL).unwrap().destinations.is_empty());
    // to_graphviz(&modules);

    let mut cycles: Vec<Cycle> = modules
        .get(NODE_INIT)
        .unwrap()
        .destinations
        .iter()
        .map(|dest| {
            find_cycle(
                modules.clone(),
                Action {
                    from: NODE_INIT.to_string(),
                    to: dest.clone(),
                    pulse: Pulse::Low,
                },
            )
        })
        .collect();

    println!("# Cycles");
    print_cycles(&cycles);

    let oft = cycles[0].start_at;
    assert!(cycles.iter().all(|c| c.start_at == oft));

    for cycle in &mut cycles {
        cycle.start_at -= oft;
        cycle.path_len -= oft;
        cycle.high_presses -= oft;
    }

    println!("# After offseting");
    print_cycles(&cycles);

    let common = cycles.iter().fold(1usize, |acc, x| lcm(acc, x.len));
    println!("# Common: {}", common);

    let cycle_oft = cycles.iter().fold(0usize, |acc, c| acc.max(c.high.0));
    assert!(cycles.iter().all(|c| cycle_oft <= c.high.1));
    println!("# Cycle oft: {}", cycle_oft);

    println!("# Result: {}", oft + common - 1);
}

fn main() {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)
        .unwrap()
        .join()
        .unwrap();
}
