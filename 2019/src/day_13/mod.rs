use crate::intcode::{IntcodeComputer, RunStatus};
use anyhow::{Context, Result};
use std::cmp::Ordering;
use std::collections::HashMap;

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/src/day_13/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;

    let mut arcade = ArcadeCabinett::new(&input)?;
    arcade.run()?;
    let num_block_tiles = arcade
        .tiles
        .iter()
        .filter(|(_, id)| matches!(id, TileId::Block))
        .count();
    println!("Day 13, Part 1: Number of block tiles: {}", num_block_tiles);

    let mut arcade = ArcadeCabinett::new(&input)?;
    arcade.insert_coins(1);
    arcade.run()?;
    println!("Day 11, Part 2: Score: {}", arcade.segment_display);

    Ok(())
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum TileId {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}

impl TryFrom<isize> for TileId {
    type Error = anyhow::Error;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Empty),
            1 => Ok(Self::Wall),
            2 => Ok(Self::Block),
            3 => Ok(Self::HorizontalPaddle),
            4 => Ok(Self::Ball),
            v => anyhow::bail!("Invalid TileId: {v}"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum JoystickMove {
    Neutral,
    Left,
    Right,
}

impl From<JoystickMove> for isize {
    fn from(value: JoystickMove) -> Self {
        match value {
            JoystickMove::Neutral => 0,
            JoystickMove::Left => -1,
            JoystickMove::Right => 1,
        }
    }
}

struct ArcadeCabinett {
    computer: IntcodeComputer,
    tiles: HashMap<(isize, isize), TileId>,
    segment_display: isize,
    output_cache: Vec<isize>,
}

impl ArcadeCabinett {
    const COIN_COUNT_ADDR: usize = 0;

    fn new(program: &str) -> Result<Self> {
        Ok(Self {
            computer: IntcodeComputer::new(program, [])?,
            tiles: HashMap::new(),
            segment_display: 0,
            output_cache: Vec::new(),
        })
    }

    fn insert_coins(&mut self, coins: isize) {
        *self
            .computer
            .at_mut(Self::COIN_COUNT_ADDR)
            .expect("Valid constant address") += coins;
    }

    fn input_joystick(&mut self, joystick_move: JoystickMove) {
        self.computer.push_input(joystick_move.into());
    }

    fn run(&mut self) -> Result<()> {
        loop {
            let status = self.computer.run()?;
            match status {
                RunStatus::WaitForInput => {
                    // Find paddle
                    let paddle = self
                        .tiles
                        .iter()
                        .find_map(|(pos, tile)| {
                            if *tile == TileId::HorizontalPaddle {
                                Some(pos.0)
                            } else {
                                None
                            }
                        })
                        .context("Failed to detect paddle")?;

                    // Find ball
                    let ball = self
                        .tiles
                        .iter()
                        .find_map(|(pos, tile)| {
                            if *tile == TileId::Ball {
                                Some(pos.0)
                            } else {
                                None
                            }
                        })
                        .context("Failed to detect ball")?;

                    // Move paddle towards ball
                    match ball.cmp(&paddle) {
                        Ordering::Equal => {
                            self.input_joystick(JoystickMove::Neutral);
                        }
                        Ordering::Less => {
                            self.input_joystick(JoystickMove::Left);
                        }
                        Ordering::Greater => {
                            self.input_joystick(JoystickMove::Right);
                        }
                    }
                }
                RunStatus::OutputValue => {
                    self.output_cache.push(self.computer.next_output().unwrap());
                    if self.output_cache.len() == 3 {
                        match (self.output_cache[0], self.output_cache[1]) {
                            (-1, 0) => {
                                self.segment_display = self.output_cache[2];
                            }
                            pos => {
                                let tile_id = TileId::try_from(self.output_cache[2])?;
                                self.tiles.insert(pos, tile_id);
                            }
                        }
                        self.output_cache.clear();
                    }
                }
                RunStatus::Halt => break,
            }
        }

        Ok(())
    }
}
