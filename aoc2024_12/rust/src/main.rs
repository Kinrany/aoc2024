use std::{
    collections::BTreeSet,
    convert::Infallible,
    fmt::Display,
    io::{BufRead, BufReader},
    str::FromStr,
};

use indexmap::IndexSet;
use itertools::iproduct;

fn cell_iter(input: &str) -> impl Iterator<Item = (i32, i32, char)> + '_ {
    input.lines().enumerate().flat_map(|(x, line)| {
        line.chars()
            .filter(|ch| ['.', '#', '^', 'v', '>', '<'].contains(ch))
            .enumerate()
            .map(move |(y, ch)| (x as i32, y as i32, ch))
    })
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn new(ch: char) -> Option<Self> {
        Self::from_str(&ch.to_string()).ok()
    }
    #[must_use]
    fn turn_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }
    fn dx(self) -> i32 {
        match self {
            Self::Up => -1,
            Self::Down => 1,
            Self::Left => 0,
            Self::Right => 0,
        }
    }
    fn dy(self) -> i32 {
        match self {
            Self::Up => 0,
            Self::Down => 0,
            Self::Left => -1,
            Self::Right => 1,
        }
    }
}
impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "^" => Ok(Self::Up),
            "v" => Ok(Self::Down),
            ">" => Ok(Self::Right),
            "<" => Ok(Self::Left),
            _ => Err(()),
        }
    }
}
impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = match self {
            Self::Up => '^',
            Self::Down => 'v',
            Self::Left => '<',
            Self::Right => '>',
        };
        write!(f, "{ch}")
    }
}

#[derive(Clone, Debug)]
struct Map {
    size: (i32, i32),
    obstacles: BTreeSet<(i32, i32)>,
}
impl Map {
    fn new(input: &str) -> Self {
        let size = (
            input.lines().count() as i32,
            input.lines().next().unwrap().len() as i32,
        );
        let obstacles = cell_iter(input)
            .filter_map(|(x, y, ch)| (ch == '#').then_some((x, y)))
            .collect();
        Self { size, obstacles }
    }

    fn on_map(&self, (x, y): (i32, i32)) -> bool {
        (0..self.size.0).contains(&x) && (0..self.size.1).contains(&y)
    }

    fn is_obstacle(&self, (x, y): (i32, i32)) -> bool {
        self.obstacles.contains(&(x, y))
    }

    fn next(&self, guard: Guard) -> Option<Guard> {
        let next_cell = guard.next_cell();
        if !self.on_map(next_cell) {
            return None;
        }
        if self.is_obstacle(next_cell) {
            Some(guard.turn_right())
        } else {
            Some(guard.move_forward())
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Guard {
    position: (i32, i32),
    direction: Direction,
}
impl Guard {
    fn new(input: &str) -> Self {
        let Some((position, direction)) =
            cell_iter(input).find_map(|(x, y, ch)| Direction::new(ch).map(|dir| ((x, y), dir)))
        else {
            panic!("invalid input");
        };

        Self {
            position,
            direction,
        }
    }

    fn next_cell(self) -> (i32, i32) {
        let x = self.position.0 + self.direction.dx();
        let y = self.position.1 + self.direction.dy();
        (x, y)
    }

    #[must_use]
    fn move_forward(self) -> Self {
        Self {
            position: self.next_cell(),
            direction: self.direction,
        }
    }

    #[must_use]
    fn turn_right(self) -> Self {
        Self {
            position: self.position,
            direction: self.direction.turn_right(),
        }
    }
}

#[derive(Clone, Debug)]
struct Board {
    map: Map,
    guard: Guard,
}
impl Board {
    /// Returns all positions, and `true` if the guard gets stuck in a loop.
    pub fn simulate(&self) -> (IndexSet<Guard>, bool) {
        let mut guard = self.guard;

        let mut history = IndexSet::<Guard>::new();
        history.insert(guard);

        loop {
            let Some(new_guard) = self.map.next(guard) else {
                return (history, false);
            };
            guard = new_guard;
            if history.contains(&guard) {
                return (history, true);
            }
            history.insert(guard);
        }
    }
}
impl FromStr for Board {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            map: Map::new(s),
            guard: Guard::new(s),
        })
    }
}
impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in 0..self.map.size.0 {
            for y in 0..self.map.size.1 {
                if self.map.is_obstacle((x, y)) {
                    write!(f, "#")?;
                } else if self.guard.position == (x, y) {
                    write!(f, "{}", self.guard.direction)?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn input(mut input: impl BufRead) -> anyhow::Result<String> {
    let mut s = String::new();
    input.read_to_string(&mut s)?;
    Ok(s.trim().into())
}

fn solution(input: &str) -> usize {
    // We are going to straight up simulate the guard walking.
    let Ok(board) = Board::from_str(input);

    // This is actually slow AF, takes about 2 minutes to run all the simulations.
    iproduct!(0..board.map.size.0, 0..board.map.size.1)
        .filter(|(x, y)| !board.map.is_obstacle((*x, *y)) && board.guard.position != (*x, *y))
        .filter(|(x, y)| {
            let mut board = board.clone();
            board.map.obstacles.insert((*x, *y));
            let (_, looped) = board.simulate();
            looped
        })
        .count()
}

fn main() -> anyhow::Result<()> {
    let input = input(BufReader::new(std::io::stdin()))?;
    let answer = solution(&input);
    println!("{answer}");
    Ok(())
}

#[test]
fn example_round_trip() {
    let text = include_str!("../../example.txt");
    let Ok(board) = Board::from_str(text);
    assert_eq!(text, board.to_string());
    insta::assert_snapshot!(board);
}

#[test]
fn make_step_up() {
    let input = "....#.....
                 .........#
                 ..........
                 ..#.......
                 .......#..
                 ..........
                 .#..^.....
                 ........#.
                 #.........
                 ......#...";
    let Ok(mut board) = Board::from_str(input);
    board.guard = board.map.next(board.guard).unwrap();
    insta::assert_snapshot!(board);
}

#[test]
fn make_step_right() {
    let input = "....#.....
                 .........#
                 ..........
                 ..#.......
                 .......#..
                 ..........
                 .#..>.....
                 ........#.
                 #.........
                 ......#...";
    let Ok(mut board) = Board::from_str(input);
    board.guard = board.map.next(board.guard).unwrap();
    insta::assert_snapshot!(board);
}

#[test]
fn turn_right() {
    let input = "....#.....
                 .........#
                 ..........
                 ..#.......
                 .......#..
                 ....#.....
                 .#..^.....
                 ........#.
                 #.........
                 ......#...";
    let Ok(mut board) = Board::from_str(input);
    board.guard = board.map.next(board.guard).unwrap();
    insta::assert_snapshot!(board);
}

#[test]
fn turn_down() {
    let input = "....#.....
                 .........#
                 ..........
                 ..#.......
                 .......#..
                 ..........
                 .#..>#....
                 ........#.
                 #.........
                 ......#...";
    let Ok(mut board) = Board::from_str(input);
    board.guard = board.map.next(board.guard).unwrap();
    insta::assert_snapshot!(board);
}

#[test]
fn solve_example() {
    let input = "....#.....
                 .........#
                 ..........
                 ..#.......
                 .......#..
                 ..........
                 .#..^.....
                 ........#.
                 #.........
                 ......#...";
    assert_eq!(solution(input), 6);
}
