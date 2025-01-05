use anyhow::{bail, ensure, Ok, Result};
use rstest::rstest;
use std::io::{self, BufRead};
use std::thread;

#[derive(Debug, Clone)]
enum El {
    L(i64),
    Open,
    Close,
}

fn parse(str: &[char]) -> Result<Vec<El>> {
    let mut result: Vec<El> = Vec::with_capacity(str.len());
    let mut idx: usize = 0;
    while idx != str.len() {
        match str[idx] {
            '[' => {
                result.push(El::Open);
                idx += 1;
            }
            ']' => {
                result.push(El::Close);
                idx += 1;
            }
            ',' => {
                idx += 1;
            }
            ch => {
                ensure!(ch.is_ascii_digit());
                let mut str_num = String::new();
                while idx < str.len() && str[idx].is_ascii_digit() {
                    str_num.push(str[idx]);
                    idx += 1;
                }
                result.push(El::L(str_num.parse()?));
            }
        }
    }
    Ok(result)
}

fn to_str(parsed: &[El]) -> String {
    let mut result = String::new();
    for idx in 0..parsed.len() {
        match parsed[idx] {
            El::L(n) => {
                result.push_str(&n.to_string());
                if idx + 1 < parsed.len() {
                    if let El::Open = parsed[idx + 1] {
                        result.push(',');
                    }
                    if let El::L(_) = parsed[idx + 1] {
                        result.push(',');
                    }
                }
            }
            El::Open => result.push('['),
            El::Close => {
                result.push(']');
                if idx + 1 < parsed.len() {
                    if let El::Open = parsed[idx + 1] {
                        result.push(',');
                    }
                    if let El::L(_) = parsed[idx + 1] {
                        result.push(',');
                    }
                }
            }
        }
    }

    result
}

fn split(parsed: &mut Vec<El>) -> bool {
    for idx in 0..parsed.len() {
        if let El::L(n) = parsed[idx] {
            if n >= 10 {
                parsed.splice(
                    idx..idx + 1,
                    [El::Open, El::L(n / 2), El::L(n / 2 + n % 2), El::Close],
                );
                return true;
            }
        }
    }

    false
}

fn explode(parsed: &mut Vec<El>) -> Result<bool> {
    let mut depth = 0;
    let mut idx = 0;

    while idx < parsed.len() {
        match parsed[idx] {
            El::L(lhs)
                if depth >= 5 && idx + 1 < parsed.len() && matches!(parsed[idx + 1], El::L(_)) =>
            {
                let El::L(rhs) = parsed[idx + 1] else {
                    bail!("");
                };

                ensure!(matches!(parsed[idx - 1], El::Open));
                ensure!(matches!(parsed[idx + 2], El::Close));
                parsed.splice(idx - 1..idx + 3, [El::L(0)]);

                for lhs_idx in (0..idx - 1).rev() {
                    if let El::L(prev) = parsed[lhs_idx] {
                        parsed[lhs_idx] = El::L(lhs + prev);
                        break;
                    }
                }
                for rhs_el in parsed.iter_mut().skip(idx) {
                    if let El::L(prev) = rhs_el {
                        *rhs_el = El::L(rhs + *prev);
                        break;
                    }
                }

                return Ok(true);
            }
            El::Open => depth += 1,
            El::Close => depth -= 1,
            _ => {}
        }
        idx += 1
    }

    ensure!(depth == 0);

    Ok(false)
}

fn apply_all(parsed: &mut Vec<El>) -> Result<()> {
    while explode(parsed)? || split(parsed) {}
    Ok(())
}

fn cal_magnituted_int(parsed: &[El], idx: &mut usize) -> Result<i64> {
    if *idx == parsed.len() {
        return Ok(0);
    }
    if let El::L(n) = parsed[*idx] {
        *idx += 1;
        return Ok(n);
    }

    ensure!(matches!(parsed[*idx], El::Open));
    *idx += 1;
    let lhs = cal_magnituted_int(parsed, idx)?;
    let rhs = cal_magnituted_int(parsed, idx)?;
    ensure!(matches!(parsed[*idx], El::Close));
    *idx += 1;

    Ok(3 * lhs + 2 * rhs)
}

fn cal_magnituted(parsed: &[El]) -> Result<i64> {
    let mut idx = 0;
    cal_magnituted_int(parsed, &mut idx)
}

fn run() -> Result<()> {
    let mut numbers: Vec<Vec<El>> = Vec::new();
    for maybe_line in io::stdin().lock().lines() {
        let line: Vec<char> = maybe_line?.replace(" ", "").chars().collect();
        if line.is_empty() {
            continue;
        }

        numbers.push(parse(&line)?);
    }

    let mut max_num: Vec<El> = Vec::new();
    let mut max_mag = 0;
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i == j {
                continue;
            }

            let mut num: Vec<El> = Vec::with_capacity(numbers[i].len() + numbers[j].len() + 100);
            num.push(El::Open);
            num.extend(numbers[i].clone());
            num.extend(numbers[j].clone());
            num.push(El::Close);

            apply_all(&mut num)?;
            let mag = cal_magnituted(&num)?;
            if max_mag < mag {
                max_mag = mag;
                max_num = num;
            }
        }
    }

    println!("{}", to_str(&max_num));
    println!("{}", max_mag);

    Ok(())
}

fn main() -> Result<()> {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)?
        .join()
        .unwrap()
}

#[rstest]
#[case("", "")]
#[case("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]")]
#[case("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]")]
#[case("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]")]
#[case(
    "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
    "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"
)]
#[case("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[7,0]]]]")]
fn explode_test(#[case] inp: &str, #[case] exp: &str) {
    let mut parsed = parse(&inp.chars().collect::<Vec<char>>()).unwrap();
    let _ = explode(&mut parsed);
    assert_eq!(exp, to_str(&parsed));
}

#[rstest]
#[case("", "")]
#[case(
    "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]",
    "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
)]
#[case(
    "[[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]",
    "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"
)]
#[case(
    "[[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]],[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]]",
    "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]"
)]
#[case(
    "[[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]],[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]]",
    "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]"
)]
#[case(
    "[[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]],[7,[5,[[3,8],[1,4]]]]]",
    "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]"
)]
#[case(
    "[[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]],[[2,[2,2]],[8,[8,1]]]]",
    "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]"
)]
#[case(
    "[[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]],[2,9]]",
    "[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]"
)]
#[case(
    "[[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]],[1,[[[9,3],9],[[9,0],[0,7]]]]]",
    "[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]"
)]
#[case(
    "[[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]],[[[5,[7,4]],7],1]]",
    "[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]"
)]
#[case(
    "[[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]],[[[[4,2],2],6],[8,7]]]",
    "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
)]
fn apply_all_test(#[case] inp: &str, #[case] exp: &str) {
    let mut parsed = parse(&inp.chars().collect::<Vec<char>>()).unwrap();
    apply_all(&mut parsed).unwrap();
    assert_eq!(exp, to_str(&parsed));
}

#[rstest]
#[case("", 0)]
#[case("[9,1]", 29)]
#[case("[1,9]", 21)]
#[case("[[9,1],[1,9]]", 129)]
#[case("[[1,2],[[3,4],5]]", 143)]
#[case("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384)]
#[case("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445)]
#[case("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791)]
#[case("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137)]
#[case("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]", 3488)]
fn cal_magnituted_test(#[case] inp: &str, #[case] exp: i64) {
    let parsed = parse(&inp.chars().collect::<Vec<char>>()).unwrap();
    let result = cal_magnituted(&parsed).unwrap();
    assert_eq!(exp, result);
}
