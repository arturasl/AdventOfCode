use std::io::{self, BufRead};

fn main() {
    let mut sum: i32 = 0;
    for maybe_line in io::stdin().lock().lines() {
        let line: String = maybe_line.unwrap().trim().chars().collect();
        let search: Vec<(u8, &str)> = vec![
            (1, "one"),
            (2, "two"),
            (3, "three"),
            (4, "four"),
            (5, "five"),
            (6, "six"),
            (7, "seven"),
            (8, "eight"),
            (9, "nine"),
        ];

        let mut found: Vec<(usize, u8)> = vec![];

        for (num, num_name) in search {
            found.extend(
                line.match_indices(num_name)
                    .map(|(i, _)| (i, num))
                    .collect::<Vec<(usize, u8)>>(),
            );
            found.extend(
                line.match_indices(&format!("{}", num))
                    .map(|(i, _)| (i, num))
                    .collect::<Vec<(usize, u8)>>(),
            );
        }

        let val = format!(
            "{}{}",
            found.iter().min().unwrap().1,
            found.iter().max().unwrap().1
        );
        println!("{}", val);
        let num: i32 = val.parse().unwrap();
        sum += num;
    }
    println!("{}", sum);
}
