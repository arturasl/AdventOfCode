use std::error::Error;
use std::io::{self, BufRead};
use std::thread;

fn dfs(pos: (usize, usize), map: &mut Vec<Vec<char>>) {
    map[pos.0][pos.1] = 'v';
    let mut result: usize = 1;

    for y_oft in -1i64..1 + 1 {
        for x_oft in -1i64..1 + 1 {
            if y_oft != 0 && x_oft != 0 {
                continue;
            }
            if (y_oft == -1 && pos.0 == 0) || (x_oft == -1 && pos.1 == 0) {
                continue;
            }
            let new_pos: (usize, usize) = (
                ((pos.0 as i64) + y_oft) as usize,
                ((pos.1 as i64) + x_oft) as usize,
            );
            if new_pos.0 >= map.len() || new_pos.1 >= map[pos.0].len() {
                continue;
            }
            if map[new_pos.0][new_pos.1] != '#' {
                continue;
            }
            dfs(new_pos, map);
        }
    }
}

fn fill(pos: (usize, usize), map: &mut Vec<Vec<char>>, from: char, to: char) -> bool {
    if map[pos.0][pos.1] != from {
        return false;
    }
    map[pos.0][pos.1] = to;
    let mut result: bool = false;

    for y_oft in -1i64..1 + 1 {
        for x_oft in -1i64..1 + 1 {
            if y_oft != 0 && x_oft != 0 {
                continue;
            }
            if (y_oft == -1 && pos.0 == 0) || (x_oft == -1 && pos.1 == 0) {
                result = true;
                continue;
            }
            let new_pos: (usize, usize) = (
                ((pos.0 as i64) + y_oft) as usize,
                ((pos.1 as i64) + x_oft) as usize,
            );
            if new_pos.0 >= map.len() || new_pos.1 >= map[pos.0].len() {
                result = true;
                continue;
            }
            if fill(new_pos, map, from, to) {
                result = true;
            }
        }
    }

    result
}

fn show_map(map: &Vec<Vec<char>>, title: &str) {
    println!("# {}", title);
    for r in map {
        for x in r {
            print!("{}", x);
        }
        println!("");
    }
    println!("");
}

fn run() {
    let lines: Vec<Vec<char>> = io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap().trim().chars().collect::<Vec<char>>())
        .collect();

    show_map(&lines, "Read");

    let mut map: Vec<Vec<char>> = vec![vec!['.'; lines[0].len() * 3]; lines.len() * 3];
    let mut s_pos: (usize, usize) = (0, 0);

    for row in 0..lines.len() {
        for col in 0..lines[row].len() {
            if lines[row][col] != '.' {
                map[row * 3 + 1][col * 3 + 1] = '#';
            }

            match lines[row][col] {
                '|' => {
                    map[row * 3 + 0][col * 3 + 1] = '#';
                    map[row * 3 + 2][col * 3 + 1] = '#';
                }
                '-' => {
                    map[row * 3 + 1][col * 3 + 0] = '#';
                    map[row * 3 + 1][col * 3 + 2] = '#';
                }
                'L' => {
                    map[row * 3 + 0][col * 3 + 1] = '#';
                    map[row * 3 + 1][col * 3 + 2] = '#';
                }
                'J' => {
                    map[row * 3 + 0][col * 3 + 1] = '#';
                    map[row * 3 + 1][col * 3 + 0] = '#';
                }
                '7' => {
                    map[row * 3 + 1][col * 3 + 0] = '#';
                    map[row * 3 + 2][col * 3 + 1] = '#';
                }
                'F' => {
                    map[row * 3 + 1][col * 3 + 2] = '#';
                    map[row * 3 + 2][col * 3 + 1] = '#';
                }
                '.' => {}
                'S' => {
                    s_pos = (row, col);
                }
                _ => panic!(""),
            };
        }
    }

    show_map(&map, "After translating");

    if s_pos.1 * 3 + 3 < map[s_pos.0 * 3 + 1].len() {
        map[s_pos.0 * 3 + 1][s_pos.1 * 3 + 2] = map[s_pos.0 * 3 + 1][s_pos.1 * 3 + 3];
    }
    if s_pos.1 * 3 >= 1 {
        map[s_pos.0 * 3 + 1][s_pos.1 * 3 + 0] = map[s_pos.0 * 3 + 1][s_pos.1 * 3 - 1];
    }
    if s_pos.0 * 3 >= 1 {
        map[s_pos.0 * 3 + 0][s_pos.1 * 3 + 1] = map[s_pos.0 * 3 - 1][s_pos.1 * 3 + 1];
    }
    if s_pos.0 * 3 + 3 < map.len() {
        map[s_pos.0 * 3 + 2][s_pos.1 * 3 + 1] = map[s_pos.0 * 3 + 3][s_pos.1 * 3 + 1];
    }

    show_map(&map, "After filling S");

    dfs((s_pos.0 * 3 + 1, s_pos.1 * 3 + 1), &mut map);

    show_map(&map, "After dfs");

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] != 'v' {
                map[i][j] = '.';
            }
        }
    }

    show_map(&map, "After cleaning");

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if fill((i, j), &mut map, '.', 'I') {
                fill((i, j), &mut map, 'I', 'O');
            }
        }
    }

    show_map(&map, "After interior");

    let mut truely_empty: i64 = 0;
    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            let mut all_empty: bool = true;
            for ii in 0..3 {
                for jj in 0..3 {
                    all_empty = all_empty && map[i * 3 + ii][j * 3 + jj] == 'I';
                }
            }
            if !all_empty {
                continue;
            }

            truely_empty += 1;
            for ii in 0..3 {
                for jj in 0..3 {
                    map[i * 3 + ii][j * 3 + jj] = 'X';
                }
            }
        }
    }

    show_map(&map, "After finding truely empty");

    println!("Empty: {}", truely_empty);
}

fn main() {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)
        .unwrap()
        .join()
        .unwrap();
}
