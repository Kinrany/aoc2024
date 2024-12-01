use std::io::{BufRead, BufReader};

use anyhow::Context;

fn input(input: impl BufRead) -> anyhow::Result<Vec<(i32, i32)>> {
    input
        .lines()
        .map(|result| {
            let s = result.context("failed to read line")?;
            let (a, b) = s.split_once("   ").context("invalid line")?;
            Ok((a.parse()?, b.parse()?))
        })
        .collect()
}

fn solution(input: Vec<(i32, i32)>) -> i32 {
    let (mut list1, mut list2): (Vec<_>, Vec<_>) = input.into_iter().unzip();
    list1.sort_unstable();
    list2.sort_unstable();
    list1
        .into_iter()
        .zip(list2)
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn main() -> anyhow::Result<()> {
    let answer = solution(input(BufReader::new(std::io::stdin()))?);
    println!("{answer}");
    Ok(())
}

#[test]
fn parse_input() {
    let txt = include_bytes!("../../example.txt");
    let input = input(txt.as_slice()).unwrap();
    insta::assert_debug_snapshot!(input);
}

#[test]
fn solve_example() {
    let input = vec![(3, 4), (4, 3), (2, 5), (1, 3), (3, 9), (3, 3)];
    assert_eq!(solution(input), 11);
}
