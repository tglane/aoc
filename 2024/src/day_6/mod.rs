use anyhow::{bail, Context, Result};
use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn apply(&self, pos: (usize, usize), max: (usize, usize)) -> Result<(usize, usize)> {
        Ok(match self {
            Self::Up => (pos.0.checked_sub(1).context("")?, pos.1),
            Self::Down => {
                if pos.0 + 1 >= max.0 {
                    bail!("");
                }
                (pos.0 + 1, pos.1)
            }
            Self::Right => {
                if pos.1 + 1 >= max.1 {
                    bail!("");
                }
                (pos.0, pos.1 + 1)
            }
            Self::Left => (pos.0, pos.1.checked_sub(1).context("")?),
        })
    }

    fn turn(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn get_dir(pos: char) -> Option<Direction> {
    match pos {
        '^' => Some(Direction::Up),
        '<' => Some(Direction::Left),
        '>' => Some(Direction::Right),
        'v' => Some(Direction::Down),
        _ => None,
    }
}

fn positions_visited(mace: &Vec<Vec<char>>) -> Result<usize> {
    let mut visited = HashSet::<(usize, usize)>::new();

    let mut loop_visited = HashSet::<(usize, usize, Direction)>::new();

    let mut dir = Direction::Up;
    let mut pos = (0, 0);
    'outer: for i in 0..mace.len() {
        for j in 0..mace[i].len() {
            if let Some(found_dir) = get_dir(mace[i][j]) {
                dir = found_dir;
                pos = (i, j);
                visited.insert((i, j));
                break 'outer;
            }
        }
    }

    loop {
        if let Ok(new_pos) = dir.apply(pos, (mace.len(), mace[0].len())) {
            if mace[new_pos.0][new_pos.1] == '#' {
                dir = dir.turn();

                let loop_tup = (pos.0, pos.1, dir);
                if loop_visited.contains(&loop_tup) {
                    bail!("Loop detected");
                }
                loop_visited.insert(loop_tup);
            } else {
                pos = new_pos;
                visited.insert(new_pos);

                let loop_tup = (pos.0, pos.1, dir);
                if loop_visited.contains(&loop_tup) {
                    bail!("Loop detected");
                }
                loop_visited.insert(loop_tup);
            }
        } else {
            return Ok(visited.len());
        }
    }
}

fn loop_positions(mace: &Vec<Vec<char>>) -> usize {
    let mut mace = mace.clone();

    let mut loop_locations = 0;
    for i in 0..mace.len() {
        for j in 0..mace[i].len() {
            let tmp = mace[i][j].clone();
            if tmp != '#' {
                mace[i][j] = '#';

                if let Err(_) = positions_visited(&mace) {
                    loop_locations += 1;
                }

                mace[i][j] = tmp;
            }
        }
    }

    loop_locations
}

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/src/day_6/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;
    let mace = parse_input(&input);

    let pos_visited = positions_visited(&mace)?;
    println!("Day 6, Part 1: Visited positions by guard: {pos_visited}");

    let loop_positions = loop_positions(&mace);
    println!("Day 6, Part 2: Obstacle positions to create loop: {loop_positions}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn part_one() {
        let input = parse_input(INPUT);
        let visited = positions_visited(&input).unwrap();
        assert_eq!(visited, 41);
    }

    #[test]
    fn part_two() {
        let input = parse_input(INPUT);
        let loop_positions = loop_positions(&input);
        assert_eq!(loop_positions, 6);
    }
}
