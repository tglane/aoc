use anyhow::{Context, Result};

fn calc_fuel(masses: &[usize], fuel_weight: bool) -> usize {
    if !fuel_weight {
        masses.iter().map(|mass| mass / 3 - 2).sum()
    } else {
        masses.iter().map(|mass| calc_fuel_inner(*mass)).sum()
    }
}

fn calc_fuel_inner(mass: usize) -> usize {
    if let Some(fuel) = (mass / 3).checked_sub(2) {
        fuel + calc_fuel_inner(fuel)
    } else {
        0
    }
}

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/src/day_1/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;

    let masses = input
        .lines()
        .map(|l| l.parse::<usize>().context("Invalid mass"))
        .collect::<Result<Vec<_>>>()?;

    let fuel = calc_fuel(&masses, false);
    println!("Day 1, Part 1: Fuel needed: {fuel}");

    let extra_fuel = calc_fuel(&masses, true);
    println!("Day 1, Part 1: Fuel needed: {extra_fuel}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "12
14
1969
100756";

    #[test]
    fn part_one() {
        let masses = INPUT
            .lines()
            .map(|l| l.parse::<usize>().context("Invalid mass"))
            .collect::<Result<Vec<_>>>()
            .unwrap();
        assert_eq!(calc_fuel(&masses, false), 2 + 2 + 654 + 33583);
    }

    #[test]
    fn part_two() {
        let masses = INPUT
            .lines()
            .skip(1)
            .map(|l| l.parse::<usize>().context("Invalid mass"))
            .collect::<Result<Vec<_>>>()
            .unwrap();
        assert_eq!(calc_fuel(&masses, true), 2 + 966 + 50346);
    }
}
