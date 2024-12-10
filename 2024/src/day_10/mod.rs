use std::collections::HashSet;

use anyhow::{Context, Result};

#[derive(Debug)]
struct Map {
    data: Vec<Vec<u32>>,
    heads: Vec<(isize, isize)>,
}

impl Map {
    fn score(&self) -> usize {
        let mut map_score = 0;
        for head in &self.heads {
            let mut visited = HashSet::new();
            self.trail_score(head, Some(&mut visited))
                .inspect(|score| map_score += score);
        }
        map_score
    }

    fn rating(&self) -> usize {
        let mut map_rating = 0;
        for head in &self.heads {
            self.trail_score(head, None)
                .inspect(|score| map_rating += score);
        }
        map_rating
    }

    fn in_bounds(&self, pos: &(isize, isize)) -> bool {
        !(pos.0 < 0
            || pos.0 as usize >= self.data.len()
            || pos.1 < 0
            || pos.1 as usize >= self.data[pos.0 as usize].len())
    }

    fn trail_score(
        &self,
        pos: &(isize, isize),
        mut visited: Option<&mut HashSet<(isize, isize)>>,
    ) -> Option<usize> {
        if let Some(v) = &mut visited {
            if v.contains(pos) {
                return None;
            }
            v.insert(*pos);
        }

        if self.data[pos.0 as usize][pos.1 as usize] == 9 {
            return Some(1);
        }

        let mut indirect_scores = 0;
        let dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        for dir in dirs.iter() {
            let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
            if self.in_bounds(&new_pos)
                && self.data[new_pos.0 as usize][new_pos.1 as usize]
                    == self.data[pos.0 as usize][pos.1 as usize] + 1
            {
                self.trail_score(&new_pos, visited.as_deref_mut())
                    .inspect(|score| indirect_scores += score);
            }
        }
        Some(indirect_scores)
    }
}

fn parse_input(input: &str) -> Result<Map> {
    let mut heads = Vec::new();
    let data = input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(|(j, c)| {
                    if c == '0' {
                        heads.push((i.try_into()?, j.try_into()?));
                    }
                    c.to_digit(10).context("")
                })
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<Vec<_>>>>()?;

    Ok(Map { data, heads })
}

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/src/day_10/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;
    let map = parse_input(&input)?;

    let score = map.score();
    println!("Day 10, Part 1: Map score: {score}");

    let rating = map.rating();
    println!("Day 10, Part 2: Map rating: {rating}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn part_one() {
        let map = parse_input(INPUT).unwrap();
        let score = map.score();
        assert_eq!(score, 36);
    }

    #[test]
    fn part_two() {
        let map = parse_input(INPUT).unwrap();
        let score = map.rating();
        assert_eq!(score, 81);
    }
}
