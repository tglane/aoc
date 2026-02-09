use anyhow::{Context, Result, bail};
use std::collections::VecDeque;

#[derive(Copy, Clone, Debug)]
struct Parameter {
    value: isize,
    mode: ParameterMode,
}

impl Parameter {
    fn new(value: isize, mode: ParameterMode) -> Self {
        Self { value, mode }
    }
}

#[derive(Copy, Clone, Debug)]
enum OpCode {
    Add(Parameter, Parameter, Parameter),
    Mul(Parameter, Parameter, Parameter),
    Input(Parameter),
    Output(Parameter),
    JumpIfTrue(Parameter, Parameter),
    JumpIfFalse(Parameter, Parameter),
    LessThan(Parameter, Parameter, Parameter),
    Equals(Parameter, Parameter, Parameter),
    RelativeBaseOffset(Parameter),
    Halt,
}

impl OpCode {
    fn len(&self) -> usize {
        match self {
            Self::Add(_, _, _) => 4,
            Self::Mul(_, _, _) => 4,
            Self::Input(_) => 2,
            Self::Output(_) => 2,
            Self::JumpIfTrue(_, _) => 3,
            Self::JumpIfFalse(_, _) => 3,
            Self::LessThan(_, _, _) => 4,
            Self::Equals(_, _, _) => 4,
            Self::RelativeBaseOffset(_) => 2,
            Self::Halt => 1,
        }
    }
}

impl OpCode {
    fn decode(mem: &[isize]) -> Result<Self> {
        let mut instruction = mem[0];

        let op_code_val = instruction % 100;

        instruction /= 100;
        let mode_a = ParameterMode::try_from(instruction % 10)?;
        instruction /= 10;
        let mode_b = ParameterMode::try_from(instruction % 10)?;
        instruction /= 10;
        let mode_c = ParameterMode::try_from(instruction % 10)?;

        let op_code = match op_code_val {
            1 => Self::Add(
                Parameter::new(mem[1], mode_a),
                Parameter::new(mem[2], mode_b),
                Parameter::new(mem[3], mode_c),
            ),
            2 => Self::Mul(
                Parameter::new(mem[1], mode_a),
                Parameter::new(mem[2], mode_b),
                Parameter::new(mem[3], mode_c),
            ),
            3 => Self::Input(Parameter::new(mem[1], mode_a)),
            4 => Self::Output(Parameter::new(mem[1], mode_a)),
            5 => Self::JumpIfTrue(
                Parameter::new(mem[1], mode_a),
                Parameter::new(mem[2], mode_b),
            ),
            6 => Self::JumpIfFalse(
                Parameter::new(mem[1], mode_a),
                Parameter::new(mem[2], mode_b),
            ),
            7 => Self::LessThan(
                Parameter::new(mem[1], mode_a),
                Parameter::new(mem[2], mode_b),
                Parameter::new(mem[3], mode_c),
            ),
            8 => Self::Equals(
                Parameter::new(mem[1], mode_a),
                Parameter::new(mem[2], mode_b),
                Parameter::new(mem[3], mode_c),
            ),
            9 => Self::RelativeBaseOffset(Parameter::new(mem[1], mode_a)),
            99 => Self::Halt,
            num => bail!("Invalid op code {}", num),
        };

        Ok(op_code)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ParameterMode {
    Position,  // Parameter is viewed as an address
    Immediate, // Parameter is viewed as a value
    Relative,  // Parameter is viewed as an address/offset from 'relative base'
}

impl TryFrom<isize> for ParameterMode {
    type Error = anyhow::Error;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        let op_code = match value {
            0 => Self::Position,
            1 => Self::Immediate,
            2 => Self::Relative,
            num => bail!("Invalid parameter mode {}", num),
        };
        Ok(op_code)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum RunStatus {
    Halt,
    WaitForInput,
    OutputValue,
}

impl RunStatus {
    pub fn is_halted(&self) -> bool {
        matches!(self, Self::Halt)
    }

    pub fn is_waiting(&self) -> bool {
        matches!(self, Self::WaitForInput)
    }

    pub fn output_ready(&self) -> bool {
        matches!(self, Self::OutputValue)
    }
}

#[derive(Clone, Debug)]
pub struct IntcodeComputer {
    memory: Vec<isize>,
    relative_base: usize,
    inst_ptr: usize,
    input: VecDeque<isize>,
    output: VecDeque<isize>,
}

impl IntcodeComputer {
    const DEFAULT_MEM_VAL: isize = 0;

    pub fn new(mem_repr: &str, input: impl IntoIterator<Item = isize>) -> Result<Self> {
        let memory = mem_repr
            .trim()
            .split(',')
            .map(|s| s.parse::<isize>().context("Invalid number"))
            .collect::<Result<Vec<_>>>()?;

        Ok(Self {
            memory,
            relative_base: 0,
            inst_ptr: 0,
            input: input.into_iter().collect(),
            output: VecDeque::default(),
        })
    }

    pub fn push_input(&mut self, input: isize) {
        self.input.push_back(input);
    }

    pub fn buffered_input(&self) -> usize {
        self.input.len()
    }

    pub fn next_output(&mut self) -> Option<isize> {
        self.output.pop_front()
    }

    pub fn consume_output(&mut self) -> VecDeque<isize> {
        std::mem::take(&mut self.output)
    }

    pub fn buffered_output(&self) -> usize {
        self.output.len()
    }

    #[allow(dead_code)]
    pub fn get_memory(&self) -> &[isize] {
        &self.memory
    }

    pub fn at(&self, idx: impl TryInto<usize>) -> Result<&isize> {
        let idx: usize = idx
            .try_into()
            .map_err(|_| anyhow::Error::msg("Address index failed to convert into usize"))?;
        if idx >= self.memory.len() {
            return Ok(&Self::DEFAULT_MEM_VAL);
        }

        self.memory.get(idx).context("Invalid address")
    }

    pub fn at_mut(&mut self, idx: impl TryInto<usize>) -> Result<&mut isize> {
        let idx: usize = idx
            .try_into()
            .map_err(|_| anyhow::Error::msg("Address index failed to convert into usize"))?;
        if idx >= self.memory.len() {
            self.memory.resize(idx * 2, 0);
        }

        self.memory.get_mut(idx).context("Invalid address")
    }

    fn write_to(&mut self, param: &Parameter, val: isize) -> Result<()> {
        // When writing to an address we expect the parameter to be in position mode to be used as
        // an address.
        match param.mode {
            ParameterMode::Position => *self.at_mut(param.value)? = val,
            ParameterMode::Immediate => anyhow::bail!("Invalid parameter mode for writing"),
            ParameterMode::Relative => {
                let real_addr = param.value + isize::try_from(self.relative_base)?;
                *self.at_mut(real_addr)? = val;
            }
        }
        Ok(())
    }

    fn read_from(&self, param: &Parameter) -> Result<isize> {
        match param.mode {
            ParameterMode::Position => self.at(param.value).cloned(),
            ParameterMode::Immediate => Ok(param.value),
            ParameterMode::Relative => {
                let real_addr = param.value + isize::try_from(self.relative_base)?;
                self.at(real_addr).cloned()
            }
        }
    }

    pub fn run(&mut self) -> Result<RunStatus> {
        loop {
            // println!("Mem: {:?}", &self.memory[..15]);
            // println!(
            //     "Run at inst ptr {:?} = {:?}",
            //     self.inst_ptr, self.memory[self.inst_ptr]
            // );
            let op = OpCode::decode(&self.memory[self.inst_ptr..])?;
            // println!("Decoded inst: {op:?}");

            match op {
                OpCode::Add(ref a, ref b, ref out) => {
                    self.write_to(out, self.read_from(a)? + self.read_from(b)?)?;

                    self.inst_ptr += op.len();
                }
                OpCode::Mul(ref a, ref b, ref out) => {
                    self.write_to(out, self.read_from(a)? * self.read_from(b)?)?;

                    self.inst_ptr += op.len();
                }
                OpCode::Input(ref store) => {
                    if let Some(input_val) = self.input.pop_front() {
                        self.write_to(store, input_val)?;

                        self.inst_ptr += op.len();
                    } else {
                        return Ok(RunStatus::WaitForInput);
                    }
                }
                OpCode::Output(ref load) => {
                    let out_val = self.read_from(load)?;
                    self.output.push_back(out_val);

                    self.inst_ptr += op.len();

                    return Ok(RunStatus::OutputValue);
                }
                OpCode::JumpIfTrue(ref comp, ref jump_to) => {
                    if self.read_from(comp)? != 0 {
                        self.inst_ptr = self.read_from(jump_to)?.try_into()?;
                    } else {
                        self.inst_ptr += op.len();
                    }
                }
                OpCode::JumpIfFalse(ref comp, ref jump_to) => {
                    if self.read_from(comp)? == 0 {
                        self.inst_ptr = self.read_from(jump_to)?.try_into()?;
                    } else {
                        self.inst_ptr += op.len();
                    }
                }
                OpCode::LessThan(ref a, ref b, ref out) => {
                    let a = self.read_from(a)?;
                    let b = self.read_from(b)?;

                    if a < b {
                        self.write_to(out, 1)?;
                    } else {
                        self.write_to(out, 0)?;
                    }

                    self.inst_ptr += op.len();
                }
                OpCode::Equals(ref a, ref b, ref out) => {
                    let a = self.read_from(a)?;
                    let b = self.read_from(b)?;

                    if a == b {
                        self.write_to(out, 1)?;
                    } else {
                        self.write_to(out, 0)?;
                    }

                    self.inst_ptr += op.len();
                }
                OpCode::RelativeBaseOffset(ref relative_base_mod) => {
                    let new_relative_base =
                        isize::try_from(self.relative_base)? + self.read_from(relative_base_mod)?;
                    self.relative_base = new_relative_base.try_into()?;

                    self.inst_ptr += op.len();
                }
                OpCode::Halt => return Ok(RunStatus::Halt),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relative_base_mod() {
        let mut computer = IntcodeComputer {
            memory: vec![109, -7, 99],
            relative_base: 50,
            inst_ptr: 0,
            input: Default::default(),
            output: Default::default(),
        };
        computer.run().unwrap();
        assert_eq!(computer.relative_base, 43);

        let mut computer = IntcodeComputer {
            memory: vec![109, 19, 204, -34, 99],
            relative_base: 2000,
            inst_ptr: 0,
            input: Default::default(),
            output: Default::default(),
        };
        computer.run().unwrap();
        assert_eq!(computer.relative_base, 2019);
    }
}
