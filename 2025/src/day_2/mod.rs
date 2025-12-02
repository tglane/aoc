use crate::Day;
use anyhow::Result;
use std::path::Path;

pub(crate) struct DayTwo {
    input: String,
}

impl Day for DayTwo {
    fn new<P: AsRef<Path>>(path: P) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            input: std::fs::read_to_string(path)?,
        })
    }

    fn part_one(&self) -> Result<()> {
        let invalid_sum = parse_input(&self.input)?
            .into_iter()
            .map(|r| {
                r.into_iter()
                    .filter(|i| !i.valid())
                    .map(|i| i.0)
                    .sum::<usize>()
            })
            .sum::<usize>();
        println!("Day 2 - Part 1: Sum of invalid IDs: {invalid_sum}");
        Ok(())
    }

    fn part_two(&self) -> Result<()> {
        let invalid_sum = parse_input(&self.input)?
            .into_iter()
            .map(|r| {
                r.into_iter()
                    .filter(|i| !i.valid_complex())
                    .map(|i| i.0)
                    .sum::<usize>()
            })
            .sum::<usize>();
        println!("Day 2 - Part 2: Sum of complex invalid IDs: {invalid_sum}");
        Ok(())
    }
}

#[derive(Debug)]
struct IdRange {
    start: Id,
    end: Id,
}

impl IntoIterator for IdRange {
    type Item = Id;
    type IntoIter = IdRangeIter;

    fn into_iter(self) -> Self::IntoIter {
        IdRangeIter {
            curr: self.start,
            end: self.end.next(),
        }
    }
}

struct IdRangeIter {
    curr: Id,
    end: Id,
}

impl Iterator for IdRangeIter {
    type Item = Id;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr == self.end {
            return None;
        }

        let yielded = self.curr;
        self.curr = self.curr.next();
        Some(yielded)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Id(usize);

impl Id {
    fn valid(&self) -> bool {
        let s = self.0.to_string();

        if s.len() % 2 == 0 {
            let (a, b) = s.split_at(s.len() / 2);

            let a = a.parse::<usize>().unwrap();
            let b = b.parse::<usize>().unwrap();

            return a != b;
        }

        return true;
    }

    fn valid_complex(&self) -> bool {
        let s = self.0.to_string();

        if s.len() == 1 {
            return true;
        }

        let mut parts = 2;

        while parts != s.len() {
            // Check parts parts to be equal
            let part_len = s.len() / parts;
            let mut chunks = s.as_bytes().chunks(part_len);
            if chunks.all(|c| c == &s.as_bytes()[..part_len]) {
                return false;
            }

            parts += 1;
        }

        // Here we need to check if every single char is the same
        if s.chars().all(|c| Some(c) == s.chars().nth(0)) {
            return false;
        }

        return true;
    }

    fn next(self) -> Id {
        Id(self.0 + 1)
    }
}

impl<N> From<N> for Id
where
    N: Into<usize>,
{
    fn from(value: N) -> Self {
        Self(value.into())
    }
}

fn parse_input(input: &str) -> Result<Vec<IdRange>> {
    input
        .split(",")
        .map(|s| {
            let (a, b) = s
                .split_whitespace()
                .next()
                .ok_or(anyhow::Error::msg("Empty ID range"))?
                .split_once("-")
                .ok_or(anyhow::Error::msg("Malformed range"))?;
            Ok(IdRange {
                start: Id(a.parse()?),
                end: Id(b.parse()?),
            })
        })
        .collect::<Result<Vec<_>>>()
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;

    #[test]
    fn part_one() {
        let invalid_sum = parse_input(INPUT)
            .unwrap()
            .into_iter()
            .map(|r| {
                r.into_iter()
                    .filter(|i| !i.valid())
                    .map(|i| i.0)
                    .sum::<usize>()
            })
            .sum::<usize>();
        assert_eq!(invalid_sum, 1227775554);
    }

    #[test]
    fn part_two() {
        let invalid_complex_sum = parse_input(INPUT)
            .unwrap()
            .into_iter()
            .map(|r| {
                r.into_iter()
                    .filter(|i| !i.valid_complex())
                    .map(|i| i.0)
                    .sum::<usize>()
            })
            .sum::<usize>();
        assert_eq!(invalid_complex_sum, 4174379265);
    }
}
