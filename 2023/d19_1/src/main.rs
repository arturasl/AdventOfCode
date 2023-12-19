use regex::Regex;
use std::collections::HashMap;
use std::io;
use std::thread;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Check {
    letter: char,
    op: char,
    num: i64,
    target: String,
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Workflow {
    name: String,
    default: String,
    checks: Vec<Check>,
}

fn run() {
    let mut lines_it = io::stdin().lines().map(|x| x.unwrap().trim().to_string());

    let re_workflow = Regex::new(r"^(?<name>\w+)\{(?<checks>.*),(?<default>\w+)\}$").unwrap();
    let re_check = Regex::new(r"(?<letter>[xmas])(?<op>[<>])(?<num>\d+):(?<target>\w+)").unwrap();
    let workflows: HashMap<String, Workflow> = lines_it
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let capture = re_workflow.captures(&line).unwrap();

            Workflow {
                name: capture["name"].to_string(),
                default: capture["default"].to_string(),
                checks: re_check
                    .captures_iter(&capture["checks"])
                    .map(|c| Check {
                        letter: c["letter"].chars().next().unwrap(),
                        op: c["op"].chars().next().unwrap(),
                        num: c["num"].parse().unwrap(),
                        target: c["target"].to_string(),
                    })
                    .collect(),
            }
        })
        .map(|workflow| (workflow.name.clone(), workflow))
        .collect();

    let re_part = Regex::new(r"^\{x=(?<x>\d+),m=(?<m>\d+),a=(?<a>\d+),s=(?<s>\d+)\}$").unwrap();
    let parts: Vec<HashMap<char, i64>> = lines_it
        .filter(|line| !line.is_empty())
        .map(|line| {
            let capture = re_part.captures(&line).unwrap();
            let mut part: HashMap<char, i64> = HashMap::new();
            for letter in "xmas".chars() {
                part.insert(
                    letter,
                    capture[letter.to_string().as_str()].parse().unwrap(),
                );
            }

            part
        })
        .collect();

    let mut result: i64 = 0;
    for part in parts {
        let mut cur_workflow_name: String = "in".to_string();

        let accepted = loop {
            if cur_workflow_name == "A" {
                break true;
            }
            if cur_workflow_name == "R" {
                break false;
            }

            let cur_workflow: &Workflow = workflows.get(&cur_workflow_name).unwrap();
            let mut next_workflow_name: String = cur_workflow.default.clone();
            for check in &cur_workflow.checks {
                let cur_amount = *part.get(&check.letter).unwrap();
                if (cur_amount > check.num && check.op == '>')
                    || (cur_amount < check.num && check.op == '<')
                {
                    next_workflow_name = check.target.clone();
                    break;
                }
            }

            cur_workflow_name = next_workflow_name;
        };

        if accepted {
            result += part.values().sum::<i64>();
        }
    }

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
