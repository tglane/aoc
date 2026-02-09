use crate::intcode::{IntcodeComputer, RunStatus};
use anyhow::{Context, Result};
use std::{collections::HashSet, io};

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/src/day_25/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;

    let mut bot = Bot::new(&input)?;
    // loop {
    //     let mut inst_buffer = String::new();
    //     io::stdin().read_line(&mut inst_buffer)?;
    //     bot.execute_ascii(inst_buffer)?;
    // }

    guess_password(&mut bot)?;

    Ok(())
}

fn guess_password(bot: &mut Bot) -> Result<()> {
    const ITEM_VEC: [&str; 8] = [
        "food ration",
        "fixed point",
        "weather machine",
        "semiconductor",
        "planetoid",
        "coin",
        "pointer",
        "klein bottle",
    ];

    for bit_vec in 0..256 {
        // for bit_vec in 4..5 {
        let mut bot = bot.clone();

        collect_all_items(&mut bot)?;

        // Drop items according to bit vec
        for i in 0..8 {
            if bit_vec & (1 << i) != 0 {
                bot.execute(Instruction::Drop(Item(ITEM_VEC[i].into())))?;
            }
        }

        // This will halt and thus throw an error once the correct items are carried
        bot.execute(Instruction::Movement(Movement::North))?;
    }
    Ok(())
}

enum Step {
    Move(Movement),
    MoveAndTake((Movement, &'static str)),
}

fn collect_all_items(bot: &mut Bot) -> Result<()> {
    let steps = [
        Step::MoveAndTake((Movement::West, "semiconductor")),
        Step::MoveAndTake((Movement::West, "planetoid")),
        Step::MoveAndTake((Movement::West, "food ration")),
        Step::MoveAndTake((Movement::West, "fixed point")),
        Step::MoveAndTake((Movement::West, "klein bottle")),
        Step::Move(Movement::East),
        Step::Move(Movement::South),
        Step::MoveAndTake((Movement::West, "weather machine")),
        Step::Move(Movement::East),
        Step::Move(Movement::North),
        Step::Move(Movement::East),
        Step::Move(Movement::East),
        Step::Move(Movement::South),
        Step::Move(Movement::South),
        Step::MoveAndTake((Movement::South, "pointer")),
        Step::Move(Movement::North),
        Step::Move(Movement::North),
        Step::MoveAndTake((Movement::East, "coin")),
        Step::Move(Movement::East),
        Step::Move(Movement::North),
        Step::Move(Movement::East),
    ];

    for step in steps {
        match step {
            Step::Move(inst) => bot.execute(Instruction::Movement(inst))?,
            Step::MoveAndTake((inst, item)) => {
                bot.execute(Instruction::Movement(inst))?;
                bot.execute(Instruction::Take(Item(item.into())))?;
            }
        }
    }
    bot.execute(Instruction::Inventory)?;

    Ok(())
}

#[derive(Clone)]
struct Bot {
    computer: IntcodeComputer,
    inventory: HashSet<String>,
}

impl Bot {
    fn new(program: &str) -> Result<Self> {
        let mut this = Self {
            computer: IntcodeComputer::new(program, [])?,
            inventory: HashSet::new(),
        };
        this.run_until_command_required()?;
        Ok(this)
    }

    fn run_until_command_required(&mut self) -> Result<()> {
        loop {
            match self.computer.run()? {
                RunStatus::Halt => anyhow::bail!("Intcode computer halted unexpectedly"),
                RunStatus::WaitForInput => return Ok(()),
                RunStatus::OutputValue => {
                    let out = self
                        .computer
                        .next_output()
                        .context("Intcode computer did not provide output")?;
                    print!("{}", (out as u8) as char);
                }
            }
        }
    }

    fn execute(&mut self, instruction: Instruction) -> Result<()> {
        for ascii_char in instruction.as_ascii() {
            self.computer.push_input(ascii_char);
        }

        self.run_until_command_required()?;

        Ok(())
    }

    fn execute_ascii(&mut self, instruction: String) -> Result<()> {
        for ascii_char in instruction.chars() {
            self.computer.push_input(ascii_char as isize);
        }

        self.run_until_command_required()?;

        Ok(())
    }
}

enum Instruction {
    Movement(Movement),
    Take(Item),
    Drop(Item),
    Inventory,
}

impl Instruction {
    const ASCII_NEWLINE: isize = 10;

    fn as_ascii(&self) -> Vec<isize> {
        let mut buf = match self {
            Self::Movement(m) => m.as_ascii(),
            Self::Take(item) => {
                let mut buf = vec![
                    't' as isize,
                    'a' as isize,
                    'k' as isize,
                    'e' as isize,
                    ' ' as isize,
                ];
                buf.append(&mut item.as_ascii());
                buf
            }
            Self::Drop(item) => {
                let mut buf = vec![
                    'd' as isize,
                    'r' as isize,
                    'o' as isize,
                    'p' as isize,
                    ' ' as isize,
                ];
                buf.append(&mut item.as_ascii());
                buf
            }
            Self::Inventory => {
                vec!['i' as isize, 'n' as isize, 'v' as isize]
            }
        };
        buf.push(Self::ASCII_NEWLINE);
        buf
    }
}

enum Movement {
    North,
    South,
    East,
    West,
}

impl Movement {
    fn as_ascii(&self) -> Vec<isize> {
        match self {
            Self::North => vec![
                'n' as isize,
                'o' as isize,
                'r' as isize,
                't' as isize,
                'h' as isize,
            ],
            Self::South => vec![
                's' as isize,
                'o' as isize,
                'u' as isize,
                't' as isize,
                'h' as isize,
            ],
            Self::East => vec!['e' as isize, 'a' as isize, 's' as isize, 't' as isize],
            Self::West => vec!['w' as isize, 'e' as isize, 's' as isize, 't' as isize],
        }
    }
}

struct Item(String);

impl Item {
    fn as_ascii(&self) -> Vec<isize> {
        self.0.chars().map(|c| c as isize).collect()
    }
}
