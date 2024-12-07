use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    io::{BufRead, BufReader},
};

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

struct Rules<T> {
    /// `map[a].contains(b)` represents a rule `a > b`.
    map: HashMap<T, HashSet<T>>,
}
impl<T: Clone + Hash + Eq> Rules<T> {
    fn new(rules: &[(T, T)]) -> Self {
        let map = rules
            .iter()
            .fold(HashMap::new(), |mut acc: HashMap<T, HashSet<T>>, (b, a)| {
                acc.entry(a.clone()).or_default().insert(b.clone());
                acc
            });
        Self { map }
    }
    fn iter_lesser(&self, x: &T) -> impl Iterator<Item = T> + '_ {
        self.map.get(x).into_iter().flatten().cloned()
    }
    fn sort_topologically(&self, items: &[T]) -> Vec<T> {
        let mut unused: HashSet<T> = items.iter().cloned().collect();
        let mut sorted = vec![];
        while let Some(x) = unused.iter().next() {
            let mut x = x.clone();
            // Climb down with unused numbers for as long as we can
            while let Some(y) = self.iter_lesser(&x).find(|y| unused.contains(y)) {
                x = y;
            }
            // Push the smallest number
            unused.remove(&x);
            sorted.push(x);
        }
        sorted
    }
}

fn solution((rules, updates): Input) -> i32 {
    // Make a map of rules that's easy to look up
    let rules = Rules::new(&rules);

    updates
        .into_iter()
        // Sort the numbers in each update, discarding updates that were already sorted.
        .filter_map(move |update| {
            let sorted_update = rules.sort_topologically(&update);
            (update != sorted_update).then_some(sorted_update)
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
    assert_eq!(solution(input), 123);
}
