use anyhow::Result;
use std::collections::HashMap;

fn parse_input(input: &str) -> Result<HashMap<u64, usize>> {
    Ok(input.trim().split(' ').map(|s| s.parse::<u64>()).fold(
        HashMap::new(),
        |mut map, parse_res| {
            if let Ok(k) = parse_res {
                map.entry(k).and_modify(|cnt| *cnt += 1).or_insert(1);
            }
            map
        },
    ))
}

fn split_in_half(n: u64) -> (u64, u64) {
    let div = 10u64.pow((n.ilog10() + 1) / 2);
    (n / div, n % div)
}

fn blink(stones: &HashMap<u64, usize>) -> HashMap<u64, usize> {
    let mut next = HashMap::with_capacity(stones.capacity());
    for (stone, cnt) in stones {
        if *stone == 0 {
            *next.entry(1).or_default() += cnt;
        } else if stone.ilog10() % 2 == 1 {
            // Number of digits is even
            let (first, sec) = split_in_half(*stone);
            *next.entry(first).or_insert(0) += cnt;
            *next.entry(sec).or_insert(0) += cnt;
        } else {
            *next.entry(*stone * 2024).or_default() += cnt;
        }
    }
    next
}

fn stones_after_blinks(stones: &HashMap<u64, usize>, blinks: usize) -> usize {
    let mut next = blink(stones);
    for _ in 1..blinks {
        next = blink(&next);
    }
    next.iter().map(|(_, cnt)| cnt).sum()
}

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/src/day_11/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;
    let stones = parse_input(&input)?;

    let num_stones = stones_after_blinks(&stones, 25);
    println!("Day 11, Part 1: Number of stones after 25 blinks: {num_stones}");

    let num_stones = stones_after_blinks(&stones, 75);
    println!("Day 11, Part 2: Number of stones after 75 blinks: {num_stones}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "125 17";

    #[test]
    fn part_one() {
        let stones = parse_input(INPUT).unwrap();
        let num_stones = stones_after_blinks(&stones, 25);
        assert_eq!(num_stones, 55312);
    }
}
