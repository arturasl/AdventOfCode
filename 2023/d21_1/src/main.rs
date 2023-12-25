use std::io;
use std::thread;

const STEPS: usize = 64;

fn expand(map: Vec<Vec<char>>, default: char) -> Vec<Vec<char>> {
    let cols: usize = map[0].len();

    vec![vec![default; cols + 2]]
        .into_iter()
        .chain(map.into_iter().map(|r| {
            vec![default]
                .into_iter()
                .chain(r.into_iter())
                .chain(vec![default])
                .collect::<Vec<char>>()
        }))
        .chain(vec![vec![default; cols + 2]])
        .collect::<Vec<Vec<char>>>()
}

fn visit(
    map: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<Vec<bool>>>,
    steps: usize,
    pos: (usize, usize),
) {
    if pos.0 == 0
        || pos.0 == map.len() - 1
        || pos.1 == 0
        || pos.1 == map[0].len() - 1
        || steps == STEPS + 1
    {
        return;
    }
    if map[pos.0][pos.1] == '#' {
        return;
    }
    if visited[steps][pos.0][pos.1] {
        return;
    }
    visited[steps][pos.0][pos.1] = true;

    for dir in [(0, 1), (2, 1), (1, 0), (1, 2)] {
        let next_pos = (pos.0 + dir.0 - 1, pos.1 + dir.1 - 1);
        visit(map, visited, steps + 1, next_pos);
    }
}

fn run() {
    let map: Vec<Vec<char>> = expand(
        io::stdin()
            .lines()
            .map(|x| x.unwrap().trim().to_string())
            .filter(|line| !line.is_empty())
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect(),
        '?',
    );
    let mut visited: Vec<Vec<Vec<bool>>> =
        vec![vec![vec![false; map[0].len()]; map.len()]; STEPS + 1];

    let mut s_pos: (usize, usize) = (0, 0);
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if map[row][col] == 'S' {
                s_pos = (row, col);
            }
        }
    }

    visit(&map, &mut visited, 0, s_pos);

    println!(
        "{}",
        visited[STEPS]
            .iter()
            .map(|row| row.iter().map(|c| *c as usize).sum::<usize>())
            .sum::<usize>()
    );
}

fn main() {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)
        .unwrap()
        .join()
        .unwrap();
}
