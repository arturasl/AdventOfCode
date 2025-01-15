use anyhow::{ensure, Context, Ok, Result};
use regex::Regex;
use rstest::rstest;
use std::io::{self, BufRead};
use std::thread;

#[derive(Debug, PartialEq, Eq)]
struct Cube {
    xrng: (i64, i64),
    yrng: (i64, i64),
    zrng: (i64, i64),
}

impl Cube {
    fn volume(&self) -> i64 {
        (self.xrng.1 - self.xrng.0).abs()
            * (self.yrng.1 - self.yrng.0).abs()
            * (self.zrng.1 - self.zrng.0).abs()
    }

    fn is_empty(&self) -> bool {
        self.xrng.1 == self.xrng.0 || self.yrng.1 == self.yrng.0 || self.zrng.1 == self.zrng.0
    }

    fn intersect(&self, other: &Cube) -> Cube {
        let fn_intersect_rngs = |lhs: &(i64, i64), rhs: &(i64, i64)| -> (i64, i64) {
            let rng = (lhs.0.max(rhs.0), lhs.1.min(rhs.1));
            if rng.0 >= rng.1 {
                (0, 0)
            } else {
                rng
            }
        };

        Cube {
            xrng: fn_intersect_rngs(&self.xrng, &other.xrng),
            yrng: fn_intersect_rngs(&self.yrng, &other.yrng),
            zrng: fn_intersect_rngs(&self.zrng, &other.zrng),
        }
    }

    fn split(&self, other: &Cube) -> Result<Vec<Cube>> {
        ensure!(self.intersect(other) == *other);
        ensure!(!self.is_empty());
        ensure!(!other.is_empty());

        Ok([
            Cube {
                xrng: self.xrng,
                yrng: self.yrng,
                zrng: (self.zrng.0, other.zrng.0),
            },
            Cube {
                xrng: self.xrng,
                yrng: self.yrng,
                zrng: (other.zrng.1, self.zrng.1),
            },
            Cube {
                xrng: (self.xrng.0, other.xrng.0),
                yrng: self.yrng,
                zrng: other.zrng,
            },
            Cube {
                xrng: (other.xrng.1, self.xrng.1),
                yrng: self.yrng,
                zrng: other.zrng,
            },
            Cube {
                xrng: other.xrng,
                yrng: (self.yrng.0, other.yrng.0),
                zrng: other.zrng,
            },
            Cube {
                xrng: other.xrng,
                yrng: (other.yrng.1, self.yrng.1),
                zrng: other.zrng,
            },
        ]
        .into_iter()
        .filter(|c| !c.is_empty())
        .collect::<Vec<Cube>>())
    }
}

#[derive(Debug)]
struct Action {
    action: bool,
    cube: Cube,
}

fn calc_int(covering: &Cube, actions: &[Action]) -> Result<i64> {
    if actions.is_empty() {
        return Ok(0);
    }

    let other = covering.intersect(&actions[0].cube);
    if other.is_empty() {
        return calc_int(covering, &actions[1..]);
    }

    Ok(i64::from(actions[0].action) * other.volume()
        + covering
            .split(&other)?
            .into_iter()
            .map(|sub| calc_int(&sub, &actions[1..]))
            .sum::<Result<i64>>()?)
}

fn calc(actions: &[Action]) -> Result<i64> {
    calc_int(
        &Cube {
            xrng: (i64::MIN, i64::MAX),
            yrng: (i64::MIN, i64::MAX),
            zrng: (i64::MIN, i64::MAX),
        },
        actions,
    )
}

fn run() -> Result<()> {
    let re_action = Regex::new(
        r"^(?<action>on|off) x=(?<xmin>-?\d+)..(?<xmax>-?\d+),y=(?<ymin>-?\d+)..(?<ymax>-?\d+),z=(?<zmin>-?\d+)..(?<zmax>-?\d+)$",
    )?;

    let mut actions: Vec<Action> = Vec::new();
    for maybe_line in io::stdin().lock().lines() {
        let line = maybe_line?.trim().to_owned();
        if line.is_empty() {
            continue;
        }

        let caps = re_action.captures(&line).context("")?;
        actions.push(Action {
            action: &caps["action"] == "on",
            cube: Cube {
                xrng: (caps["xmin"].parse()?, caps["xmax"].parse::<i64>()? + 1),
                yrng: (caps["ymin"].parse()?, caps["ymax"].parse::<i64>()? + 1),
                zrng: (caps["zmin"].parse()?, caps["zmax"].parse::<i64>()? + 1),
            },
        });
    }

    actions.reverse();

    println!("{}", calc(&actions)?);

    Ok(())
}

fn main() -> Result<()> {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)?
        .join()
        .unwrap()
}

#[rstest]
#[case(Cube{xrng:(1, 3), yrng:(4, 7), zrng:(8, 12)}, 24)]
#[case(Cube{xrng:(1, 1), yrng:(4, 7), zrng:(8, 12)}, 0)]
#[case(Cube{xrng:(-1, -3), yrng:(-4, -7), zrng:(-8, -12)}, 24)]
fn volume_test(#[case] cube: Cube, #[case] expected: i64) {
    assert_eq!(cube.volume(), expected)
}

#[rstest]
#[case(
    Cube{xrng:(1, 3), yrng:(4, 7), zrng:(8, 12)},
    Cube{xrng:(1, 3), yrng:(4, 7), zrng:(8, 12)},
    Cube{xrng:(1, 3), yrng:(4, 7), zrng:(8, 12)}
)]
#[case(
    Cube{xrng:(1, 2), yrng:(1, 2), zrng:(1, 2)},
    Cube{xrng:(3, 5), yrng:(3, 5), zrng:(3, 5)},
    Cube{xrng:(0, 0), yrng:(0, 0), zrng:(0, 0)}
)]
#[case(
    Cube{xrng:(0, 10), yrng:(0, 10), zrng:(0, 10)},
    Cube{xrng:(3, 5), yrng:(3, 5), zrng:(3, 5)},
    Cube{xrng:(3, 5), yrng:(3, 5), zrng:(3, 5)}
)]
#[case(
    Cube{xrng:(0, 10), yrng:(0, 10), zrng:(0, 10)},
    Cube{xrng:(5, 15), yrng:(5, 15), zrng:(5, 15)},
    Cube{xrng:(5, 10), yrng:(5, 10), zrng:(5, 10)}
)]
fn intersect_test(#[case] lhs: Cube, #[case] rhs: Cube, #[case] expected: Cube) {
    assert_eq!(lhs.intersect(&rhs), expected);
    assert_eq!(rhs.intersect(&lhs), expected);
}

#[rstest]
#[case(
    Cube{xrng:(1, 10), yrng:(2, 11), zrng:(3, 12)},
    Cube{xrng:(4, 6), yrng:(4, 6), zrng:(4, 6)},
    vec![
        Cube{xrng:(1, 10), yrng:(2, 11), zrng:(3, 4)},
        Cube{xrng:(1, 10), yrng:(2, 11), zrng:(6, 12)},
        Cube{xrng:(1, 4), yrng:(2, 11), zrng:(4, 6)},
        Cube{xrng:(6, 10), yrng:(2, 11), zrng:(4, 6)},
        Cube{xrng:(4, 6), yrng:(2, 4), zrng:(4, 6)},
        Cube{xrng:(4, 6), yrng:(6, 11), zrng:(4, 6)},
    ]
)]
fn split_test(#[case] lhs: Cube, #[case] rhs: Cube, #[case] expected: Vec<Cube>) {
    assert_eq!(lhs.split(&rhs).unwrap(), expected);
    assert_eq!(
        lhs.volume(),
        rhs.volume() + expected.iter().map(|c| c.volume()).sum::<i64>()
    );
}

#[rstest]
#[case(
    vec![Action{action: true,cube: Cube{xrng: (1, 2), yrng: (1, 2), zrng: (1, 2)}}],
    1
)]
#[case(
    vec![
        Action{action: true, cube: Cube{xrng: (1, 2), yrng: (1, 2), zrng: (1, 2)}},
        Action{action: true, cube: Cube{xrng: (1, 2), yrng: (1, 2), zrng: (1, 2)}},
        Action{action: false, cube: Cube{xrng: (1, 2), yrng: (1, 2), zrng: (1, 2)}},
        Action{action: true, cube: Cube{xrng: (1, 2), yrng: (1, 2), zrng: (1, 2)}}
    ],
    1
)]
#[case(
    vec![
        Action{action: false, cube: Cube{xrng: (1, 2), yrng: (1, 2), zrng: (1, 2)}},
        Action{action: true, cube: Cube{xrng: (1, 2), yrng: (1, 2), zrng: (1, 2)}},
    ],
    0
)]
#[case(
    vec![
        Action{action: true, cube: Cube{xrng: (1, 2), yrng: (1, 2), zrng: (1, 2)}},
        Action{action: true, cube: Cube{xrng: (2, 3), yrng: (2, 3), zrng: (2, 3)}},
    ],
    2
)]
#[case(
    vec![
        Action{action: true, cube: Cube{xrng: (1, 2), yrng: (1, 2), zrng: (1, 2)}},
        Action{action: true, cube: Cube{xrng: (-10, -9), yrng: (-8, -7), zrng: (-6, -5)}},
        Action{action: true, cube: Cube{xrng: (2, 3), yrng: (2, 3), zrng: (2, 3)}},
    ],
    3
)]
fn calc_test(#[case] actions: Vec<Action>, #[case] expected: i64) {
    assert_eq!(calc(&actions).unwrap(), expected);
}
