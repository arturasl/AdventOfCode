use gcollections::ops::*;
use interval::interval_set::*;
use regex::Regex;
use std::collections::HashMap;
use std::io;
use std::thread;

#[derive(Debug)]
struct Check {
    letter: char,
    op: char,
    num: i64,
    target: String,
}

#[derive(Debug)]
struct Workflow {
    name: String,
    default: String,
    checks: Vec<Check>,
}

type Part = HashMap<char, IntervalSet<i64>>;

const MI: i64 = 1;
const MA: i64 = 4_000;

fn int(v: (i64, i64)) -> IntervalSet<i64> {
    vec![v].to_interval_set()
}

fn solve(
    get_to: (String, Option<usize>),
    workflows: &HashMap<String, Workflow>,
    reverse: &HashMap<String, Vec<(String, Option<usize>)>>,
) -> Vec<Part> {
    let mut cur_part: Part = "xmas".chars().map(|c| (c, int((MI, MA)))).collect();
    let workflow = workflows.get(&get_to.0).unwrap();

    if let Some(should_match) = get_to.1 {
        let check: &Check = &workflow.checks[should_match];
        let rng: (i64, i64) = if check.op == '>' {
            (check.num + 1, MA)
        } else {
            (MI, check.num - 1)
        };
        cur_part.insert(check.letter, int(rng));
    }

    for i in 0..get_to.1.unwrap_or(workflow.checks.len()) {
        let check: &Check = &workflow.checks[i];
        let rng: (i64, i64) = if check.op == '>' {
            (MI, check.num)
        } else {
            (check.num, MA)
        };
        cur_part
            .entry(check.letter)
            .and_modify(|e| *e = e.intersection(&int(rng)));
    }

    if get_to.0 == "in" {
        return vec![cur_part];
    }

    let mut result: Vec<Part> = vec![];
    for next_to in reverse.get(&get_to.0).unwrap() {
        for mut other_part in solve(next_to.clone(), workflows, reverse) {
            for (letter, segs) in &mut other_part {
                *segs = segs.intersection(cur_part.get(letter).unwrap());
            }

            if other_part.values().all(|seg| !seg.is_empty()) {
                result.push(other_part);
            }
        }
    }

    result
}

fn run() {
    let re_workflow = Regex::new(r"^(?<name>\w+)\{(?<checks>.*),(?<default>\w+)\}$").unwrap();
    let re_check = Regex::new(r"(?<letter>[xmas])(?<op>[<>])(?<num>\d+):(?<target>\w+)").unwrap();
    let workflows: HashMap<String, Workflow> = io::stdin()
        .lines()
        .map(|x| x.unwrap().trim().to_string())
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

    let mut reverse: HashMap<String, Vec<(String, Option<usize>)>> = HashMap::new();
    for workflow in workflows.values() {
        for (i, check) in workflow.checks.iter().enumerate() {
            reverse
                .entry(check.target.to_string())
                .or_default()
                .push((workflow.name.clone(), Some(i)));
        }
        reverse
            .entry(workflow.default.to_string())
            .or_default()
            .push((workflow.name.clone(), None));
    }

    // dbg!(solve(("pv".to_string(), None), &workflows, &reverse));

    let mut result: usize = 0;
    for go_to in reverse.get("A").unwrap() {
        for part in solve(go_to.clone(), &workflows, &reverse) {
            result += part.values().map(|x| x.size() as usize).product::<usize>();
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
