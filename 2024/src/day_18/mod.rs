use anyhow::{Context, Result};
use std::collections::{HashMap, VecDeque};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Left,
    Down,
}

impl Direction {
    fn left(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Right => Self::Up,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
        }
    }

    fn right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Left => Self::Up,
            Self::Down => Self::Left,
        }
    }

    fn step(&self, pos: &(usize, usize)) -> (usize, usize) {
        match self {
            Self::Up => (pos.0, pos.1 - 1),
            Self::Right => (pos.0 + 1, pos.1),
            Self::Left => (pos.0 - 1, pos.1),
            Self::Down => (pos.0, pos.1 + 1),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Pos {
    pos: (usize, usize),
    dir: Direction,
    cost: usize,
}

impl Pos {
    fn next(&self) -> [Pos; 3] {
        [
            Pos {
                pos: self.dir.step(&self.pos),
                dir: self.dir,
                cost: self.cost + 1,
            },
            Pos {
                pos: self.pos,
                dir: self.dir.left(),
                cost: self.cost,
            },
            Pos {
                pos: self.pos,
                dir: self.dir.right(),
                cost: self.cost,
            },
        ]
    }
}

struct Mace {
    map: Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Mace {
    fn fill(&mut self, blocks: &[(usize, usize)]) {
        for b in blocks {
            self.map[b.1][b.0] = '#';
        }
    }

    fn solve(&self) -> Option<usize> {
        let mut min_cost = usize::MAX;
        let mut seen = HashMap::<((usize, usize), Direction), usize>::new();

        let mut queue = VecDeque::from([Pos {
            pos: self.start,
            dir: Direction::Right,
            cost: 0,
        }]);

        while let Some(pos) = queue.pop_front() {
            // Already seen with lower cost -> do not need to follow that path
            if seen.get(&(pos.pos, pos.dir)).is_some_and(|c| *c < pos.cost) {
                continue;
            }

            if pos.cost > min_cost {
                continue;
            }

            // Reached the destination
            if pos.pos == self.end {
                min_cost = usize::min(min_cost, pos.cost);
                continue;
            }

            for n in pos.next() {
                if self.map[n.pos.1][n.pos.0] == '#' {
                    continue;
                }

                let s = seen.get(&(n.pos, n.dir));
                if s.is_none_or(|c| *c > n.cost) {
                    seen.insert((n.pos, n.dir), n.cost);
                    queue.push_front(n.clone());
                }
            }
        }

        if min_cost == usize::MAX {
            None
        } else {
            Some(min_cost)
        }
    }
}

fn find_first_block(mut mace: Mace, blocks: &[(usize, usize)]) -> Option<(usize, usize)> {
    for b in blocks.iter() {
        mace.fill(&[*b]);
        if mace.solve().is_none() {
            return Some((b.0 - 1, b.1 - 1));
        }
    }
    return None;
}

fn parse_input(input: &str, dimensions: usize) -> Result<(Vec<(usize, usize)>, Mace)> {
    let start = (1, 1);
    let end = (dimensions, dimensions);
    let mut map = vec![vec!['.'; dimensions + 2]; dimensions + 2];

    for x in 0..dimensions + 2 {
        map[0][x] = '#';
        map[dimensions + 1][x] = '#';
        map[x][0] = '#';
        map[x][dimensions + 1] = '#';
    }

    let lines = input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').context("")?;
            Ok((x.parse::<usize>()? + 1, y.parse::<usize>()? + 1))
        })
        .collect::<Result<Vec<_>>>();

    Ok((lines?, Mace { map, start, end }))
}

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/src/day_18/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;
    let (blocks, mut mace) = parse_input(&input, 71)?;
    mace.fill(&blocks[0..1024]);

    let min_steps = mace.solve();
    println!("Day 18, Part 1: Min steps for the mace: {min_steps:?}");

    let first_blocker = find_first_block(mace, &blocks[1024..]).unwrap();
    println!("Day 18, Part 2: Path first blocked after block at: {first_blocker:?}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn part_one() {
        let (blocks, mut mace) = parse_input(INPUT, 7).unwrap();
        mace.fill(&blocks[0..12]);
        let min_steps = mace.solve().unwrap();
        assert_eq!(min_steps, 22);
    }

    #[test]
    fn part_two() {
        let (blocks, mut mace) = parse_input(INPUT, 7).unwrap();
        mace.fill(&blocks[0..12]);
        let first_blocker = find_first_block(mace, &blocks[12..]).unwrap();
        assert_eq!(first_blocker, (6, 1));
    }
}
