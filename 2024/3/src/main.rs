use anyhow::{bail, Result};
use regex::Regex;

struct State {
    val: i32,
    mul_active: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            val: 0,
            mul_active: true,
        }
    }
}

enum Op {
    Mul(i32, i32),
    Do,
    Dont,
}

impl Op {
    fn exec(&self, state: &mut State, allowance: bool) {
        match self {
            Self::Mul(a, b) => {
                if !allowance || (allowance && state.mul_active) {
                    state.val += a * b;
                }
            }
            Self::Do => state.mul_active = true,
            Self::Dont => state.mul_active = false,
        }
    }
}

fn parse_input(input: &str) -> Result<Vec<Op>> {
    let re =
        Regex::new("(?<mul>mul\\((?<a>[0-9]+),(?<b>[0-9]+)\\))|(?<do>do\\((?<c>\\s*)(?<d>\\s*)\\))|(?<dont>don't\\((?<e>\\s*)(?<f>\\s*)\\))")?;

    let ops: Result<Vec<_>> = re
        .captures_iter(input)
        .map(|cap| {
            if cap.name("mul").is_some() {
                let (_, [_mul, a, b]) = cap.extract();
                Ok(Op::Mul(a.parse()?, b.parse()?))
            } else if cap.name("do").is_some() {
                Ok(Op::Do)
            } else if cap.name("dont").is_some() {
                Ok(Op::Dont)
            } else {
                bail!("")
            }
        })
        .collect();

    Ok(ops?)
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let ops = parse_input(&input)?;

    let state_one = ops.iter().fold(State::default(), |mut state, op| {
        op.exec(&mut state, false);
        state
    });
    println!("Part 1: {}", state_one.val);

    let state_two = ops.iter().fold(State::default(), |mut state, op| {
        op.exec(&mut state, true);
        state
    });
    println!("Part 2: {}", state_two.val);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    const INPUT_WITH_DO_DONT: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn part_one() {
        let ops = parse_input(INPUT).unwrap();
        let state = ops.iter().fold(State::default(), |mut state, op| {
            op.exec(&mut state, false);
            state
        });
        assert_eq!(state.val, 161);
    }

    #[test]
    fn part_two() {
        let ops = parse_input(INPUT_WITH_DO_DONT).unwrap();
        let state = ops.iter().fold(State::default(), |mut state, op| {
            op.exec(&mut state, true);
            state
        });
        assert_eq!(state.val, 48);
    }
}
