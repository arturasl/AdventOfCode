use std::io;
use std::thread;

fn walk(pos: (usize, usize), prev_pos: (usize, usize), map: &Vec<Vec<char>>) -> Option<usize> {
    if map[pos.0][pos.1] == '#' {
        return None;
    }

    if pos == (map.len() - 1, map[0].len() - 2) {
        return Some(1);
    }

    let cur = map[pos.0][pos.1];

    let mut result: Option<usize> = None;
    for (name, oft) in [('^', (-1, 0)), ('>', (0, 1)), ('v', (1, 0)), ('<', (0, -1))] {
        if name != cur && cur != '.' {
            continue;
        }

        let next_pos = (
            ((pos.0 as i64) + oft.0) as usize,
            ((pos.1 as i64) + oft.1) as usize,
        );
        if next_pos == prev_pos {
            continue;
        }
        let w = walk(next_pos, pos, map).map(|r| r + 1).or(result);
        if result.is_none() {
            result = w;
        } else {
            result = result.zip(w).map(|(a, b)| a.max(b));
        }
    }

    result
}

fn run() {
    let map: Vec<Vec<char>> = io::stdin()
        .lines()
        .map(|x| x.unwrap().trim().to_string())
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    assert!(map[0][0..3] == ['#', '.', '#'] && map[1][1] == '.');
    assert!(
        map[map.len() - 1][map[0].len() - 3..] == ['#', '.', '#']
            && map[map.len() - 2][map[0].len() - 2] == '.'
    );

    println!("{:?}", walk((1, 1), (0, 1), &map));
}

fn main() {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)
        .unwrap()
        .join()
        .unwrap();
}
