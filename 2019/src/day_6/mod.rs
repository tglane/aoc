use anyhow::{Context, Result};
use std::collections::{HashMap, VecDeque};

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/src/day_6/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;

    let orbits = parse_input(&input).unwrap();
    let checksum = orbit_count_checksum(&orbits);
    println!("Day 6, Part 1: Orbit count checksum: {checksum}");

    let min_transfers = min_orbit_transfers(&orbits, "YOU".into(), "SAN".into());
    println!("Day 6, Part 2: Minimum amount of transfers to Sante: {min_transfers:?}");

    Ok(())
}

fn parse_input(input: &str) -> Result<HashMap<String, Vec<String>>> {
    let mut orbits = HashMap::<String, Vec<String>>::new();
    for orbit in input.lines() {
        let (center, orbiter) = orbit.split_once(')').context("")?;
        orbits
            .entry(center.to_string())
            .and_modify(|list| list.push(orbiter.to_string()))
            .or_insert(vec![orbiter.to_string()]);
    }
    Ok(orbits)
}

fn orbit_count_checksum(orbits: &HashMap<String, Vec<String>>) -> usize {
    // Reverse
    let mut orbiting = HashMap::<String, String>::new();
    for (center, orbiter) in orbits.iter() {
        for o in orbiter {
            orbiting.insert(o.to_string(), center.to_string());
        }
    }

    let mut orbit_sum = 0;
    let mut queue = orbiting.values().collect::<VecDeque<_>>();
    while let Some(center) = queue.pop_front() {
        orbit_sum += 1;
        if let Some(next_center) = orbiting.get(center) {
            queue.push_back(next_center);
        }
    }

    orbit_sum
}

fn min_orbit_transfers(
    orbits: &HashMap<String, Vec<String>>,
    mut start: String,
    mut end: String,
) -> Option<usize> {
    // Reverse
    let mut orbiting = HashMap::<String, String>::new();
    for (center, orbiter) in orbits.iter() {
        for o in orbiter {
            orbiting.insert(o.to_string(), center.to_string());
        }
    }

    let mut start_orbiting = vec![start.clone()];
    while let Some(next) = orbiting.get(&start) {
        start_orbiting.push(next.clone());
        start = next.clone();
    }

    let mut end_orbiting = vec![end.clone()];
    while let Some(next) = orbiting.get(&end) {
        end_orbiting.push(next.clone());
        end = next.clone();
    }

    for (s, start) in start_orbiting.iter().enumerate() {
        for (e, end) in end_orbiting.iter().enumerate() {
            if start == end {
                return Some(s + e - 2);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = r#"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
"#;

    const INPUT2: &str = r#"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN
"#;

    #[test]
    fn part_one() {
        let orbits = parse_input(INPUT1).unwrap();
        let checksum = orbit_count_checksum(&orbits);
        assert_eq!(checksum, 42);
    }

    #[test]
    fn part_two() {
        let orbits = parse_input(INPUT2).unwrap();
        let transfers = min_orbit_transfers(&orbits, "YOU".into(), "SAN".into()).unwrap();
        assert_eq!(transfers, 4);
    }
}
