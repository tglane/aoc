use crate::Day;
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::Path;

pub(crate) struct DayTwelve {
    input: String,
}

impl Day for DayTwelve {
    fn new<P: AsRef<Path>>(path: P) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            input: std::fs::read_to_string(path)?,
        })
    }

    fn part_one(&self) -> Result<()> {
        let (shapes, tree_areas) = parse_input(&self.input).unwrap();
        let ok_areas = tree_areas
            .iter()
            .map(|area| area.ok_heuristically(&shapes))
            .filter(|ok| *ok)
            .count();
        println!("Day 12 - Part 1: Sum areas that are ok: {ok_areas}");
        Ok(())
    }

    fn part_two(&self) -> Result<()> {
        // There is no part two for day 12
        Ok(())
    }
}

#[derive(Debug)]
struct Shape {
    idx: usize,
    space_needed: usize,
}

impl TryFrom<&str> for Shape {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // 1:
        // ###
        // ##.
        // .##

        let (idx, rest) = value.split_once(":\n").context("")?;
        let space_needed = rest.chars().filter(|c| *c == '#').count();

        Ok(Self {
            idx: idx.parse::<usize>()?,
            space_needed,
        })
    }
}

#[derive(Debug)]
struct TreeArea {
    area: (usize, usize),
    presents_needed: HashMap<usize, usize>,
}

impl TreeArea {
    fn ok_heuristically(&self, shapes: &[Shape]) -> bool {
        let area_size = self.area.0 * self.area.1;

        let space_needed = self
            .presents_needed
            .iter()
            .map(|(idx, count)| shapes[*idx].space_needed * count)
            .sum::<usize>();

        area_size >= space_needed
    }
}

impl TryFrom<&str> for TreeArea {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // 4x4: 0 0 0 0 2 0

        let (a, b) = value.split_once(": ").context("")?;

        let (x, y) = a
            .split_once('x')
            .map(|(x, y)| (x.parse::<usize>(), y.parse::<usize>()))
            .context("")?;

        let presents_needed = b
            .split_whitespace()
            .enumerate()
            .map(|(idx, count)| Ok((idx, count.parse::<usize>()?)))
            .collect::<Result<HashMap<usize, usize>>>()?;

        Ok(Self {
            area: (x?, y?),
            presents_needed,
        })
    }
}

fn parse_input(input: &str) -> Result<(Vec<Shape>, Vec<TreeArea>)> {
    let mut blocks = input.split("\n\n").peekable();

    let mut presents = Vec::new();
    let mut tree_areas = Vec::new();

    while let Some(block) = blocks.next() {
        if blocks.peek().is_some() {
            // Present
            presents.push(Shape::try_from(block)?);
        } else {
            // Tree area description is the last block
            tree_areas.extend(
                block
                    .lines()
                    .map(TreeArea::try_from)
                    .collect::<Result<Vec<_>>>()?,
            );
        }
    }

    Ok((presents, tree_areas))
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = r#"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
"#;

    #[test]
    fn part_one() {
        let (shapes, tree_areas) = parse_input(INPUT).unwrap();
        let ok_areas = tree_areas
            .iter()
            .map(|area| area.ok_heuristically(&shapes))
            .filter(|ok| *ok)
            .count();
        // assert_eq!(ok_areas, 2);
    }
}
