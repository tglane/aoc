use std::fmt::Display;

use anyhow::{bail, Context, Result};
use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
enum OpCode {
    Adv(isize), // 0
    Bxl(isize), // 1
    Bst(isize), // 2
    Jnz(isize), // 3
    Bxc(isize), // 4
    Out(isize), // 5
    Bdv(isize), // 6
    Cdv(isize), // 7
}

impl TryFrom<(usize, isize)> for OpCode {
    type Error = anyhow::Error;
    fn try_from(value: (usize, isize)) -> std::result::Result<Self, Self::Error> {
        match value.0 {
            0 => Ok(Self::Adv(value.1)),
            1 => Ok(Self::Bxl(value.1)),
            2 => Ok(Self::Bst(value.1)),
            3 => Ok(Self::Jnz(value.1)),
            4 => Ok(Self::Bxc(value.1)),
            5 => Ok(Self::Out(value.1)),
            6 => Ok(Self::Bdv(value.1)),
            7 => Ok(Self::Cdv(value.1)),
            n => bail!("Invalid op code {}", n),
        }
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            Self::Adv(value) => format!("Adv (0) -> {value}"),
            Self::Bxl(value) => format!("Bxl (1) -> {value}"),
            Self::Bst(value) => format!("Bst (2) -> {value}"),
            Self::Jnz(value) => format!("Jnz (3) -> {value}"),
            Self::Bxc(value) => format!("Bxc (4) -> {value}"),
            Self::Out(value) => format!("Out (5) -> {value}"),
            Self::Bdv(value) => format!("Bdv (6) -> {value}"),
            Self::Cdv(value) => format!("Cdv (7) -> {value}"),
        };
        write!(f, "{out}")
    }
}

#[derive(Clone, Debug)]
struct ChronospatialComputer {
    reg_a: isize,
    reg_b: isize,
    reg_c: isize,
    inst_ptr: usize,
}

impl ChronospatialComputer {
    fn reset(&mut self, a: isize, b: isize, c: isize) {
        self.reg_a = a;
        self.reg_b = b;
        self.reg_c = c;
        self.inst_ptr = 0;
    }

    fn run(&mut self, ops: &[OpCode]) -> Result<String> {
        let mut out = Vec::<u8>::new();
        while let Some(op) = ops.get(self.inst_ptr) {
            match op {
                OpCode::Adv(operand) => {
                    self.reg_a = self.reg_a >> self.decode_combo_operand(*operand)?;
                }
                OpCode::Bxl(operand) => self.reg_b ^= operand,
                OpCode::Bst(operand) => {
                    self.reg_b = ((self.decode_combo_operand(*operand)? % 8) + 8) % 8
                }
                OpCode::Jnz(operand) => {
                    if self.reg_a != 0 {
                        self.inst_ptr = self.decode_combo_operand(*operand)?.try_into()?;
                        continue;
                    }
                }
                OpCode::Bxc(_operand) => self.reg_b ^= self.reg_c,
                OpCode::Out(operand) => {
                    let val = ((self.decode_combo_operand(*operand)? % 8) + 8) % 8;
                    println!("Out: {val}");
                    out.push(val.try_into()?);
                }
                OpCode::Bdv(operand) => {
                    self.reg_b = self.reg_a >> self.decode_combo_operand(*operand)?;
                }
                OpCode::Cdv(operand) => {
                    self.reg_c = self.reg_a >> self.decode_combo_operand(*operand)?;
                }
            }

            self.inst_ptr += 2;
        }

        // Output should be a string joined by ,
        let out = out
            .into_iter()
            .enumerate()
            .fold(String::new(), |mut s, (idx, it)| {
                if idx == 0 {
                    s.push_str(&format!("{it}"));
                } else {
                    s.push_str(&format!(",{it}"));
                }
                s
            });
        Ok(out)
    }

    fn decode_combo_operand(&self, operand: isize) -> Result<isize> {
        match operand {
            0..=3 => Ok(operand),
            4 => Ok(self.reg_a),
            5 => Ok(self.reg_b),
            6 => Ok(self.reg_c),
            _ => bail!("Invalid combo operand"),
        }
    }
}

fn calc_a_reg_replicating(mut comp: ChronospatialComputer, ops: &[OpCode]) -> Result<isize> {
    let (mut a, b, c) = (0, comp.reg_b, comp.reg_c);
    for i in (0..ops.len()).rev() {
        a <<= 3;
        comp.reset(a, b, c);
        while parse_op_codes(&comp.run(&ops)?)? != &ops[i..] {
            a += 1;
            comp.reset(a, b, c);
        }
    }
    Ok(a)
}

fn parse_op_codes(input: &str) -> Result<Vec<OpCode>> {
    if input.is_empty() {
        return Ok(Vec::default());
    }

    let op_iter = input
        .split(',')
        .map(|s| s.parse::<usize>().context("ops"))
        .collect::<Result<Vec<_>>>()?;

    let mut parsed_ops = Vec::new();
    for w in op_iter.windows(2) {
        parsed_ops.push(OpCode::try_from((w[0], w[1].try_into()?))?);
    }
    Ok(parsed_ops)
}

fn parse_input(input: &str) -> Result<(ChronospatialComputer, Vec<OpCode>)> {
    let (registers, ops) = input.split_once("\n\n").context("")?;

    let register_re = Regex::new("Register [ABC]: (?<num>[0-9]+)")?;
    let mut captures = register_re.captures_iter(registers);

    let reg_a = captures
        .next()
        .and_then(|cap| {
            let (_, [num]) = cap.extract();
            num.parse::<isize>().ok()
        })
        .context("a")?;
    let reg_b = captures
        .next()
        .and_then(|cap| {
            let (_, [num]) = cap.extract();
            num.parse::<isize>().ok()
        })
        .context("b")?;
    let reg_c = captures
        .next()
        .and_then(|cap| {
            let (_, [num]) = cap.extract();
            num.parse::<isize>().ok()
        })
        .context("c")?;

    let computer = ChronospatialComputer {
        reg_a,
        reg_b,
        reg_c,
        inst_ptr: 0,
    };

    let parsed_ops = parse_op_codes(
        ops.trim()
            .strip_prefix("Program: ")
            .context("Invalid program sequence")?,
    )?;

    Ok((computer, parsed_ops))
}

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/src/day_17/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;
    let (mut computer, ops) = parse_input(&input)?;
    let computer_backup = computer.clone();

    let output = computer.run(&ops)?;
    println!("Day 17, Part 1: Output of the program: {output}");

    let a_val_replicating = calc_a_reg_replicating(computer_backup, &ops)?;
    println!("Day 17, Part 2: Value for register a to creating self replicating output: {a_val_replicating}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const INPUT_2: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn part_one() {
        let (mut computer, ops) = parse_input(INPUT).unwrap();
        let out = computer.run(&ops).unwrap();
        assert_eq!(out, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn part_two() {
        let (computer, ops) = parse_input(INPUT_2).unwrap();
        let a_val = calc_a_reg_replicating(computer, &ops).unwrap();
        assert_eq!(a_val, 117440);
    }
}
