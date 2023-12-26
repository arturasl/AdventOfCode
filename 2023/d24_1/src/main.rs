use itertools::Itertools;
use std::io;
use std::thread;

#[derive(Debug)]
struct Dir {
    a: f64,
    b: f64,
    c: f64,
}

#[derive(Debug)]
struct Line {
    point: Dir,
    dir: Dir,
}

const MI: f64 = 200000000000000f64;
const MA: f64 = 400000000000000f64;
// const MI: f64 = 7f64;
// const MA: f64 = 27f64;

fn solve(lhs: &Line, rhs: &Line) -> bool {
    // A + B * v = C + D * u
    // A x D + B x D * v = C x D
    // B x D * v = C x D - A x D
    // v = (C x D - A x D) / B x D
    let cd = rhs.point.a * rhs.dir.b - rhs.point.b * rhs.dir.a;
    let ad = lhs.point.a * rhs.dir.b - lhs.point.b * rhs.dir.a;
    let bd = lhs.dir.a * rhs.dir.b - lhs.dir.b * rhs.dir.a;
    if bd.abs() < 0.000001 {
        return false;
    }

    let v = (cd - ad) / bd;
    if v < 0f64 {
        return false;
    }

    let r = (lhs.point.a + lhs.dir.a * v, lhs.point.b + lhs.dir.b * v);
    let u = (r.0 - rhs.point.a) / rhs.dir.a;
    if u < 0f64 {
        return false;
    }
    return MI <= r.0 && r.0 <= MA && MI <= r.1 && r.1 <= MA;
}

fn run() {
    let lines: Vec<Line> = io::stdin()
        .lines()
        .map(|x| x.unwrap().trim().replace(" ", "").to_string())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (point, dir) = line
                .split('@')
                .map(|part| {
                    let (a, b, c) = part
                        .split(',')
                        .map(|p| p.parse::<f64>().unwrap())
                        .collect_tuple()
                        .unwrap();
                    Dir { a, b, c }
                })
                .collect_tuple()
                .unwrap();
            Line { point, dir }
        })
        .collect();

    let mut result: usize = 0;
    for i in 0..lines.len() {
        for j in i + 1..lines.len() {
            result += solve(&lines[i], &lines[j]) as usize;
        }
    }
    println!("{result}");
}

fn main() {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)
        .unwrap()
        .join()
        .unwrap();
}
