use std::io::{BufReader, Read};

use itertools::Itertools;

fn parse_lit<'a>(input: &'a [u8], literal: &[u8]) -> Option<&'a [u8]> {
    input.starts_with(literal).then(|| &input[literal.len()..])
}

#[test]
fn test_parse_lit() {
    assert_eq!(parse_lit(b"", b"x"), None);
    assert_eq!(parse_lit(b"x", b"x"), Some(b"".as_slice()));
    assert_eq!(parse_lit(b"xy", b"xy"), Some(b"".as_slice()));
    assert_eq!(parse_lit(b"xy", b"x"), Some(b"y".as_slice()));
    assert_eq!(parse_lit(b"xy", b"xz"), None);
}

fn parse_num(input: &[u8]) -> Option<(i32, &[u8])> {
    let first_non_number = input
        .iter()
        .find_position(|n| !n.is_ascii_digit())
        .map_or(input.len(), |(idx, _)| idx);
    let (number, rest) = input.split_at(first_non_number);
    let s = std::str::from_utf8(number).ok()?;
    let number: i32 = s.parse().ok()?;
    Some((number, rest))
}

#[test]
fn test_parse_num() {
    assert_eq!(parse_num(b""), None);
    assert_eq!(parse_num(b"1"), Some((1, b"".as_slice())));
    assert_eq!(parse_num(b"12"), Some((12, b"".as_slice())));
    assert_eq!(parse_num(b"a"), None);
    assert_eq!(parse_num(b"1a"), Some((1, b"a".as_slice())));
}

fn parse_mul(input: &[u8]) -> Option<((i32, i32), &[u8])> {
    let input = parse_lit(input, b"mul(")?;
    let (n1, input) = parse_num(input)?;
    let input = parse_lit(input, b",")?;
    let (n2, input) = parse_num(input)?;
    let input = parse_lit(input, b")")?;
    Some(((n1, n2), input))
}

#[test]
fn test_parse_mul() {
    assert_eq!(parse_mul(b""), None);
    assert_eq!(parse_mul(b"mul(1,2)"), Some(((1, 2), b"".as_slice())));
    assert_eq!(parse_mul(b"mul(1,2)x"), Some(((1, 2), b"x".as_slice())));
    assert_eq!(parse_mul(b"xmul(1,2)"), None);
}

fn parse_dodont(input: &[u8]) -> Option<(bool, &[u8])> {
    #[allow(clippy::manual_map)]
    if let Some(rest) = parse_lit(input, b"do()") {
        Some((true, rest))
    } else if let Some(rest) = parse_lit(input, b"don't()") {
        Some((false, rest))
    } else {
        None
    }
}

fn solution(mut input: &[u8]) -> i32 {
    let mut sum = 0;
    let mut dodont = true;
    while !input.is_empty() {
        if let Some(((a, b), rest)) = parse_mul(input) {
            if dodont {
                sum += a * b;
            }
            input = rest;
        } else if let Some((new_dodont, rest)) = parse_dodont(input) {
            dodont = new_dodont;
            input = rest;
        } else {
            input = &input[1..];
        }
    }
    sum
}

fn main() -> anyhow::Result<()> {
    let mut bytes = vec![];
    BufReader::new(std::io::stdin()).read_to_end(&mut bytes)?;
    let answer = solution(&bytes);
    println!("{answer}");
    Ok(())
}

#[test]
fn solve_example() {
    let bytes = include_bytes!("../../example.txt");
    assert_eq!(solution(bytes), 48);
}
