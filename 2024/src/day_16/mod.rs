use anyhow::Result;
use std::collections::{HashMap, HashSet, VecDeque};

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
                cost: self.cost + 1000,
            },
            Pos {
                pos: self.pos,
                dir: self.dir.right(),
                cost: self.cost + 1000,
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
    fn solve(
        &self,
    ) -> (
        usize,
        HashMap<((usize, usize), Direction), HashSet<((usize, usize), Direction)>>,
        VecDeque<((usize, usize), Direction)>,
    ) {
        let mut min_cost = usize::MAX;
        let mut seen = HashMap::<((usize, usize), Direction), usize>::new();
        let mut last_pos = VecDeque::new();

        let mut pred =
            HashMap::<((usize, usize), Direction), HashSet<((usize, usize), Direction)>>::new();

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

            // Reached the destination
            if pos.pos == self.end {
                min_cost = usize::min(min_cost, pos.cost);
                last_pos.push_back((pos.pos, pos.dir));
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

                    pred.insert((n.pos, n.dir), [(pos.pos, pos.dir)].into());
                } else if s.is_some_and(|c| *c == n.cost) {
                    pred.entry((n.pos, n.dir))
                        .or_default()
                        .insert((pos.pos, pos.dir));
                }
            }
        }

        (min_cost, pred, last_pos)
    }
}

fn tiles_on_track(
    mut last_pos: VecDeque<((usize, usize), Direction)>,
    pred: &HashMap<((usize, usize), Direction), HashSet<((usize, usize), Direction)>>,
) -> usize {
    let mut path = HashSet::new();
    let mut seen = HashSet::new();
    while let Some(pos) = last_pos.pop_front() {
        if let Some(p) = pred.get(&pos) {
            if !seen.insert(pos) {
                continue;
            }

            for n in p {
                path.insert(n.0);
                last_pos.push_back((n.0, n.1));
            }
        }
    }
    path.len() + 1
}

fn parse_input(input: &str) -> Result<Mace> {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let map = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start = (x, y);
                    } else if c == 'E' {
                        end = (x, y);
                    }
                    c
                })
                .collect()
        })
        .collect();
    Ok(Mace { map, start, end })
}

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/src/day_16/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;
    let mace = parse_input(&input)?;
    let (min_costs, pred, last_pos) = mace.solve();
    println!("Day 16, Part 1: Min costs for the mace: {min_costs}");

    let tiles = tiles_on_track(last_pos, &pred);
    println!("Day 16, Part 2: Number of tiles in the way: {tiles}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    #[test]
    fn part_one() {
        let mace = parse_input(INPUT).unwrap();
        let (min_cost, _, _) = mace.solve();
        assert_eq!(min_cost, 7036);
    }

    #[test]
    fn part_two() {
        let mace = parse_input(INPUT).unwrap();
        let (_, pred, last_pos) = mace.solve();
        let tiles = tiles_on_track(last_pos, &pred);
        assert_eq!(tiles, 58);
    }
}
