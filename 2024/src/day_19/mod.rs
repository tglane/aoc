use anyhow::{Context, Result};
use std::collections::HashMap;

struct TowelPattern(String);

struct Towel(String);

impl Towel {
    fn as_ref(&self) -> TowelRef {
        TowelRef(self.0.as_str())
    }

    fn ok(&self, pattern: &[TowelPattern]) -> bool {
        let mut cache = HashMap::new();
        self.as_ref().num_arrangements(pattern, &mut cache) != 0
    }

    fn num_arrangements(&self, pattern: &[TowelPattern]) -> usize {
        let mut cache = HashMap::new();
        self.as_ref().num_arrangements(pattern, &mut cache)
    }
}

#[derive(PartialEq, Eq, Hash)]
struct TowelRef<'towel>(&'towel str);

impl<'towel> TowelRef<'towel> {
    fn next_part(&self, pattern: &TowelPattern) -> Option<Self> {
        if self.0.starts_with(pattern.0.as_str()) {
            Some(Self(&self.0[pattern.0.len()..]))
        } else {
            None
        }
    }

    fn num_arrangements(
        &self,
        pattern: &[TowelPattern],
        cache: &mut HashMap<String, usize>,
    ) -> usize {
        if let Some(e) = cache.get(self.0) {
            return *e;
        }

        if self.0.len() == 0 {
            return 1;
        }

        let mut found = 0;
        for pat in pattern {
            if let Some(next) = self.next_part(pat) {
                found += next.num_arrangements(pattern, cache);
            }
        }

        cache.insert(self.0.to_string(), found);
        found
    }
}

fn parse_input(input: &str) -> Result<(Vec<TowelPattern>, Vec<Towel>)> {
    let (pattern, display) = input.split_once("\n\n").context("")?;

    let pattern = pattern
        .split(", ")
        .map(|m| TowelPattern(m.into()))
        .collect();

    let display = display.lines().map(|m| Towel(m.into())).collect();

    Ok((pattern, display))
}

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/src/day_19/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;
    let (towel_pattern, towels) = parse_input(&input)?;

    let count_ok = towels.iter().filter(|t| t.ok(&towel_pattern)).count();
    println!("Day 19, Part 1: Count of valid patterns: {}", count_ok);

    let count_arrangements = towels
        .iter()
        .map(|t| t.num_arrangements(&towel_pattern))
        .sum::<usize>();
    println!(
        "Day 19, Part 2: Count of valid arrangements: {}",
        count_arrangements
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn part_one() {
        let (towel_pattern, towels) = parse_input(INPUT).unwrap();
        let count_ok = towels.iter().filter(|t| t.ok(&towel_pattern)).count();
        assert_eq!(count_ok, 6);
    }

    #[test]
    fn part_two() {
        let (towel_pattern, towels) = parse_input(INPUT).unwrap();
        let num_arrangements = towels
            .iter()
            .map(|t| t.num_arrangements(&towel_pattern))
            .sum::<usize>();
        assert_eq!(num_arrangements, 16);
    }
}
