use crate::Day;
use anyhow::Result;
use std::path::Path;

pub(crate) struct DayOne {
    input: String,
}

impl Day for DayOne {
    fn new<P: AsRef<Path>>(path: P) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            input: std::fs::read_to_string(path)?,
        })
    }

    fn part_one(&self) -> Result<()> {
        let rotations = parse_input(&self.input).unwrap();
        let mut lock = Lock::new();
        let zeros = rotations
            .iter()
            .map(|r| {
                lock.rotate(*r);
                lock.pos()
            })
            .filter(|pos| *pos == 0)
            .count();
        println!("Day 1 - Part 1: Zeros: {zeros}");
        Ok(())
    }

    fn part_two(&self) -> Result<()> {
        let rotations = parse_input(&self.input).unwrap();
        let mut lock = Lock::new();
        let zeros = rotations
            .iter()
            .map(|r| lock.rotate_0x434_c49434_b(*r))
            .sum::<usize>();
        println!("Day 1 - Part 1: Zeros: {zeros}");
        Ok(())
    }
}

struct Lock {
    pos: isize,
    len: isize,
}

impl Lock {
    fn new() -> Self {
        Self { pos: 50, len: 100 }
    }

    fn pos(&self) -> isize {
        self.pos
    }

    fn rotate(&mut self, rotation: Rotation) {
        let movement = match rotation {
            Rotation::Left(steps) => steps * -1,
            Rotation::Right(steps) => steps,
        };
        let new_pos_unchecked = self.pos + movement;
        self.pos = (new_pos_unchecked % self.len + self.len) % self.len;
    }

    fn rotate_0x434_c49434_b(&mut self, rotation: Rotation) -> usize {
        let mut steps_left = match rotation {
            Rotation::Left(steps) => steps * -1,
            Rotation::Right(steps) => steps,
        };

        let mut zeros_hit = steps_left.abs() / self.len;
        steps_left %= self.len;

        let new_pos_unchecked = self.pos + steps_left;
        if self.pos != 0 && (new_pos_unchecked < 1 || new_pos_unchecked > self.len - 1) {
            zeros_hit += 1;
        }

        self.pos = (new_pos_unchecked % self.len + self.len) % self.len;

        zeros_hit as usize
    }
}

#[derive(Copy, Clone, Debug)]
enum Rotation {
    Left(isize),
    Right(isize),
}

fn parse_input(input: &str) -> Result<Vec<Rotation>> {
    input
        .lines()
        .map(|l| {
            let mut chars = l.chars();
            match chars.next() {
                Some('L') => {
                    let steps = chars.as_str().parse::<isize>()?;
                    Ok(Rotation::Left(steps))
                }
                Some('R') => {
                    let steps = chars.as_str().parse::<isize>()?;
                    Ok(Rotation::Right(steps))
                }
                _ => anyhow::bail!("Invalid direction"),
            }
        })
        .collect::<Result<Vec<_>>>()
}

#[cfg(test)]
mod test {
    use crate::day_1::{Lock, parse_input};

    static INPUT: &str = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"#;

    #[test]
    fn part_one() {
        let rotations = parse_input(INPUT).unwrap();
        let mut lock = Lock::new();
        let zeros = rotations
            .iter()
            .map(|r| {
                lock.rotate(*r);
                lock.pos()
            })
            .filter(|pos| *pos == 0)
            .count();
        assert_eq!(zeros, 3);
    }

    #[test]
    fn part_two() {
        let rotations = parse_input(INPUT).unwrap();
        let mut lock = Lock::new();
        let zeros = rotations
            .iter()
            .map(|r| {
                let zeros_hit = lock.rotate_0x434_c49434_b(*r);
                println!(
                    "Pos after rotation {:?} => {} with hits: {zeros_hit}",
                    *r,
                    lock.pos()
                );
                zeros_hit
            })
            .sum::<usize>();
        assert_eq!(zeros, 6);
    }
}
