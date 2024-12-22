use anyhow::{Context, Result};
use std::collections::HashMap;

fn parse_input(input: &str) -> Result<Vec<SecretNumber>> {
    input
        .lines()
        .map(|l| {
            l.parse::<usize>()
                .context("Invalid number")
                .map(|n| SecretNumber(n))
        })
        .collect::<Result<Vec<_>>>()
}

#[derive(Clone, Copy, Debug)]
struct SecretNumber(usize);

impl SecretNumber {
    fn mix(&mut self, val: usize) {
        self.0 ^= val;
    }

    fn prune(&mut self) {
        const MOD_OP: usize = 16777216;
        self.0 %= MOD_OP;
    }

    fn price(&self) -> usize {
        self.0 % 10
    }
}

fn next_secret(mut secret: SecretNumber) -> SecretNumber {
    let v = secret.0 * 64;
    secret.mix(v);
    secret.prune();

    let v = secret.0 / 32;
    secret.mix(v);
    secret.prune();

    let v = secret.0 * 2048;
    secret.mix(v);
    secret.prune();

    secret
}

fn calculate_secret(mut secret: SecretNumber, steps: usize) -> SecretNumber {
    for _ in 0..steps {
        secret = next_secret(secret);
    }
    secret
}

fn max_bananas(mut buyers: Vec<SecretNumber>) -> Result<usize> {
    let mut change_map = HashMap::<[isize; 4], usize>::new();

    for secret in buyers.iter_mut() {
        let mut price = secret.price();

        let mut changes = [0_isize; 2000];
        let mut local_change_map = HashMap::<[isize; 4], usize>::new();

        for i in 0..2000 {
            *secret = next_secret(*secret);
            let new_price = secret.price();

            changes[i] =
                TryInto::<isize>::try_into(new_price)? - TryInto::<isize>::try_into(price)?;

            if i >= 3 {
                let ch = changes[i - 3..=i].try_into()?;
                local_change_map.entry(ch).or_insert(new_price);
            }

            price = new_price;
        }

        for (change_seq, bananas) in local_change_map.iter() {
            *change_map.entry(*change_seq).or_insert(0) += bananas;
        }
    }

    change_map.values().max().cloned().context("")
}

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/src/day_22/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;
    let initials = parse_input(&input)?;

    let secret_sum = initials
        .iter()
        .map(|s| calculate_secret(*s, 2000).0)
        .sum::<usize>();
    println!("Day 22, Part 1: Sum of updated secret numbers: {secret_sum}");

    let max_banana_count = max_bananas(initials)?;
    println!("Day 22, Part 2: Max bananas to buy: {max_banana_count}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1
10
100
2024";

    const INPUT_2: &str = "1
2
3
2024";

    #[test]
    fn part_one() {
        let initials = parse_input(INPUT).unwrap();

        let secret_sum = initials
            .iter()
            .map(|s| {
                let new = calculate_secret(*s, 2000);
                println!("s {} -> new {}", s.0, new.0);
                new.0
            })
            .sum::<usize>();
        assert_eq!(secret_sum, 37327623);
    }

    #[test]
    fn part_two() {
        let initials = parse_input(INPUT_2).unwrap();
        let max_banana_count = max_bananas(initials).unwrap();
        assert_eq!(max_banana_count, 23);
    }
}
