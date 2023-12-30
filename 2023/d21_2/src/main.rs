use itertools::Itertools;
use rstest::*;
use std::io;
use std::iter::once;
use std::thread;

fn perfect(steps: usize) -> usize {
    (steps + 1).pow(2)
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

fn rot(map: &Vec<Vec<char>>, pos: (usize, usize)) -> (usize, usize) {
    assert!(pos.0 != 0 && pos.0 != map.len() - 1);
    assert!(pos.1 != 0 && pos.1 != map[pos.0].len() - 1);
    pos
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

struct Qubic {
    visited: Vec<Vec<Vec<bool>>>,
    map: Vec<Vec<char>>,
    steps: usize,
}

impl Qubic {
    fn new(mut map: Vec<Vec<char>>, steps: usize) -> Qubic {
        let new_dim = (steps / map.len().min(map[0].len()) + 1) * 2;
        map = repeat(map, new_dim);
        map = expand(map, '?');
        Qubic {
            visited: vec![vec![vec![false; map[0].len()]; map.len()]; steps + 1],
            map,
            steps,
        }
    }

    fn visit(self: &mut Qubic, steps: usize, pos: (usize, usize)) {
        if steps == self.steps + 1
            || self.map[pos.0][pos.1] == '#'
            || self.visited[steps][pos.0][pos.1]
        {
            return;
        }
        self.visited[steps][pos.0][pos.1] = true;

        for dir in [(0, 1), (2, 1), (1, 0), (1, 2)] {
            let next_pos = rot(&self.map, (pos.0 + dir.0 - 1, pos.1 + dir.1 - 1));
            self.visit(steps + 1, next_pos);
        }
    }

    fn solve(self: &mut Qubic) -> usize {
        self.visit(0, find_s(&self.map));
        self.visited[self.steps]
            .iter()
            .map(|row| row.iter().map(|c| *c as usize).sum::<usize>())
            .sum::<usize>()
    }

    fn print(self: &Qubic) {
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

fn run() {
    let map: Vec<Vec<char>> = str_to_map(
        &io::stdin()
            .lines()
            .map(|x| x.unwrap().trim().to_string())
            .intersperse("\n".to_string())
            .collect::<String>(),
    );

    // for steps in [66 * 1, 66 * 3 - 1, 66 * 5 - 2] {
    for steps in [66 * 1] {
        let p = perfect(steps);
        let mut solution = Qubic::new(map.clone(), steps);
        let c = solution.solve();
        solution.print();

        println!(
            "# steps: {}, correct: {}, diff w perfect: {}, next: {}, next next: {}",
            steps,
            c,
            p - c,
            c * 9,
            c * 25
        );
    }
}

fn main() {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)
        .unwrap()
        .join()
        .unwrap();
}

#[rstest]
#[trace]
#[case(6, 16)]
#[case(10, 50)]
#[case(50, 1594)]
#[case(100, 6536)]
fn test_qubic(#[case] steps: usize, #[case] ans: usize) {
    let mut solution = Qubic::new(str_to_map(include_str!("small.in")), steps);
    assert_eq!(solution.solve(), ans);
}
