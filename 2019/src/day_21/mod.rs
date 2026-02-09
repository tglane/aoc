use crate::intcode::{IntcodeComputer, RunStatus};
use anyhow::{Context, Result};

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/src/day_21/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;

    // NOT C J
    // AND D J
    // NOT A T
    // OR T J
    // WALK
    let instructions = vec![
        Instruction::Not(ImmutableRegister::C.into(), MutableRegister::Jump),
        Instruction::And(ImmutableRegister::D.into(), MutableRegister::Jump),
        Instruction::Not(ImmutableRegister::A.into(), MutableRegister::Temp),
        Instruction::Or(MutableRegister::Temp.into(), MutableRegister::Jump),
        Instruction::Walk,
    ];
    let mut bot = SpringDroid::new(&input, instructions)?;
    let hull_damage = bot.run()?;
    println!("Day 21, Part 1: Hull damage: {}", hull_damage);

    // NOT C J
    // AND D J
    // AND H J
    // NOT B T
    // AND D T
    // OR T J
    // NOT A T
    // OR T J
    let instructions = vec![
        Instruction::Not(ImmutableRegister::C.into(), MutableRegister::Jump),
        Instruction::And(ImmutableRegister::D.into(), MutableRegister::Jump),
        Instruction::And(ImmutableRegister::H.into(), MutableRegister::Jump),
        Instruction::Not(ImmutableRegister::B.into(), MutableRegister::Temp),
        Instruction::And(ImmutableRegister::D.into(), MutableRegister::Temp),
        Instruction::Or(MutableRegister::Temp.into(), MutableRegister::Jump),
        Instruction::Not(ImmutableRegister::A.into(), MutableRegister::Temp),
        Instruction::Or(MutableRegister::Temp.into(), MutableRegister::Jump),
        Instruction::Run,
    ];
    let mut bot = SpringDroid::new(&input, instructions)?;
    let extended_hull_damage = bot.run()?;
    println!("Day 22, Part 1: Hull damage: {}", extended_hull_damage);

    Ok(())
}

struct SpringDroid {
    computer: IntcodeComputer,
}

impl SpringDroid {
    fn new(program: &str, instructions: Vec<Instruction>) -> Result<Self> {
        let buff = instructions
            .into_iter()
            .fold(Vec::<isize>::new(), |mut buff, inst| {
                buff.append(&mut inst.as_ascii());
                buff
            });
        Ok(Self {
            computer: IntcodeComputer::new(program, buff)?,
        })
    }

    fn run(&mut self) -> Result<isize> {
        loop {
            match self.computer.run()? {
                RunStatus::Halt => {
                    let output = self.computer.consume_output();
                    return output.iter().last().context("No output computed").cloned();
                }
                RunStatus::WaitForInput => anyhow::bail!("No input request expected"),
                RunStatus::OutputValue => (),
            }
        }
    }
}

enum Instruction {
    Walk,
    Run,
    And(Register, MutableRegister),
    Or(Register, MutableRegister),
    Not(Register, MutableRegister),
}

impl Instruction {
    const ASCII_SPACE: isize = 32;
    const ASCII_NEWLINE: isize = 10;

    fn as_ascii(&self) -> Vec<isize> {
        match self {
            Self::Walk => vec![
                'W' as isize,
                'A' as isize,
                'L' as isize,
                'K' as isize,
                Self::ASCII_NEWLINE,
            ],
            Self::Run => vec![
                'R' as isize,
                'U' as isize,
                'N' as isize,
                Self::ASCII_NEWLINE,
            ],
            Self::And(a, b) => {
                vec![
                    'A' as isize,
                    'N' as isize,
                    'D' as isize,
                    Self::ASCII_SPACE,
                    a.as_ascii(),
                    Self::ASCII_SPACE,
                    b.as_ascii(),
                    Self::ASCII_NEWLINE,
                ]
            }
            Self::Or(a, b) => {
                vec![
                    'O' as isize,
                    'R' as isize,
                    Self::ASCII_SPACE,
                    a.as_ascii(),
                    Self::ASCII_SPACE,
                    b.as_ascii(),
                    Self::ASCII_NEWLINE,
                ]
            }
            Self::Not(a, b) => {
                vec![
                    'N' as isize,
                    'O' as isize,
                    'T' as isize,
                    Self::ASCII_SPACE,
                    a.as_ascii(),
                    Self::ASCII_SPACE,
                    b.as_ascii(),
                    Self::ASCII_NEWLINE,
                ]
            }
        }
    }
}

enum Register {
    Mutable(MutableRegister),
    Immutable(ImmutableRegister),
}

impl Register {
    fn as_ascii(&self) -> isize {
        match self {
            Self::Mutable(m) => m.as_ascii(),
            Self::Immutable(i) => i.as_ascii(),
        }
    }
}

impl From<MutableRegister> for Register {
    fn from(value: MutableRegister) -> Self {
        Self::Mutable(value)
    }
}

impl From<ImmutableRegister> for Register {
    fn from(value: ImmutableRegister) -> Self {
        Self::Immutable(value)
    }
}

enum MutableRegister {
    Temp,
    Jump,
}

impl MutableRegister {
    fn as_ascii(&self) -> isize {
        match self {
            Self::Temp => 'T' as isize,
            Self::Jump => 'J' as isize,
        }
    }
}

#[allow(dead_code)]
enum ImmutableRegister {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
}

impl ImmutableRegister {
    fn as_ascii(&self) -> isize {
        match self {
            Self::A => 'A' as isize,
            Self::B => 'B' as isize,
            Self::C => 'C' as isize,
            Self::D => 'D' as isize,
            Self::E => 'E' as isize,
            Self::F => 'F' as isize,
            Self::G => 'G' as isize,
            Self::H => 'H' as isize,
            Self::I => 'I' as isize,
        }
    }
}
