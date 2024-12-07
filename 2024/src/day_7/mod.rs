use anyhow::{Context, Result};
use std::collections::VecDeque;

#[derive(Default, Debug, Clone)]
struct Calibration {
    result: usize,
    tmp: usize,
    operands: VecDeque<usize>,
}

#[derive(Debug)]
enum Operation {
    Add,
    Mul,
    Concat,
}

fn parse_input(input: &str) -> Result<Vec<Calibration>> {
    input
        .lines()
        .map(|l| {
            let (result, operands) = l.split_once(": ").context("")?;

            let operands = operands
                .split_whitespace()
                .map(str::parse::<usize>)
                .map(|s| s.unwrap())
                .collect();

            Ok(Calibration {
                result: result.parse()?,
                tmp: 0,
                operands,
            })
        })
        .collect()
}

fn eval_calibration(c: &mut Calibration, allow_concat: bool) -> bool {
    if let Some(o) = c.operands.pop_front() {
        c.tmp = o;
    }
    eval_calibration_inner(&mut (c.clone()), Operation::Add, allow_concat)
        || eval_calibration_inner(&mut (c.clone()), Operation::Mul, allow_concat)
        || (allow_concat
            && eval_calibration_inner(&mut (c.clone()), Operation::Concat, allow_concat))
}

fn eval_calibration_inner(c: &mut Calibration, o: Operation, allow_concat: bool) -> bool {
    if c.result == c.tmp && c.operands.is_empty() {
        return true;
    } else if c.result < c.tmp || c.operands.is_empty() {
        return false;
    }

    c.tmp = match o {
        Operation::Add => c.tmp + c.operands.pop_front().unwrap(),
        Operation::Mul => c.tmp * c.operands.pop_front().unwrap(),
        Operation::Concat => format!("{}{}", c.tmp, c.operands.pop_front().unwrap())
            .parse()
            .unwrap(),
    };

    eval_calibration_inner(&mut (c.clone()), Operation::Add, allow_concat)
        || eval_calibration_inner(&mut (c.clone()), Operation::Mul, allow_concat)
        || (allow_concat
            && eval_calibration_inner(&mut (c.clone()), Operation::Concat, allow_concat))
}

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/src/day_7/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;
    let mut calibrations = parse_input(&input)?;

    let mut valid = 0;
    for c in &mut calibrations.clone() {
        if eval_calibration(c, false) {
            valid += c.result;
        }
    }
    println!("Day 7, Part 1: Valid calibrations: {valid}");

    let mut valid_with_concat = 0;
    for c in &mut calibrations {
        if eval_calibration(c, true) {
            valid_with_concat += c.result;
        }
    }
    println!("Day 7, Part 2: Valid calibrations with concatenation: {valid_with_concat}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn part_one() {
        let mut calibrations = parse_input(INPUT).unwrap();

        let mut valid = 0;
        for c in &mut calibrations {
            if eval_calibration(c, false) {
                valid += c.result;
            }
        }

        assert_eq!(valid, 3749);
    }

    #[test]
    fn part_two() {
        let mut calibrations = parse_input(INPUT).unwrap();

        let mut valid = 0;
        for c in &mut calibrations {
            if eval_calibration(c, true) {
                valid += c.result;
            }
        }

        assert_eq!(valid, 11387);
    }
}
