use anyhow::{bail, Context, Result};
use regex::Regex;

#[derive(Debug)]
struct Machine {
    a: (isize, isize),
    b: (isize, isize),
    price: (isize, isize),
}

impl Machine {
    /// Solve system of two equations made up by the machine definition
    ///
    /// ax * a + bx * b = px
    /// ay * a + by * b = py
    ///
    /// Solve for a and b.
    /// The costs for pressing button A are 3, the costs for pressing button B are 1.
    /// So total costs is 3 * a + b (result of this function)
    fn solve(&self, modifier: isize) -> isize {
        let price = (self.price.0 + modifier, self.price.1 + modifier);

        let pybx_pxby = price.1 * self.b.0 - price.0 * self.b.1;
        let bxay_axby = self.b.0 * self.a.1 - self.a.0 * self.b.1;
        let presses_a = pybx_pxby / bxay_axby;

        if pybx_pxby % bxay_axby == 0 {
            let pressed_b = (price.1 - presses_a * self.a.1) / self.b.1;
            (3 * presses_a + pressed_b) as isize
        } else {
            0
        }
    }
}

fn parse_input(input: &str) -> Result<Vec<Machine>> {
    let button_matcher = Regex::new("Button \\w: X\\+(?<x>\\d+), Y\\+(?<y>\\d+)")?;
    let price_matcher = Regex::new("Prize: X=(?<x>\\d+), Y=(?<y>\\d+)")?;

    let machines = input
        .split("\n\n")
        .map(|block| {
            let mut lines = block.lines();

            let a = if let Some(cap) = button_matcher.captures(lines.nth(0).context("")?) {
                let (_, [x, y]) = cap.extract();
                (x.parse()?, y.parse()?)
            } else {
                bail!("Failed to extract Button A");
            };

            let b = if let Some(cap) = button_matcher.captures(lines.nth(0).context("")?) {
                let (_, [x, y]) = cap.extract();
                (x.parse()?, y.parse()?)
            } else {
                bail!("Failed to extract Button B");
            };

            let price = if let Some(cap) = price_matcher.captures(lines.nth(0).context("")?) {
                let (_, [x, y]) = cap.extract();
                (x.parse()?, y.parse()?)
            } else {
                bail!("Failed to extract Price");
            };

            Ok(Machine { a, b, price })
        })
        .collect::<Result<Vec<_>>>();

    Ok(machines?)
}

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/src/day_13/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;
    let machines = parse_input(&input)?;

    let min_costs = machines.iter().map(|m| m.solve(0)).sum::<isize>();
    println!("Day 13, Part 1: Total minimal costs: {min_costs}");

    let min_costs_modified = machines
        .iter()
        .map(|m| m.solve(10000000000000))
        .sum::<isize>();
    println!("Day 13, Part 2: Total minimal costs modified: {min_costs_modified}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn part_one() {
        let machines = parse_input(INPUT).unwrap();
        let costs = machines.iter().map(|m| m.solve(0)).sum::<isize>();
        assert_eq!(costs, 480);
    }

    #[test]
    fn part_two() {
        let machines = parse_input(INPUT).unwrap();
        let _costs = machines
            .iter()
            .map(|m| m.solve(10000000000000))
            .sum::<isize>();
        // No value given by AoC, so just run it and check if we do not get panics
    }
}
