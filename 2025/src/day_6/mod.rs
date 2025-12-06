use crate::Day;
use anyhow::Result;
use std::path::Path;

pub(crate) struct DaySix {
    input: String,
}

impl Day for DaySix {
    fn new<P: AsRef<Path>>(path: P) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            input: std::fs::read_to_string(path)?,
        })
    }

    fn part_one(&self) -> Result<()> {
        let problems = parse_horizontal_blocks(&self.input).unwrap();
        let grand_total = problems.iter().fold(0, |acc, p| acc + p.solve());
        println!("Day 6 - Part 1: Grand total: {grand_total}");
        Ok(())
    }

    fn part_two(&self) -> Result<()> {
        let problems = parse_vertical_blocks(&self.input).unwrap();
        let grand_total = problems.iter().fold(0, |acc, p| acc + p.solve());
        println!("Day 6 - Part 2: Updated grand total: {grand_total}");
        Ok(())
    }
}

#[derive(Debug)]
struct MathProblem {
    op: Op,
    nums: Vec<usize>,
}

impl MathProblem {
    fn solve(&self) -> usize {
        self.nums
            .iter()
            .fold(self.op.base_value(), |acc, num| self.op.call(acc, *num))
    }
}

#[derive(Debug)]
enum Op {
    Add,
    Mul,
}

impl Op {
    fn base_value(&self) -> usize {
        match self {
            Self::Add => 0,
            Self::Mul => 1,
        }
    }

    fn call(&self, a: usize, b: usize) -> usize {
        match self {
            Self::Add => a + b,
            Self::Mul => a * b,
        }
    }
}

impl TryFrom<char> for Op {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(Self::Add),
            '*' => Ok(Self::Mul),
            _ => Err(anyhow::Error::msg("Invalid operand")),
        }
    }
}

impl TryFrom<&str> for Op {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "+" => Ok(Self::Add),
            "*" => Ok(Self::Mul),
            _ => Err(anyhow::Error::msg("Invalid operand")),
        }
    }
}

fn parse_horizontal_blocks(input: &str) -> Result<Vec<MathProblem>> {
    let mut lines = input.lines().rev();

    let mut problems = Vec::default();
    for op in lines
        .next()
        .ok_or(anyhow::Error::msg("Empty input"))?
        .split_whitespace()
        .map(Op::try_from)
    {
        problems.push(MathProblem {
            op: op?,
            nums: Vec::default(),
        });
    }

    for line in lines {
        for (i, val_str) in line.split_whitespace().enumerate() {
            problems[i].nums.push(val_str.parse()?);
        }
    }

    Ok(problems)
}

fn parse_vertical_blocks(input: &str) -> Result<Vec<MathProblem>> {
    let lines = input.lines();

    let ops_line = lines
        .clone()
        .last()
        .ok_or(anyhow::Error::msg("Empty input"))?;

    let lines = lines
        .take_while(|line| *line != ops_line)
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut problems = Vec::new();
    let mut curr = Vec::new();
    for i in (0..ops_line.len()).rev() {
        // Append the vertical numbers to a string so that we can later parse it into a number.
        //
        // Thats not optimal but for now it works fine. Maybe try to get rid of the string
        // allocation later.
        let mut s = String::new();
        for line in lines.iter() {
            if line[i] != ' ' {
                s.push(line[i]);
            }
        }
        if let Ok(num) = s.parse::<usize>() {
            curr.push(num);
        }

        if let Some(c) = ops_line.chars().nth(i)
            && c != ' '
        {
            // Start new problem when we encounter an operand
            // Hopefully its guaranteed by the input that the operands are always placed at the
            // beginning of a 'block'
            problems.push(MathProblem {
                op: Op::try_from(c)?,
                nums: std::mem::take(&mut curr),
            });
        }
    }

    Ok(problems)
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
"#;

    #[test]
    fn part_one() {
        let problems = parse_horizontal_blocks(INPUT).unwrap();
        let grand_total = problems.iter().fold(0, |acc, p| acc + p.solve());
        assert_eq!(grand_total, 4277556);
    }

    #[test]
    fn part_two() {
        let problems = parse_vertical_blocks(INPUT).unwrap();
        let grand_total = problems.iter().fold(0, |acc, p| acc + p.solve());
        assert_eq!(grand_total, 3263827);
    }
}
