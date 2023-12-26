use itertools::Itertools;
use num::bigint::*;
use num::rational::*;
use std::io;
use std::thread;

#[derive(Debug)]
struct Dir {
    a: BigRational,
    b: BigRational,
}

#[derive(Debug)]
struct Line {
    point: Dir,
    dir: Dir,
}

fn big_ratio(num: i64) -> BigRational {
    BigRational::from_integer(BigInt::from(num))
}

fn solve(lhs: &Line, rhs: &Line) -> bool {
    let mi = big_ratio(200000000000000);
    let ma = big_ratio(400000000000000);
    // let mi = big_ratioional::from_integer(7);
    // let ma = big_ratioional::from_integer(27);
    let zero = big_ratio(0);

    // A + B * v = C + D * u
    // A x D + B x D * v = C x D
    // B x D * v = C x D - A x D
    // v = (C x D - A x D) / B x D
    let cd = &rhs.point.a * &rhs.dir.b - &rhs.point.b * &rhs.dir.a;
    let ad = &lhs.point.a * &rhs.dir.b - &lhs.point.b * &rhs.dir.a;
    let bd = &lhs.dir.a * &rhs.dir.b - &lhs.dir.b * &rhs.dir.a;
    if bd == zero {
        return false;
    }

    let v = (cd - ad) / bd;
    if v < zero {
        return false;
    }

    let r = (
        &lhs.point.a + &lhs.dir.a * &v,
        &lhs.point.b + &lhs.dir.b * v,
    );
    let u = (&r.0 - &rhs.point.a) / &rhs.dir.a;
    if u < zero {
        return false;
    }
    return mi <= r.0 && r.0 <= ma && mi <= r.1 && r.1 <= ma;
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
                    let (a, b, _) = part
                        .split(',')
                        .map(|p| big_ratio(p.parse::<i64>().unwrap()))
                        .collect_tuple()
                        .unwrap();
                    Dir { a, b }
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
