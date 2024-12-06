use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};

use anyhow::Context;

type Input = Vec<(i32, i32)>;

fn input(input: impl BufRead) -> anyhow::Result<Input> {
    input
        .lines()
        .map(|result| {
            let s = result.context("failed to read line")?;
            let (a, b) = s.split_once("   ").context("invalid line")?;
            Ok((a.parse()?, b.parse()?))
        })
        .collect()
}

fn solution(input: Input) -> i32 {
    let (list1, list2): (Vec<_>, Vec<_>) = input.into_iter().unzip();
    let counts = list2.into_iter().fold(HashMap::new(), |mut acc, x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    });
    list1
        .into_iter()
        .map(|x| x * counts.get(&x).unwrap_or(&0))
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
    let input: Input = vec![(3, 4), (4, 3), (2, 5), (1, 3), (3, 9), (3, 3)];
    assert_eq!(solution(input), 31);
}
