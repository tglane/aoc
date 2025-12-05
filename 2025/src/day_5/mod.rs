use crate::Day;
use anyhow::Result;
use std::path::Path;

pub(crate) struct DayFive {
    input: String,
}

impl Day for DayFive {
    fn new<P: AsRef<Path>>(path: P) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            input: std::fs::read_to_string(path)?,
        })
    }

    fn part_one(&self) -> Result<()> {
        let ingredients = Ingredients::try_from(self.input.as_str()).unwrap();
        let fresh = ingredients.fresh_ingredients();
        println!("Day 5 - Part 1: Fresh ingredients: {fresh}");
        Ok(())
    }

    fn part_two(&self) -> Result<()> {
        let ingredients = Ingredients::try_from(self.input.as_str()).unwrap();
        let fresh = ingredients.max_allowed_fresh_ingredients();
        println!("Day 5 - Part 2: Max number of fresh ingredients: {fresh}");
        Ok(())
    }
}

struct Ingredients {
    fresh_ranges: Vec<(usize, usize)>,
    ingredients: Vec<usize>,
}

impl Ingredients {
    fn fresh_ingredients(&self) -> usize {
        self.ingredients
            .iter()
            .filter(|ingredient| self.is_fresh(**ingredient))
            .count()
    }

    fn max_allowed_fresh_ingredients(&self) -> usize {
        self.fresh_ranges
            .iter()
            .map(|(start, end)| end - start + 1)
            .sum()
    }

    fn is_fresh(&self, ingredient: usize) -> bool {
        // It is guaranteed on construction that the ranges are sorted.
        // Therefore we can stop early if the number is lower than the start of the next range.
        for (start, end) in self.fresh_ranges.iter() {
            if ingredient < *start {
                return false;
            } else if ingredient <= *end {
                return true;
            }
        }
        false
    }
}

impl TryFrom<&str> for Ingredients {
    type Error = anyhow::Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let (fresh_ranges, ingredients) = input.split_once("\n\n").ok_or(anyhow::Error::msg(""))?;

        let mut fresh_ranges = fresh_ranges
            .lines()
            .map(|line| line.split_once('-').ok_or(anyhow::Error::msg("")))
            .map(|res| {
                res.and_then(|(start, end)| Ok((start.parse::<usize>()?, end.parse::<usize>()?)))
            })
            .collect::<Result<Vec<_>>>()?;

        fresh_ranges.sort_unstable_by_key(|(start, _end)| *start);

        let mut merged_ranges = Vec::with_capacity(fresh_ranges.len());
        merged_ranges.push(*fresh_ranges.first().ok_or(anyhow::Error::msg(""))?);

        for i in 1..fresh_ranges.len() {
            let prev = merged_ranges.last_mut().unwrap();
            let curr = fresh_ranges[i];

            // Merge if current start is less or equal to end of the previous one
            //
            // Its guaranteed on construction that the ranges are sorted by start value
            if curr.0 <= prev.1 && curr.1 > prev.1 {
                prev.1 = curr.1;
            } else if curr.0 > prev.1 {
                merged_ranges.push(curr);
            }
        }

        let ingredients = ingredients
            .lines()
            .map(|line| line.parse::<usize>().map_err(|_| anyhow::Error::msg("")))
            .collect::<Result<Vec<_>>>()?;

        Ok(Ingredients {
            fresh_ranges: merged_ranges,
            ingredients,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32
"#;

    #[test]
    fn part_one() {
        let ingredients = Ingredients::try_from(INPUT).unwrap();
        let fresh = ingredients.fresh_ingredients();
        assert_eq!(fresh, 3);
    }

    #[test]
    fn part_two() {
        let ingredients = Ingredients::try_from(INPUT).unwrap();
        let fresh = ingredients.max_allowed_fresh_ingredients();
        assert_eq!(fresh, 14);
    }
}
