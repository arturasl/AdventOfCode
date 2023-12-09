use itertools::sorted;
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, BufRead};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Priority {
    High,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Copy)]
enum Card {
    Jack,
    Num(i64),
    Queen,
    King,
    Ace,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut hands: Vec<(Vec<Card>, i64)> = vec![];
    for maybe_line in io::stdin().lock().lines() {
        let parts: Vec<String> = maybe_line?
            .trim()
            .split(' ')
            .map(|x| x.to_string())
            .collect();
        hands.push((
            parts[0]
                .chars()
                .map(|c| match c {
                    'A' => Card::Ace,
                    'K' => Card::King,
                    'Q' => Card::Queen,
                    'J' => Card::Jack,
                    'T' => Card::Num(10),
                    x => Card::Num((x as i64) - ('0' as i64)),
                })
                .collect(),
            parts[1].parse()?,
        ));
    }

    println!("{:?}", hands);

    let mut counts: Vec<(Priority, Vec<Card>, i64)> = hands
        .into_iter()
        .map(|x| {
            let mut map: HashMap<Card, usize> = HashMap::new();
            for c in &x.0 {
                map.entry(*c).and_modify(|y| *y += 1).or_insert(1);
            }

            if let Some(jaks) = map.get(&Card::Jack).map(|x| x.clone()) {
                map.remove(&Card::Jack);
                if map.is_empty() {
                    map.insert(Card::Ace, 5);
                } else {
                    let best: (Card, usize) = map
                        .iter()
                        .max_by_key(|x| x.1)
                        .map(|x| (*x.0, *x.1))
                        .unwrap();
                    map.entry(best.0).and_modify(|x| *x += jaks);
                }
            }

            let histogram: Vec<usize> = sorted(map.into_values()).rev().collect();
            let priority = match histogram.as_slice() {
                [5] => Priority::FiveOfKind,
                [4, _] => Priority::FourOfKind,
                [3, 2] => Priority::FullHouse,
                [3, ..] => Priority::ThreeOfKind,
                [2, 2, 1] => Priority::TwoPair,
                [2, 1, 1, 1] => Priority::OnePair,
                [1, 1, 1, 1, 1] => Priority::High,
                _ => panic!(),
            };
            (priority, x.0, x.1)
        })
        .collect();
    counts.sort();

    println!("{:?}", counts);

    let mut result: i64 = 0;
    for (pos, val) in counts.iter().enumerate() {
        result += ((pos as i64) + 1) * val.2;
    }

    println!("Result: {}", result);

    Ok(())
}
