use anyhow::{bail, Context, Result};
use regex::Regex;

#[derive(Debug)]
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

#[derive(Debug)]
struct ChronospatialComputer {
    reg_a: isize,
    reg_b: isize,
    reg_c: isize,
    inst_ptr: usize,
}

impl ChronospatialComputer {
    fn run(&mut self, ops: &[OpCode]) -> Result<String> {
        let mut out = Vec::<u8>::new();
        // for op in ops {
        while let Some(op) = ops.get(self.inst_ptr) {
            match op {
                OpCode::Adv(operand) => {
                    self.reg_a = self.reg_a >> self.decode_combo_operand(*operand)?;
                }
                OpCode::Bxl(operand) => self.reg_b ^= operand,
                OpCode::Bst(operand) => self.reg_b = self.decode_combo_operand(*operand)? % 8,
                OpCode::Jnz(operand) => {
                    if self.reg_a != 0 {
                        self.inst_ptr = self.decode_combo_operand(*operand)?.try_into()?;
                        continue;
                    }
                }
                OpCode::Bxc(_operand) => self.reg_b ^= self.reg_c,
                OpCode::Out(operand) => {
                    let val = self.decode_combo_operand(*operand)? % 8;
                    println!("Out: {val}");
                    out.push(val.try_into()?);
                }
                OpCode::Bdv(operand) => {
                    self.reg_b = self.reg_a >> self.decode_combo_operand(*operand)?;
                }
                OpCode::Cdv(operand) => {
                    self.reg_b = self.reg_a >> self.decode_combo_operand(*operand)?;
                }
            }

            self.inst_ptr += 2;
        }

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

    let mut parsed_ops = Vec::new();

    let op_iter = ops
        .trim()
        .strip_prefix("Program: ")
        .context("Invalid program sequence")?
        .split(',')
        .map(|s| s.parse::<usize>().context("ops"))
        .collect::<Result<Vec<_>>>()?;

    for w in op_iter.windows(2) {
        parsed_ops.push(OpCode::try_from((w[0], w[1].try_into()?))?);
    }

    // let op_iter = ops
    //     .trim()
    //     .strip_prefix("Program: ")
    //     .context("Invalid program sequence")?
    //     .split(',')
    //     .map(|s| s.parse::<usize>().context("ops"))
    //     .chunks(2);
    // for mut ch in &op_iter {
    //     let code = ch.next().context("")??;
    //     let operand = ch.next().context("")??;
    //     parsed_ops.push(OpCode::try_from((code, operand.try_into()?))?);
    // }

    Ok((computer, parsed_ops))
}

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/src/day_17/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;
    let (mut computer, ops) = parse_input(&input)?;
    println!("ops: {ops:?}");

    let output = computer.run(&ops)?;
    println!("Day 17, Part 1: Output of the program: {output}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    #[test]
    fn part_one() {
        let (mut computer, ops) = parse_input(INPUT).unwrap();
        println!("Ops: {ops:?}");
        let out = computer.run(&ops).unwrap();
        assert_eq!(out, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn part_two() {}
}
