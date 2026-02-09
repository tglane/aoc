use crate::intcode::IntcodeComputer;
use anyhow::{Context, Result};
use std::{collections::HashMap, fmt::Display};

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/src/day_11/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;

    // All tiles black at the beginning
    // Input color of current tile to computer:
    //      black 0
    //      white 1
    // Outputs two values:
    //      1. color to paint the tile in (0 b; 1 w)
    //      2. direction of the robot to turn (0 -> left 90 deg; 1 -> right 90 deg)
    // After turn the robot moves forward one tile

    let mut robot = PaintingRobot::new(&input, Color::Black)?;
    robot.run_robot()?;
    let painted_tiles = robot.tiles_painted.len();
    println!("Day 11, Part 1: Number of painted tiles: {}", painted_tiles);

    let mut robot = PaintingRobot::new(&input, Color::White)?;
    robot.run_robot()?;
    println!("Day 11, Part 2: Painted tiles:");
    robot.print_tiles_painted();

    Ok(())
}

#[derive(Clone, Copy, Debug)]
enum Color {
    Black,
    White,
}

impl From<Color> for isize {
    fn from(value: Color) -> Self {
        match value {
            Color::Black => 0,
            Color::White => 1,
        }
    }
}

impl TryFrom<isize> for Color {
    type Error = anyhow::Error;

    fn try_from(value: isize) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Black),
            1 => Ok(Self::White),
            v => anyhow::bail!("Invalid color code: {v}"),
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Black => write!(f, "#"),
            Self::White => write!(f, "."),
        }
    }
}

struct PaintingRobot {
    computer: IntcodeComputer,
    tiles_painted: HashMap<(isize, isize), Color>,
    pos: (isize, isize),
    rotation: i32, // Facing upwards; 0 degrees (towards higher y position)
}

impl PaintingRobot {
    fn new(program: &str, start_field: Color) -> Result<Self> {
        Ok(Self {
            computer: IntcodeComputer::new(program, [start_field.into()])?,
            tiles_painted: HashMap::new(),
            pos: (0, 0),
            rotation: 0,
        })
    }

    fn run_robot(&mut self) -> Result<()> {
        while self.computer.run()?.is_waiting() {
            // Get output to process robot movement
            let color_to_paint = self
                .computer
                .next_output()
                .map(Color::try_from)
                .context("Expecting output after program ran")??;

            self.tiles_painted
                .entry(self.pos)
                .and_modify(|curr_color| *curr_color = color_to_paint)
                .or_insert(color_to_paint);

            let next_move = self
                .computer
                .next_output()
                .context("Expecting output after program ran")?;
            match next_move {
                0 => {
                    self.rotation -= 90;
                    if self.rotation < 0 {
                        self.rotation += 360;
                    }
                }
                1 => self.rotation = (self.rotation + 90) % 360,
                v => anyhow::bail!("Invalid move encoding: {v}"),
            }

            self.make_move();

            if let Some(curr_color) = self.tiles_painted.get(&self.pos) {
                self.computer.push_input((*curr_color).into());
            } else {
                self.computer.push_input(Color::Black.into());
            }
        }

        Ok(())
    }

    fn print_tiles_painted(&self) {
        let min = self
            .tiles_painted
            .iter()
            .fold((isize::MAX, isize::MAX), |mut min, (pos, _)| {
                min.0 = std::cmp::min(min.0, pos.0);
                min.1 = std::cmp::min(min.1, pos.1);
                min
            });
        let max = self
            .tiles_painted
            .iter()
            .fold((0_isize, 0_isize), |mut max, (pos, _)| {
                max.0 = std::cmp::max(max.0, pos.0);
                max.1 = std::cmp::max(max.1, pos.1);
                max
            });

        for y in min.1 - 1..=max.1 + 1 {
            for x in min.0 - 1..=max.0 + 1 {
                if let Some(pos) = self.tiles_painted.get(&(x, y)) {
                    print!("{}", pos);
                } else {
                    print!("{}", Color::Black);
                }
            }
            println!();
        }
    }

    fn make_move(&mut self) {
        match self.rotation {
            0 | 360 => self.pos.1 += 1,
            90 => self.pos.0 += 1,
            180 => self.pos.1 -= 1,
            270 => self.pos.0 -= 1,
            _ => (),
        }
    }
}
