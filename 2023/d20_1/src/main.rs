use regex::Regex;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::io;
use std::thread;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug)]
enum Node {
    Passthrough(),
    FlipFlop(bool),
    Conjunction(HashMap<String, Pulse>),
}

#[derive(Debug)]
struct Module {
    name: String,
    node: Node,
    destinations: Vec<String>,
}

#[derive(Debug)]
struct Action {
    from: String,
    to: String,
    pulse: Pulse,
}

fn read_modules() -> HashMap<String, Module> {
    let re_module =
        Regex::new(r"^\s*(?<modifier>(:?[%&])?)(?<name>\w+)\s*->\s*(?<dest>(:?\w|\s|,)+)\s*$")
            .unwrap();
    let mut modules: HashMap<String, Module> = io::stdin()
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
                },
            )
        })
        .collect();

    let mut parents: HashMap<String, Vec<String>> = HashMap::new();
    for (name, module) in &modules {
        for dest in &module.destinations {
            modules.get(dest).map(|dest_module| {
                if let Node::Conjunction(_) = &dest_module.node {
                    parents.entry(dest.clone()).or_default().push(name.clone());
                }
            });
        }
    }

    for module in modules.values_mut() {
        if let Node::Conjunction(paren) = &mut module.node {
            for parent in parents.get(&module.name).unwrap() {
                paren.insert(parent.clone(), Pulse::Low);
            }
        }
    }

    modules
}

fn run() {
    let mut modules = read_modules();
    let mut queue: VecDeque<Action> = VecDeque::new();
    let mut result: HashMap<Pulse, i64> = HashMap::new();

    for _ in 0..1_000 {
        queue.push_back(Action {
            from: "".to_string(),
            to: "broadcaster".to_string(),
            pulse: Pulse::Low,
        });

        while let Some(action) = queue.pop_front() {
            *result.entry(action.pulse.clone()).or_default() += 1;

            let maybe_module = modules.get_mut(&action.to);
            if maybe_module.is_none() {
                continue;
            }
            let module = maybe_module.unwrap();

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

    println!("Result: {}", result.values().product::<i64>());
}

fn main() {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)
        .unwrap()
        .join()
        .unwrap();
}
