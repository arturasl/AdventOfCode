use ahash::HashMapExt;
use anyhow::{bail, ensure, Context, Error, Ok, Result};
use memoize::memoize;
use std::io::{self, BufRead};
use std::str::FromStr;
use std::thread;

#[derive(Debug, PartialEq, Eq)]
enum Op {
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

impl FromStr for Op {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "add" => Op::Add,
            "mul" => Op::Mul,
            "div" => Op::Div,
            "mod" => Op::Mod,
            "eql" => Op::Eql,
            _ => bail!(format!("Could not parse Op: {}", s)),
        })
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum Register {
    X = 0,
    Y = 1,
    Z = 2,
    W = 3,
}

impl FromStr for Register {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "x" => Register::X,
            "y" => Register::Y,
            "z" => Register::Z,
            "w" => Register::W,
            _ => bail!(format!("Could not parse register: {}", s)),
        })
    }
}

#[derive(Debug)]
enum Var {
    Reg(Register),
    Num(i64),
}

impl FromStr for Var {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Register::from_str(s)
            .map(Var::Reg)
            .or_else(|_| Ok(Var::Num(s.parse::<i64>()?)))
    }
}

#[derive(Debug)]
struct Exp {
    lhs: Register,
    op: Op,
    rhs: Var,
}

#[derive(Debug)]
struct ExpChunk {
    exps: Vec<Exp>,
}

#[derive(Debug, Clone)]
struct State {
    reg_vals: [Option<i64>; 4],
}

fn eval(state: &mut State, exp_chunk: &ExpChunk) -> Result<bool> {
    for exp in exp_chunk.exps.iter() {
        let new_lhs_val: i64 = match state.reg_vals[exp.lhs as usize] {
            None => {
                ensure!(exp.op == Op::Mul);
                ensure!(matches!(exp.rhs, Var::Num(0)));
                0
            }
            Some(lhs_val) => {
                let rhs_val: i64 = match &exp.rhs {
                    Var::Reg(reg) => state.reg_vals[*reg as usize].context("")?,
                    Var::Num(num) => *num,
                };

                match exp.op {
                    Op::Add => lhs_val + rhs_val,
                    Op::Mul => lhs_val * rhs_val,
                    Op::Div => {
                        if rhs_val == 0 {
                            return Ok(false);
                        }
                        lhs_val / rhs_val
                    }
                    Op::Mod => {
                        if lhs_val < 0 || rhs_val <= 0 {
                            return Ok(false);
                        }
                        lhs_val % rhs_val
                    }
                    Op::Eql => i64::from(lhs_val == rhs_val),
                }
            }
        };

        state.reg_vals[exp.lhs as usize] = Some(new_lhs_val);
    }

    Ok(true)
}

#[memoize(Ignore: problem, CustomHasher: ahash::HashMap)]
fn solve(z: i64, idx: usize, problem: &[ExpChunk]) -> Option<i64> {
    if idx == problem.len() {
        return if z == 0 { Some(0) } else { None };
    }

    for w in (1..=9).rev() {
        let mut state = State {
            reg_vals: [None; 4],
        };
        state.reg_vals[Register::Z as usize] = Some(z);
        state.reg_vals[Register::W as usize] = Some(w);
        if eval(&mut state, &problem[idx]).ok()? {
            if let Some(res) = solve(state.reg_vals[Register::Z as usize]?, idx + 1, problem) {
                return Some(w * 10i64.checked_pow((problem.len() - idx - 1) as u32)? + res);
            }
        }
    }

    None
}

fn run() -> Result<()> {
    let mut problem: Vec<ExpChunk> = Vec::new();
    for maybe_line in io::stdin().lock().lines() {
        let parts: Vec<String> = maybe_line?
            .trim()
            .split(" ")
            .map(ToOwned::to_owned)
            .collect();
        if parts.is_empty() {
            continue;
        }

        if parts[0] == "inp" {
            ensure!(parts.len() == 2 && parts[1] == "w");
            problem.push(ExpChunk { exps: Vec::new() });
            continue;
        }

        ensure!(parts.len() == 3 && !problem.is_empty());
        problem.last_mut().context("")?.exps.push(Exp {
            lhs: Register::from_str(&parts[1])?,
            op: Op::from_str(&parts[0])?,
            rhs: Var::from_str(&parts[2])?,
        })
    }

    println!("{}", solve(0, 0, &problem).unwrap());

    Ok(())
}

fn main() -> Result<()> {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)?
        .join()
        .unwrap()
}
