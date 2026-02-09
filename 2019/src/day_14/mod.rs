use anyhow::{Context, Result};
use std::collections::HashMap;

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/src/day_14/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;

    let reactions = parse_input(&input)?;

    let ore_needed = determine_fuel_cost(&reactions, 1);
    println!("Day 14, Part 1: Ore needed to produce on Fuel: {ore_needed}");

    let max_fuel = determinal_fuel_creation(&reactions, 1_000_000_000_000);
    println!("Day 14, Part 2: Max amount of fuel created by 1_000_000_000_000 ore: {max_fuel}",);

    Ok(())
}

fn utilize_leftovers(required: &ReactionPart, leftover: &mut HashMap<String, u64>) -> u64 {
    if let Some(amount_leftover) = leftover.get_mut(&required.resource) {
        if required.amount <= *amount_leftover {
            // More leftover than needed to produce required resource
            *amount_leftover -= required.amount;
            0
        } else {
            // Less leftover than needed to produce required resource
            let remaining_to_produce = required.amount - *amount_leftover;
            leftover.remove(&required.resource);
            remaining_to_produce
        }
    } else {
        // No fitting leftover resources so we need to create all thats needed for the requested
        // resource
        required.amount
    }
}

fn determine_fuel_cost(reactions: &HashMap<String, Reaction>, fuel_needed: u64) -> u64 {
    let mut ore_needed = 0;
    let mut requirements = vec![ReactionPart::new("FUEL", fuel_needed)];

    let mut leftover = HashMap::new();

    while let Some(requirement) = requirements.pop() {
        if requirement.resource == "ORE" {
            ore_needed += requirement.amount;
            continue;
        }

        let remaining_to_produce = utilize_leftovers(&requirement, &mut leftover);
        if remaining_to_produce == 0 {
            // Required resource could be produced entirely from leftovers
            continue;
        }

        let reaction_to_produce = reactions.get(&requirement.resource).unwrap();
        let num_of_reactions =
            (remaining_to_produce as f64 / reaction_to_produce.output.amount as f64).ceil() as u64;

        // Use leftovers
        if let Some(remaining_leftover) =
            (reaction_to_produce.output.amount * num_of_reactions).checked_sub(remaining_to_produce)
            && remaining_leftover > 0
        {
            leftover
                .entry(requirement.resource)
                .and_modify(|n| *n += remaining_leftover)
                .or_insert(remaining_leftover);
        }

        for next_requirement in reaction_to_produce.input.iter() {
            let mut next_requirement = next_requirement.clone();
            next_requirement.amount *= num_of_reactions;
            requirements.push(next_requirement);
        }
    }

    ore_needed
}

fn determinal_fuel_creation(reactions: &HashMap<String, Reaction>, ore_avail: u64) -> u64 {
    // Binary search to determine max fuel that can be produced with available ore
    let cost_one = determine_fuel_cost(reactions, 1);
    let mut lower_bound = ore_avail / cost_one;
    let mut upper_bound = lower_bound * 2;
    while upper_bound - lower_bound > 1 {
        let fuel = (upper_bound + lower_bound) / 2;
        let cost = determine_fuel_cost(reactions, fuel);

        if cost < ore_avail {
            lower_bound = fuel;
        } else {
            upper_bound = fuel;
        }
    }
    lower_bound
}

#[derive(Clone, Debug)]
struct ReactionPart {
    resource: String,
    amount: u64,
}

impl ReactionPart {
    fn new(resource: impl Into<String>, amount: u64) -> Self {
        Self {
            resource: resource.into(),
            amount,
        }
    }
}

impl TryFrom<&str> for ReactionPart {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (amount, name) = value
            .split_once(' ')
            .context("Invalid reaction part format")?;
        Ok(Self {
            resource: name.to_string(),
            amount: amount.parse()?,
        })
    }
}

#[derive(Clone, Debug)]
struct Reaction {
    input: Vec<ReactionPart>,
    output: ReactionPart,
}

impl TryFrom<&str> for Reaction {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (input, output) = value.split_once(" => ").context("")?;
        Ok(Self {
            input: input
                .split(", ")
                .map(ReactionPart::try_from)
                .collect::<Result<Vec<_>>>()?,
            output: ReactionPart::try_from(output)?,
        })
    }
}

fn parse_input(input: &str) -> Result<HashMap<String, Reaction>> {
    input
        .lines()
        .map(|line| {
            let r = Reaction::try_from(line)?;
            Ok((r.output.resource.clone(), r))
        })
        .collect::<Result<HashMap<_, _>>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_M: &str = r#"9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL
"#;

    const INPUT_L: &str = r#"171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX
"#;

    #[test]
    fn part_one() {
        let reactions = parse_input(INPUT_M).unwrap();
        let ore_needed = determine_fuel_cost(&reactions, 1);
        assert_eq!(ore_needed, 165);

        let reactions = parse_input(INPUT_L).unwrap();
        let ore_needed = determine_fuel_cost(&reactions, 1);
        assert_eq!(ore_needed, 2210736);
    }

    #[test]
    fn part_two() {
        let reactions = parse_input(INPUT_L).unwrap();
        let max_fuel = determinal_fuel_creation(&reactions, 1_000_000_000_000);
        assert_eq!(max_fuel, 460664);
    }
}
