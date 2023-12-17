use std::io;
use std::thread;

#[derive(PartialEq, Eq, Copy, Clone)]
enum Dir {
    NORTH = 0,
    EAST = 1,
    SOUTH = 2,
    WEST = 3,
}

fn walk_dir(dir: &Dir, pos: (usize, usize), map: &mut Vec<Vec<Vec<char>>>) {
    let next_pos: (usize, usize) = match dir {
        Dir::NORTH => (pos.0 - 1, pos.1),
        Dir::EAST => (pos.0, pos.1 + 1),
        Dir::SOUTH => (pos.0 + 1, pos.1),
        Dir::WEST => (pos.0, pos.1 - 1),
    };
    dfs(dir, next_pos, map);
}

fn dfs(dir: &Dir, pos: (usize, usize), map: &mut Vec<Vec<Vec<char>>>) {
    if map[*dir as usize][pos.0][pos.1] == '#'
        || pos.0 == 0
        || pos.1 == 0
        || pos.0 == map[*dir as usize].len() - 1
        || pos.1 == map[*dir as usize][pos.0].len() - 1
    {
        return;
    }
    let prev = map[*dir as usize][pos.0][pos.1];
    map[*dir as usize][pos.0][pos.1] = '#';

    match prev {
        '.' => walk_dir(dir, pos, map),
        '|' => {
            if [Dir::NORTH, Dir::SOUTH].contains(&dir) {
                walk_dir(dir, pos, map);
            } else {
                walk_dir(&Dir::NORTH, pos, map);
                walk_dir(&Dir::SOUTH, pos, map);
            }
        }
        '-' => {
            if [Dir::EAST, Dir::WEST].contains(&dir) {
                walk_dir(dir, pos, map);
            } else {
                walk_dir(&Dir::EAST, pos, map);
                walk_dir(&Dir::WEST, pos, map);
            }
        }
        '/' => match dir {
            Dir::NORTH => walk_dir(&Dir::EAST, pos, map),
            Dir::EAST => walk_dir(&Dir::NORTH, pos, map),
            Dir::SOUTH => walk_dir(&Dir::WEST, pos, map),
            Dir::WEST => walk_dir(&Dir::SOUTH, pos, map),
        },
        '\\' => match dir {
            Dir::NORTH => walk_dir(&Dir::WEST, pos, map),
            Dir::EAST => walk_dir(&Dir::SOUTH, pos, map),
            Dir::SOUTH => walk_dir(&Dir::EAST, pos, map),
            Dir::WEST => walk_dir(&Dir::NORTH, pos, map),
        },
        _ => panic!(),
    }
}

fn find_result(dir: &Dir, pos: (usize, usize), map: &Vec<Vec<char>>) -> usize {
    let mut map_w_dir: Vec<Vec<Vec<char>>> = vec![map.clone(); 4];
    dfs(dir, pos, &mut map_w_dir);
    let mut result: usize = 0;
    for row in 0..map_w_dir[0].len() {
        for col in 0..map_w_dir[0][0].len() {
            result += (0..4)
                .into_iter()
                .any(|dir| map_w_dir[dir][row][col] == '#') as usize;
        }
    }
    result
}

fn run() {
    let mut map: Vec<Vec<char>> = io::stdin()
        .lines()
        .map(|x| {
            ".".chars()
                .chain(x.unwrap().trim().chars())
                .chain(".".chars())
                .collect::<Vec<char>>()
        })
        .filter(|x| !x.is_empty())
        .collect();
    let empty_row: Vec<char> = ".".chars().cycle().take(map[0].len()).collect();
    map.insert(0, empty_row.clone());
    map.push(empty_row);

    let mut result: usize = 0;
    for row in 1..map.len() - 1 {
        result = result.max(find_result(&Dir::EAST, (row, 1), &map));
        result = result.max(find_result(&Dir::WEST, (row, map[0].len() - 2), &map));
    }
    for col in 1..map[0].len() - 1 {
        result = result.max(find_result(&Dir::SOUTH, (1, col), &map));
        result = result.max(find_result(&Dir::NORTH, (map.len() - 2, col), &map));
    }

    println!("Result: {result}");
}

fn main() {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)
        .unwrap()
        .join()
        .unwrap();
}
