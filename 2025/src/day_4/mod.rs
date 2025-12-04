use crate::Day;
use anyhow::Result;
use std::path::Path;

pub(crate) struct DayFour {
    input: String,
}

impl Day for DayFour {
    fn new<P: AsRef<Path>>(path: P) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            input: std::fs::read_to_string(path)?,
        })
    }

    fn part_one(&self) -> Result<()> {
        let map = Map::try_from(self.input.as_str())?;
        let accessible_rolls = map.count_accessible_rolls();
        println!("Day 4 - Part 1: Accessible paper rolls: {accessible_rolls}");
        Ok(())
    }

    fn part_two(&self) -> Result<()> {
        let mut map = Map::try_from(self.input.as_str())?;
        let removed_rolls = map.remove_accessible_rolls();
        println!("Day 4 - Part 1: Removed paper rolls: {removed_rolls}");
        Ok(())
    }
}

struct Map(Vec<Vec<Field>>);

impl Map {
    fn print(&self) {
        for line in self.0.iter() {
            for field in line.iter() {
                print!("{field}");
            }
            println!();
        }
    }

    fn count_accessible_rolls(&self) -> usize {
        let mut accessible = 0;
        for y in 0..self.0.len() {
            for x in 0..self.0[y].len() {
                if self.0[y][x] == Field::PaperRoll && self.accessible_field(x, y) {
                    accessible += 1;
                }
            }
        }
        accessible
    }

    fn remove_accessible_rolls(&mut self) -> usize {
        let mut accessible = 0;
        for y in 0..self.0.len() {
            for x in 0..self.0[y].len() {
                if self.0[y][x] == Field::PaperRoll && self.accessible_field(x, y) {
                    accessible += 1;
                    self.0[y][x] = Field::Empty;
                }
            }
        }
        if accessible == 0 {
            accessible
        } else {
            accessible + self.remove_accessible_rolls()
        }
    }

    fn accessible_field(&self, x: usize, y: usize) -> bool {
        let mut adjacent = 0;
        for other_y in y.checked_sub(1).unwrap_or(y)..=y + 1 {
            for other_x in x.checked_sub(1).unwrap_or(x)..=x + 1 {
                if other_x < self.0[y].len()
                    && other_y < self.0.len()
                    && !(other_x == x && other_y == y)
                    && self.0[other_y][other_x] == Field::PaperRoll
                {
                    adjacent += 1;
                }
            }
        }

        adjacent < 4
    }
}

impl TryFrom<&str> for Map {
    type Error = anyhow::Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let fields = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(Field::try_from)
                    .collect::<Result<Vec<_>>>()
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Map(fields))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Field {
    PaperRoll,
    Empty,
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::PaperRoll => '@',
            Self::Empty => '.',
        };
        write!(f, " {c} ")
    }
}

impl TryFrom<char> for Field {
    type Error = anyhow::Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '@' => Ok(Field::PaperRoll),
            '.' => Ok(Field::Empty),
            _ => Err(anyhow::Error::msg("Invalid field value")),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"#;

    #[test]
    fn part_one() {
        let map = Map::try_from(INPUT).unwrap();
        let accessible_rolls = map.count_accessible_rolls();
        assert_eq!(accessible_rolls, 13);
    }

    #[test]
    fn part_two() {
        let mut map = Map::try_from(INPUT).unwrap();
        let accessible_rolls = map.remove_accessible_rolls();
        assert_eq!(accessible_rolls, 43);
    }
}
