use std::io::{BufRead, BufReader};

use anyhow::Context;
use itertools::Itertools;

type Input = Vec<Vec<i32>>;

fn input(input: impl BufRead) -> anyhow::Result<Input> {
    input
        .lines()
        .map(|result| {
            let s = result.context("failed to read line")?;
            let numbers = s.split(' ').map(|x| x.parse()).collect::<Result<_, _>>()?;
            Ok(numbers)
        })
        .collect()
}

fn safe(report: &[i32]) -> bool {
    let gradual = report
        .iter()
        .tuple_windows()
        .all(|(a, b)| (1..=3).contains(&(a - b).abs()));

    let increasing = report.iter().tuple_windows().all(|(a, b)| a < b);
    let decreasing = report.iter().tuple_windows().all(|(a, b)| a > b);

    gradual && (increasing || decreasing)
}

fn solution(input: Input) -> usize {
    input.into_iter().filter(|report| safe(report)).count()
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
    let input: Input = vec![
        vec![7, 6, 4, 2, 1],
        vec![1, 2, 7, 8, 9],
        vec![9, 7, 6, 2, 1],
        vec![1, 3, 2, 4, 5],
        vec![8, 6, 4, 4, 1],
        vec![1, 3, 6, 7, 9],
    ];
    assert_eq!(solution(input), 2);
}
