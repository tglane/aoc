use anyhow::{Context, Result};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Profile(u8, u8, u8, u8, u8);

impl Profile {
    fn at(&self, idx: usize) -> Option<&u8> {
        match idx {
            0 => Some(&self.0),
            1 => Some(&self.1),
            2 => Some(&self.2),
            3 => Some(&self.3),
            4 => Some(&self.4),
            _ => None,
        }
    }

    fn at_mut(&mut self, idx: usize) -> Option<&mut u8> {
        match idx {
            0 => Some(&mut self.0),
            1 => Some(&mut self.1),
            2 => Some(&mut self.2),
            3 => Some(&mut self.3),
            4 => Some(&mut self.4),
            _ => None,
        }
    }

    fn overlap(&self, other: &Self) -> bool {
        for i in 0..5 {
            if *self.at(i).unwrap() + *other.at(i).unwrap() > 5 {
                return true;
            }
        }
        false
    }

    fn collect<'a>(it: impl Iterator<Item = &'a str>) -> Result<Self> {
        let mut p = Self(0, 0, 0, 0, 0);
        for line in it.skip(1) {
            for (i, ch) in line.chars().enumerate() {
                if ch == '#' {
                    *p.at_mut(i).context("Invalid profile index")? += 1;
                    assert!(*p.at(i).unwrap() <= 5);
                }
            }
        }
        Ok(p)
    }
}

fn count_pairs(locks: &[Profile], keys: &[Profile]) -> usize {
    let mut pairs = 0;
    for lock in locks {
        for key in keys {
            if !lock.overlap(key) {
                pairs += 1;
            }
        }
    }
    pairs
}

fn parse_input(input: &str) -> Result<(Vec<Profile>, Vec<Profile>)> {
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for block in input.split("\n\n") {
        if block.lines().next() == Some("#####") {
            locks.push(Profile::collect(block.lines())?);
        } else {
            keys.push(Profile::collect(block.lines().rev())?);
        }
    }

    Ok((locks, keys))
}

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/src/day_25/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;
    let (locks, keys) = parse_input(&input)?;

    let pairs = count_pairs(&locks, &keys);
    println!("Day 25, Part 1: Number of lock-key pairs: {pairs}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

    #[test]
    fn part_one() {
        let (locks, keys) = parse_input(INPUT).unwrap();
        let pairs = count_pairs(&locks, &keys);
        assert_eq!(pairs, 3);
    }
}
