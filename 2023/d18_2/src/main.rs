use gcollections::ops::*;
use interval::interval_set::*;
use regex::Regex;
use std::collections::BTreeMap;
use std::io;
use std::thread;

fn walk(pos: (i64, i64), dir: char, steps: i64) -> (bool, (i64, i64)) {
    let dir: (i64, i64) = match dir {
        'U' => (-1, 0),
        'R' => (0, 1),
        'D' => (1, 0),
        'L' => (0, -1),
        _ => panic!(),
    };
    (dir.1 != 0, (pos.0 + dir.0 * steps, pos.1 + dir.1 * steps))
}

fn run() {
    let re_direction =
        Regex::new(r"^(?:[URDL])\s+(?:\d+)\s+\(#(?<steps_hex>.{5})(?<dir>.)\)$").unwrap();
    let instructions: Vec<(char, i64)> = io::stdin()
        .lines()
        .map(|x| x.unwrap().trim().to_string())
        .filter(|x| !x.is_empty())
        .map(|x| {
            let captures = re_direction.captures(&x).unwrap();
            (
                match &captures["dir"] {
                    "0" => 'R',
                    "1" => 'D',
                    "2" => 'L',
                    "3" => 'U',
                    _ => panic!(),
                },
                i64::from_str_radix(&captures["steps_hex"], 16).unwrap(),
            )
        })
        .collect();

    let mut pos: (i64, i64) = (0, 0);
    let mut map: BTreeMap<i64, IntervalSet<i64>> = BTreeMap::new();
    for instruction in &instructions {
        let (is_horizontal, next_pos) = walk(pos, instruction.0, instruction.1);
        if is_horizontal {
            let val = map.entry(pos.0).or_insert(IntervalSet::empty());
            *val =
                val.union(&vec![(pos.1.min(next_pos.1), pos.1.max(next_pos.1))].to_interval_set());
        }
        pos = next_pos;
    }

    let mut area: i64 = 0;
    let mut it = map.into_iter();
    let (mut cur_y, mut cur_segments) = it.next().unwrap();
    for (other_y, other_segments) in it {
        println!("###### ");
        println!("  Cur {:?} {:?}", cur_y, cur_segments);
        println!("  Other {:?} {:?}", other_y, other_segments);
        let mut next_segments = cur_segments.symmetric_difference(&other_segments);
        for segment in other_segments {
            if cur_segments.contains(&(segment.lower() - 1)) {
                next_segments = next_segments.union(&IntervalSet::singleton(segment.lower()));
            }
            if cur_segments.contains(&(segment.upper() + 1)) {
                next_segments = next_segments.union(&IntervalSet::singleton(segment.upper()));
            }
        }

        let local_area = (cur_segments.size() as i64) * (other_y - cur_y)
            + (cur_segments.difference(&next_segments).size() as i64);
        println!("  Local area: {:?}", local_area);
        area += local_area;

        cur_segments = next_segments;
        cur_y = other_y;
    }

    println!("Area: {area}");
}

fn main() {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)
        .unwrap()
        .join()
        .unwrap();
}
