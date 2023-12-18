use regex::Regex;
use std::io;
use std::thread;

fn moving_dir(dir: char) -> (i64, i64) {
    match dir {
        'U' => (-1, 0),
        'R' => (0, 1),
        'D' => (1, 0),
        'L' => (0, -1),
        _ => panic!(),
    }
}

fn walk(mut pos: (i64, i64), dir: char, steps: i64) -> ((i64, i64), (i64, i64)) {
    let dir: (i64, i64) = moving_dir(dir);
    pos.0 += dir.0 * steps;
    pos.1 += dir.1 * steps;
    (dir, pos)
}

fn draw(pos: (i64, i64), walkable: &str, fill: char, map: &mut Vec<Vec<char>>) -> usize {
    if !walkable.contains(map[pos.0 as usize][pos.1 as usize]) {
        return 0;
    }
    if pos.0 == 0
        || pos.1 == 0
        || pos.0 as usize == map.len() - 1
        || pos.1 as usize == map[0].len() - 1
    {
        return 0;
    }
    let mut result: usize = 1;
    map[pos.0 as usize][pos.1 as usize] = fill;
    for dir in "URDL".chars() {
        let m_dir: (i64, i64) = moving_dir(dir);
        let next_pos: (i64, i64) = (pos.0 + m_dir.0, pos.1 + m_dir.1);
        result += draw(next_pos, walkable, fill, map);
    }
    result
}

fn print(map: &Vec<Vec<char>>) {
    for row in map {
        println!("{}", row.iter().collect::<String>());
    }
    println!("");
}

fn run() {
    let re_direction =
        Regex::new(r"^(?<dir>[URDL])\s+(?<steps>\d+)\s+(?<color>\(#.{6}\))$").unwrap();
    let instructions: Vec<(char, i64)> = io::stdin()
        .lines()
        .map(|x| x.unwrap().trim().to_string())
        .filter(|x| !x.is_empty())
        .map(|x| {
            let captures = re_direction.captures(&x).unwrap();
            (
                captures["dir"].chars().next().unwrap(),
                captures["steps"].parse::<i64>().unwrap(),
            )
        })
        .collect();

    let mut pos: (i64, i64) = (0, 0);
    let mut tl: (i64, i64) = (0, 0);
    let mut br: (i64, i64) = (0, 0);
    for instruction in &instructions {
        pos = walk(pos, instruction.0, instruction.1).1;
        tl = (tl.0.min(pos.0), tl.1.min(pos.1));
        br = (br.0.max(pos.0), br.1.max(pos.1));
    }
    tl.0 -= 2;
    tl.1 -= 2;
    br.0 += 2;
    br.1 += 2;

    pos = (-tl.0, -tl.1);
    br.0 -= tl.0 - 1;
    br.1 -= tl.1 - 1;
    let mut map: Vec<Vec<char>> = vec![vec!['.'; br.1 as usize]; br.0 as usize];
    for instruction in &instructions {
        let (dir, next_pos) = walk(pos, instruction.0, instruction.1);
        while pos != next_pos {
            map[pos.0 as usize][pos.1 as usize] = '#';
            pos.0 += dir.0;
            pos.1 += dir.1;
        }
    }

    print(&map);
    draw((1, 1), ".", '@', &mut map);
    print(&map);

    let mut result: usize = 0;
    for i in 1..map.len() - 1 {
        for j in 1..map[0].len() - 1 {
            result += draw((i as i64, j as i64), "#.", 'w', &mut map);
        }
    }

    print(&map);
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
