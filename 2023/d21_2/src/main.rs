#![allow(dead_code)]

use itertools::Itertools;
use rstest::*;
use std::collections::VecDeque;
use std::iter::once;

const NEEDED_STEPS: usize = 26_501_365;
const WIDTH: usize = 65;

fn perfect(steps: usize) -> usize {
    (steps + 1).pow(2)
}

#[rstest]
#[trace]
fn test_perfect(#[values(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10)] steps: usize) {
    let mut solution = Quadratic::new(&str_to_map("...\n.S.\n..."), steps);
    assert_eq!(solution.solve(), perfect(steps));
}

fn full(steps: usize) -> usize {
    if steps == 1 {
        return 1;
    }

    1 + 2 * ((steps - 1).pow(2) + (steps - 1))
}

fn expand(map: Vec<Vec<char>>, default: char) -> Vec<Vec<char>> {
    let cols: usize = map[0].len();

    once(vec![default; cols + 2])
        .chain(map.into_iter().map(|r| {
            once(default)
                .chain(r)
                .chain(once(default))
                .collect::<Vec<char>>()
        }))
        .chain(once(vec![default; cols + 2]))
        .collect()
}

fn find_s(map: &Vec<Vec<char>>) -> (usize, usize) {
    (0..map.len())
        .cartesian_product(0..map[0].len())
        .find(|(i, j)| map[*i][*j] == 'S')
        .unwrap()
}

fn repeat(mut map: Vec<Vec<char>>, copies: usize) -> Vec<Vec<char>> {
    assert!(copies % 2 == 0);
    let orig_rows = map.len();
    let orig_cols = map[0].len();
    let s_pos = find_s(&map);

    map[s_pos.0][s_pos.1] = '.';

    for _ in 0..copies {
        for row in map.iter_mut() {
            for j in 0..orig_cols {
                row.push(row[j]);
            }
        }
    }

    for _ in 0..copies {
        for i in 0..orig_rows {
            map.push(map[i].clone());
        }
    }

    let oft: usize = (1 + copies) / 2;
    map[oft * orig_rows + s_pos.0][oft * orig_cols + s_pos.1] = 'S';

    map
}

fn str_to_map(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|x| x.trim().to_string())
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
}

fn look_around(pos: &(usize, usize)) -> impl Iterator<Item = (usize, usize)> + '_ {
    [(-1, 0), (0, 1), (1, 0), (0, -1)].into_iter().map(|oft| {
        (
            ((pos.0 as i64) + oft.0) as usize,
            ((pos.1 as i64) + oft.1) as usize,
        )
    })
}

fn expand_to_fit(map: &Vec<Vec<char>>, steps: usize) -> Vec<Vec<char>> {
    let new_dim = (steps / map.len().min(map[0].len()) + 1) * 2;
    let mut new_map = repeat(map.clone(), new_dim);
    new_map = expand(new_map, '?');
    new_map
}

struct Qubic {
    visited: Vec<Vec<Vec<bool>>>,
    map: Vec<Vec<char>>,
    steps: usize,
}

impl Qubic {
    fn new(map: &Vec<Vec<char>>, steps: usize) -> Qubic {
        let expanded_map = expand_to_fit(map, steps);
        Qubic {
            visited: vec![vec![vec![false; expanded_map[0].len()]; expanded_map.len()]; steps + 1],
            map: expanded_map,
            steps,
        }
    }

    fn visit(&mut self, steps: usize, pos: (usize, usize)) {
        if steps == self.steps + 1
            || self.map[pos.0][pos.1] == '#'
            || self.visited[steps][pos.0][pos.1]
        {
            return;
        }
        self.visited[steps][pos.0][pos.1] = true;

        for next_pos in look_around(&pos) {
            self.visit(steps + 1, next_pos);
        }
    }

    fn solve(&mut self) -> usize {
        self.visit(0, find_s(&self.map));
        self.visited[self.steps]
            .iter()
            .map(|row| row.iter().map(|c| *c as usize).sum::<usize>())
            .sum::<usize>()
    }

    fn print(&self) {
        for i in 0..self.map.len() {
            for j in 0..self.map[i].len() {
                if self.visited[self.steps][i][j] {
                    print!("O");
                } else {
                    print!("{}", self.map[i][j]);
                }
            }
            println!();
        }
    }
}

#[rstest]
#[trace]
#[case(6, 16)]
#[case(10, 50)]
#[case(50, 1594)]
#[case(100, 6536)]
fn test_qubic(#[case] steps: usize, #[case] ans: usize) {
    let mut solution = Qubic::new(&str_to_map(include_str!("small.in")), steps);
    assert_eq!(solution.solve(), ans);
}

struct Quadratic {
    visited: Vec<Vec<bool>>,
    map: Vec<Vec<char>>,
    steps: usize,
    visited_counts: [usize; 2],
}

impl Quadratic {
    fn new(map: &Vec<Vec<char>>, steps: usize) -> Quadratic {
        let expanded_map = expand_to_fit(map, steps);
        Quadratic {
            visited: vec![vec![false; expanded_map[0].len()]; expanded_map.len()],
            map: expanded_map,
            steps,
            visited_counts: [0, 0],
        }
    }

    fn solve(&mut self) -> usize {
        let mut queue: VecDeque<((usize, usize), usize)> = VecDeque::new();
        queue.push_front((find_s(&self.map), 0));
        while let Some((cur_pos, cur_steps)) = queue.pop_back() {
            if cur_steps == self.steps + 1
                || self.visited[cur_pos.0][cur_pos.1]
                || self.map[cur_pos.0][cur_pos.1] == '#'
            {
                continue;
            }
            self.visited[cur_pos.0][cur_pos.1] = true;

            self.visited_counts[cur_steps % 2] += 1;

            for next_pos in look_around(&cur_pos) {
                queue.push_front((next_pos, cur_steps + 1));
            }
        }
        self.visited_counts[self.steps % 2]
    }

    fn print(&self) {
        for i in 0..self.map.len() {
            for j in 0..self.map[i].len() {
                if self.map[i][j] == '#' {
                    print!("#");
                } else if self.visited[i][j] {
                    print!("O");
                } else {
                    print!("{}", self.map[i][j]);
                }
            }
            println!();
        }
    }
}

#[rstest]
#[trace]
#[case(6, 16)]
#[case(10, 50)]
#[case(50, 1594)]
#[case(100, 6536)]
#[case(500, 167004)]
#[case(1000, 668697)]
fn test_quadratic(#[case] steps: usize, #[case] ans: usize) {
    let mut solution = Quadratic::new(&str_to_map(include_str!("small.in")), steps);
    assert_eq!(solution.solve(), ans);
}

#[rstest]
#[trace]
fn test_quadratic_same_as_qubic(#[values(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10)] steps: usize) {
    let mut quadratic = Quadratic::new(&str_to_map(include_str!("small.in")), steps);
    let mut qubic = Qubic::new(&str_to_map(include_str!("small.in")), steps);
    assert_eq!(quadratic.solve(), qubic.solve());
}

fn calc_for_large(steps: usize, reps: usize) -> usize {
    assert_eq!(steps, WIDTH + (WIDTH * 2 + 1) * reps);

    let middle = [593, 655];
    let side = [626, 618];

    let num_sides = 2 * reps * (reps + 1);
    let num_middle = 2 * reps * reps + 2 * reps + 1;

    perfect(steps)
        - (reps * reps) * middle[0]
        - (num_middle - (reps * reps)) * middle[1]
        - (num_sides / 2) * side[1]
        - (num_sides / 2) * side[0]
}

#[rstest]
#[trace]
fn test_calc_for_large(#[values(0, 1, 2, 3, 4, 5, 6)] reps: usize) {
    let steps = WIDTH + (WIDTH * 2 + 1) * reps;
    let mut quadratic = Quadratic::new(&str_to_map(include_str!("large.in")), steps);
    assert_eq!(quadratic.solve(), calc_for_large(steps, reps));
}

fn main() {
    let mut reps = NEEDED_STEPS - WIDTH;
    assert!(reps % (WIDTH * 2 + 1) == 0);
    reps /= WIDTH * 2 + 1;
    println!("{}", calc_for_large(NEEDED_STEPS, reps))
}
