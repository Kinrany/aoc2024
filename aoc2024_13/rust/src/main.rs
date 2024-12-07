use std::io::{BufRead, BufReader};

use anyhow::Context;

type Input = Vec<(u64, Vec<u64>)>;

fn input(input: impl BufRead) -> anyhow::Result<Input> {
    input
        .lines()
        .map(|result| {
            let line = result?;
            let (a, b) = line.split_once(": ").context("invalid format")?;
            Ok((
                a.parse()?,
                b.split(' ')
                    .map(str::parse::<u64>)
                    .collect::<Result<_, _>>()?,
            ))
        })
        .collect()
}

fn solution(input: Input) -> u64 {
    // Individual entries are all pretty short, shorter than 10 items,
    // so we can simply try all ~2^10 options for each one.
    input
        .into_iter()
        .filter(|(test_value, values)| {
            let test_value = *test_value;
            let first = values[0];
            let rest = &values[1..];

            // 1 if addition, 0 if multiplication
            let mut bitmask = 1 << (rest.len() + 1);
            while bitmask > 0 {
                bitmask -= 1;

                let value = rest.iter().enumerate().fold(first, |acc, (idx, x)| {
                    if bitmask & (1 << idx) == 0 {
                        acc * x
                    } else {
                        acc + x
                    }
                });

                if value == test_value {
                    return true;
                }
            }

            false
        })
        .map(|(test_value, _)| test_value)
        .sum()
}

fn main() -> anyhow::Result<()> {
    let input = input(BufReader::new(std::io::stdin()))?;
    let answer = solution(input);
    println!("{answer}");
    Ok(())
}

#[test]
fn parse_example() {
    let bytes = include_bytes!("../../example.txt");
    insta::assert_debug_snapshot!(input(bytes.as_slice()).unwrap());
}

#[test]
fn solve_example() {
    let input: Input = vec![
        (190, vec![10, 19]),
        (3267, vec![81, 40, 27]),
        (83, vec![17, 5]),
        (156, vec![15, 6]),
        (7290, vec![6, 8, 6, 15]),
        (161011, vec![16, 10, 13]),
        (192, vec![17, 8, 14]),
        (21037, vec![9, 7, 18, 13]),
        (292, vec![11, 6, 16, 20]),
    ];
    assert_eq!(solution(input), 3749);
}
