use std::io::{BufRead, BufReader};

use anyhow::Context;

type Input = (Vec<(i32, i32)>, Vec<Vec<i32>>);

fn input(mut input: impl BufRead) -> anyhow::Result<Input> {
    let mut s = String::new();
    input.read_to_string(&mut s)?;
    let (first, second) = s.split_once("\n\n").context("bad input")?;

    let first: Vec<(i32, i32)> = first
        .split('\n')
        .map(|s| {
            let (a, b) = s.split_once('|').context("bad pair")?;
            Ok((a.parse()?, b.parse()?))
        })
        .collect::<anyhow::Result<_>>()?;

    let second: Vec<Vec<i32>> = second
        .lines()
        .map(|s| {
            s.split(',')
                .map(|s| s.parse().context("not a number"))
                .collect()
        })
        .collect::<anyhow::Result<_>>()?;

    Ok((first, second))
}

fn solution((rules, updates): Input) -> i32 {
    updates
        .into_iter()
        .filter(|update| {
            rules.iter().all(|(a, b)| {
                let mut found_second = false;
                for x in update {
                    if x == a {
                        return !found_second;
                    } else if x == b {
                        found_second = true;
                    }
                }
                true
            })
        })
        .map(|update| update[update.len() / 2])
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
    let input: Input = (
        vec![
            (47, 53),
            (97, 13),
            (97, 61),
            (97, 47),
            (75, 29),
            (61, 13),
            (75, 53),
            (29, 13),
            (97, 29),
            (53, 29),
            (61, 53),
            (97, 53),
            (61, 29),
            (47, 13),
            (75, 47),
            (97, 75),
            (47, 61),
            (75, 61),
            (47, 29),
            (75, 13),
            (53, 13),
        ],
        vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ],
    );
    assert_eq!(solution(input), 143);
}
