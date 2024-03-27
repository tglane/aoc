use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn parse_input(filename: &str) -> Result<Vec<i64>, Error> {
    let reader = BufReader::new(File::open(filename)?);
    let out = reader
        .lines().next().unwrap().unwrap()
        .split(',').map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();
    return Ok(out);
}

fn min_alignment_cost(input: &[i64], cost: &dyn Fn(i64) -> i64) {
    let max_iter = input.iter().max().unwrap();
    let mut min_cost: Option<(i64, i64)> = None;

    for i in 0..*max_iter {
        // Get cost for the movement
        let mut cost_count = 0;
        for num in input.iter() {
            cost_count += cost((*num - i).abs());
        }

        // Check if calculated cost is lowest
        if (min_cost.is_some() && min_cost.unwrap().1 > cost_count) || min_cost.is_none() {
            min_cost = Some((i, cost_count));
        }
    }

    println!("Lowest cost is {} at {}", min_cost.unwrap().1, min_cost.unwrap().0);
}

fn main() {
    let input = parse_input("in.txt")
        .expect("Failed to parse input");

    min_alignment_cost(&input, &|cost| cost);
    min_alignment_cost(&input, &|cost| (cost * (cost + 1)) / 2);
}
