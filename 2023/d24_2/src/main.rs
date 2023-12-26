use itertools::Itertools;
use std::io;
use z3::ast::{Ast, Real};
use z3::*;

#[derive(Debug)]
struct Line {
    point: Vec<i64>,
    dir: Vec<i64>,
}

fn main() {
    let lines: Vec<Line> = io::stdin()
        .lines()
        .map(|x| x.unwrap().trim().replace(" ", "").to_string())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (point, dir) = line
                .split('@')
                .map(|part| part.split(',').map(|p| p.parse::<i64>().unwrap()).collect())
                .collect_tuple()
                .unwrap();
            Line { point, dir }
        })
        .collect();

    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solv = Solver::new(&ctx);

    let var = |l: &str, r: usize| Real::new_const(&ctx, format!("{}{}", l, r));

    let rock_times: Vec<Real> = (0..lines.len()).map(|i| var("rt", i)).collect();
    let rp: Vec<Real> = (0..3).map(|i| var("rp", i)).collect();
    let rv: Vec<Real> = (0..3).map(|i| var("rv", i)).collect();

    let int = |x: i64| Real::from_real_str(&ctx, &x.to_string(), "1").unwrap();
    let zero = int(0);

    for (line, rt) in lines.iter().zip(&rock_times) {
        for i in 0..3 {
            solv.assert(
                &(&int(line.point[i]) + rt * &int(line.dir[i]))._eq(&(&rp[i] + rt * &rv[i])),
            );
        }

        solv.assert(&rt.ge(&zero));
    }

    assert_eq!(solv.check(), SatResult::Sat);
    let m = solv.get_model().unwrap();

    for p in &rp {
        println!("{:?}: {:?}", p, m.eval(p, true).unwrap());
    }

    let ext_int = |x: &Real| {
        let r = m.eval(x, true).unwrap().as_real().unwrap();
        assert_eq!(r.1, 1);
        assert_eq!(r.1, 1);
        r.0
    };
    println!("{}", rp.iter().map(|r| ext_int(r)).sum::<i64>());
}
