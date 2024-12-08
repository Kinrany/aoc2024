use std::{
    fmt::Display,
    io::{BufRead, BufReader},
    str::FromStr,
};

use anyhow::Context;
use indexmap::{IndexMap, IndexSet};

const EMPTY: char = '.';

struct Input {
    antennas: IndexMap<char, IndexSet<(usize, usize)>>,
    height: usize,
    width: usize,
}
impl Input {
    fn from_bufread(mut input: impl BufRead) -> anyhow::Result<Self> {
        let mut s = String::new();
        input.read_to_string(&mut s)?;
        Self::from_str(&s)
    }
}
impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().count();
        let width = s.lines().next().context("empty input")?.len();

        let antennas = s
            .lines()
            .enumerate()
            .flat_map(|(x, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, ch)| *ch != EMPTY)
                    .map(move |(y, ch)| (x, y, ch))
            })
            .fold(
                anyhow::Ok(IndexMap::<_, IndexSet<_>>::new()),
                |acc, (x, y, ch)| {
                    let mut acc = acc?;
                    acc.entry(ch).or_default().insert((x, y));
                    Ok(acc)
                },
            )?;

        Ok(Self {
            antennas,
            height,
            width,
        })
    }
}
impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut field = vec![vec![EMPTY; self.width]; self.height];
        for (ch, positions) in &self.antennas {
            for &(x, y) in positions {
                field[x][y] = *ch;
            }
        }
        for line in field {
            for ch in line {
                write!(f, "{ch}")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn solution(input: Input) -> usize {
    let mut antinodes = IndexSet::new();
    for (_, positions) in &input.antennas {
        for &(bx, by) in positions {
            for &(cx, cy) in positions {
                if (bx, by) == (cx, cy) {
                    continue;
                }
                if let (Some(ax), Some(ay)) = ((bx * 2).checked_sub(cx), (by * 2).checked_sub(cy)) {
                    if ax < input.height && ay < input.width {
                        antinodes.insert((ax, ay));
                    }
                }
                if let (Some(dx), Some(dy)) = ((cx * 2).checked_sub(bx), (cy * 2).checked_sub(by)) {
                    if dx < input.height && dy < input.width {
                        antinodes.insert((dx, dy));
                    }
                }
            }
        }
    }
    antinodes.len()
}

fn main() -> anyhow::Result<()> {
    let input = Input::from_bufread(BufReader::new(std::io::stdin()))?;
    let answer = solution(input);
    println!("{answer}");
    Ok(())
}

#[test]
fn example_round_trip() -> anyhow::Result<()> {
    let text = include_str!("../../example.txt");
    let input = Input::from_str(text)?;
    assert_eq!(text, input.to_string());
    Ok(())
}

#[test]
fn solve_example() -> anyhow::Result<()> {
    let text = include_str!("../../example.txt");
    let input = Input::from_str(text)?;
    assert_eq!(solution(input), 14);
    Ok(())
}
