use std::io::{BufRead, BufReader};

use itertools::Itertools;
fn input(input: impl BufRead) -> anyhow::Result<Vec<Vec<char>>> {
    input
        .lines()
        .map_ok(|l| l.chars().collect_vec())
        .collect::<Result<_, _>>()
        .map_err(anyhow::Error::from)
}

fn solution(input: Vec<Vec<char>>, word: &str) -> usize {
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let x_len = input.len() as i32;
    let y_len = input[0].len() as i32;
    let i_max = word.len() as i32 - 1;

    itertools::iproduct!((0..x_len), (0..y_len), directions)
        .filter(|&(x, y, (dx, dy))| {
            (0..x_len).contains(&(x + dx * i_max))
                && (0..y_len).contains(&(y + dy * i_max))
                && word.chars().enumerate().all(|(i, ch)| {
                    let x = x + dx * i as i32;
                    let y = y + dy * i as i32;
                    input[x as usize][y as usize] == ch
                })
        })
        .inspect(|&(x, y, (dx, dy))| println!("{x} {y} {dx} {dy}"))
        .count()
}

fn main() -> anyhow::Result<()> {
    let input = input(BufReader::new(std::io::stdin()))?;
    let answer = solution(input, "XMAS");
    println!("{answer}");
    Ok(())
}

#[test]
fn parse_example() {
    let bytes = include_bytes!("../../example.txt");
    insta::assert_debug_snapshot!(input(bytes.as_slice()));
}

#[test]
fn solve_example() {
    let input = vec![
        vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
        vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
        vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
        vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
        vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
        vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
        vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
        vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
        vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
        vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X'],
    ];
    assert_eq!(solution(input, "XMAS"), 18);
}
