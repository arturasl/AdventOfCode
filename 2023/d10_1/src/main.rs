use std::error::Error;
use std::io::{self, BufRead};
use std::thread;

fn dfs(pos: (usize, usize), map: &mut Vec<Vec<char>>) -> usize {
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
            result += dfs(new_pos, map);
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
                    map[row * 3 + 0][col * 3 + 1] = '?';
                    map[row * 3 + 1][col * 3 + 0] = '?';
                    map[row * 3 + 1][col * 3 + 2] = '?';
                    map[row * 3 + 2][col * 3 + 1] = '?';
                }
                _ => panic!(""),
            };
        }
    }

    show_map(&map, "Read");

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

    let mut result: usize = dfs((s_pos.0 * 3 + 1, s_pos.1 * 3 + 1), &mut map);
    assert!(result % 3 == 0);
    result /= 3;
    assert!(result % 2 == 0);
    result /= 2;

    show_map(&map, "After dfs");

    println!("Result: {}", result);
}

fn main() {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)
        .unwrap()
        .join()
        .unwrap();
}
