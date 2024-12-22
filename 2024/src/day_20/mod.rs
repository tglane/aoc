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
    fn shortest_path(&self) -> Option<Vec<(usize, usize)>> {
        let mut seen = HashMap::<((usize, usize), Direction), usize>::new();
        let mut last_pos = VecDeque::new();

        let mut pred = HashMap::<((usize, usize), Direction), Pos>::new();

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
                last_pos.push_back(pos);
                break;
            }

            for n in pos.next() {
                if self.map[n.pos.1][n.pos.0] == '#' {
                    continue;
                }

                let s = seen.get(&(n.pos, n.dir));
                if s.is_none_or(|c| *c > n.cost) {
                    seen.insert((n.pos, n.dir), n.cost);
                    queue.push_front(n.clone());

                    pred.insert((n.pos, n.dir), pos.clone());
                }
            }
        }

        for end in last_pos {
            let mut path = vec![end];
            loop {
                let last = path.last()?.clone();
                if last.pos == self.start {
                    break;
                }
                let Some(prev) = pred.get(&(last.pos, last.dir)) else {
                    return None;
                };

                path.push(prev.clone());
            }

            // path.reverse();
            let mut path = path.iter().map(|p| p.pos).collect::<Vec<_>>();
            path.dedup();
            return Some(path);
        }

        None
    }
}

fn find_cheats(
    path: &[(usize, usize)],
    max_cheat_time: usize,
    min_savings: usize,
    start_time: usize,
) -> usize {
    let mut viable = 0;
    let cheat_start = path[start_time];
    if start_time > path.len() - min_savings {
        return 0;
    }
    let mut normal_end_time = start_time + min_savings;
    while normal_end_time < path.len() {
        let cheat_end = path[normal_end_time];

        let cheat_dist_manhattan = {
            ((cheat_start.0 as isize - cheat_end.0 as isize).abs()
                + (cheat_start.1 as isize - cheat_end.1 as isize).abs()) as usize
        };
        if cheat_dist_manhattan > max_cheat_time {
            normal_end_time += cheat_dist_manhattan - max_cheat_time;
        } else {
            let cheat_end_time = start_time + cheat_dist_manhattan;
            let savings = normal_end_time - cheat_end_time;
            if cheat_dist_manhattan <= max_cheat_time && savings >= min_savings {
                viable += 1;
            }
            normal_end_time += 1;
        }
    }
    viable
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
        "{}/src/day_20/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;
    let mace = parse_input(&input)?;
    let path = mace.shortest_path().context("No path found")?;

    let cheat_count = (0..path.len())
        .map(|cheat_start| find_cheats(&path, 2, 100, cheat_start))
        .sum::<usize>();
    println!("Day 20, Part 1: Number of cheat codes that safe 100 picoseconds: {cheat_count}");

    let cheat_count = (0..path.len())
        .map(|cheat_start| find_cheats(&path, 20, 100, cheat_start))
        .sum::<usize>();
    println!("Day 20, Part 2: Number of cheat codes that safe 100 picoseconds: {cheat_count}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn part_one() {
        let mace = parse_input(INPUT).unwrap();
        let path = mace.shortest_path().unwrap();
        assert_eq!(path.len(), 85);

        let cheat_count = (0..path.len())
            .map(|cheat_start| find_cheats(&path, 2, 2, cheat_start))
            .sum::<usize>();
        assert_eq!(cheat_count, 14 + 14 + 2 + 4 + 2 + 3 + 5);
    }
}
