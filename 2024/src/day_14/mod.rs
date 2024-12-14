use anyhow::{Context, Result};
use regex::Regex;

#[derive(Clone, Debug, Default)]
struct Robot {
    pos: (isize, isize),
    vel: (isize, isize),
}

impl Robot {
    fn steps(&mut self, steps: isize, bounds: &(isize, isize)) {
        let new_x = ((self.pos.0 + steps * self.vel.0) % bounds.0 + bounds.0) % bounds.0;
        let new_y = ((self.pos.1 + steps * self.vel.1) % bounds.1 + bounds.1) % bounds.1;
        self.pos = (new_x, new_y);
    }
}

fn parse_input(input: &str) -> Result<Vec<Robot>> {
    let matcher =
        Regex::new("p=(?<x>-?\\d+),(?<y>-?\\d+) v=(?<vx>-?\\d+),(?<vy>-?\\d+)").context("")?;
    let robots = input
        .lines()
        .map(|l| {
            let cap = matcher.captures(l).context("Failed to capture on line")?;
            let (_, [x, y, vx, vy]) = cap.extract();
            Ok(Robot {
                pos: (x.parse()?, y.parse()?),
                vel: (vx.parse()?, vy.parse()?),
            })
        })
        .collect::<Result<Vec<_>>>();
    Ok(robots?)
}

fn calc_safety_factor(robots: &[Robot], bounds: &(isize, isize)) -> usize {
    let mid_x = bounds.0 / 2;
    let mid_y = bounds.1 / 2;

    let mut quadrants = [0; 4];

    for r in robots {
        if r.pos.0 < mid_x && r.pos.1 < mid_y {
            quadrants[0] += 1;
        } else if r.pos.0 < mid_x && r.pos.1 > mid_y {
            quadrants[1] += 1;
        } else if r.pos.0 > mid_x && r.pos.1 < mid_y {
            quadrants[2] += 1;
        } else if r.pos.0 > mid_x && r.pos.1 > mid_y {
            quadrants[3] += 1;
        }
    }

    quadrants.iter().fold(1, |acc, q| acc * q)
}

fn calc_extended_safety_factor(robots: &[Robot], bounds: &(isize, isize)) -> usize {
    let first_x = bounds.0 / 3;
    let sec_x = (bounds.0 / 3) * 2 + 1;
    let first_y = bounds.1 / 3;
    let sec_y = (bounds.1 / 3) * 2 + 1;

    let mut quadrants = [0; 9];

    for r in robots {
        if r.pos.0 < first_x {
            if r.pos.1 < first_y {
                quadrants[0] += 1;
            } else if r.pos.1 > first_y && r.pos.1 < sec_y {
                quadrants[1] += 1;
            } else if r.pos.1 > sec_y {
                quadrants[2] += 1;
            }
        } else if r.pos.0 < sec_x && r.pos.0 > first_x {
            if r.pos.1 < first_y {
                quadrants[3] += 1;
            } else if r.pos.1 > first_y && r.pos.1 < sec_y {
                quadrants[4] += 1;
            } else if r.pos.1 > sec_y {
                quadrants[5] += 1;
            }
        } else if r.pos.0 > sec_x {
            if r.pos.1 < first_y {
                quadrants[6] += 1;
            } else if r.pos.1 > first_y && r.pos.1 < sec_y {
                quadrants[7] += 1;
            } else if r.pos.1 > sec_y {
                quadrants[8] += 1;
            }
        }
    }

    quadrants.iter().fold(1, |acc, q| acc * q)
}

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/src/day_14/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;
    let mut robots = parse_input(&input)?;
    let bounds = (101, 103);

    for r in &mut robots {
        r.steps(100, &bounds);
    }
    let safety_factor = calc_safety_factor(&robots, &bounds);
    println!("Day 14, Part 1: Safety factor after 100 steps: {safety_factor}");

    // Christmas tree means most robots are cumulated at a single sector.
    // Unfortunately the 4 quadrants from part one are not enought to find the tree for my input so
    // I use 9 quadrants here instead. Then the tree should be formed when the safety factor is
    // lowest
    let mut min_safety_factor = usize::MAX;
    let mut min_safety_factor_after = 100;
    for i in 1..10_000 {
        for r in &mut robots {
            r.steps(1, &bounds);
        }
        let new_safety_factor = calc_extended_safety_factor(&robots, &bounds);
        if min_safety_factor > new_safety_factor {
            min_safety_factor = new_safety_factor;
            min_safety_factor_after = i + 100;
        }
    }
    println!("Day 14, Part 2b: Christmas tree found after: {min_safety_factor_after}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn part_one() {
        let bounds = (11, 7);
        let mut robots = parse_input(INPUT).unwrap();
        for r in &mut robots {
            r.steps(100, &bounds);
        }
        assert_eq!(calc_safety_factor(&robots, &bounds), 12);
    }
}
