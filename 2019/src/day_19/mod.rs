use crate::intcode::{IntcodeComputer, RunStatus};
use anyhow::{Context, Result};

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/src/day_19/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;

    let affected_area = area_affected_by_beam(&input, (50, 50))?;
    println!(
        "Day 19, Part 1: Area size affected by tractor beam: {}",
        affected_area
    );

    let square_coordinates = part_two(&input, 100)?;
    println!(
        "Day 19, Part 2: Computed value from starting coordinates of square: {}",
        square_coordinates.0 * 10000 + square_coordinates.1
    );

    Ok(())
}

fn area_affected_by_beam(bot_code: &str, max_pos: (isize, isize)) -> Result<isize> {
    let mut affected_area = 0;
    for y in 0..max_pos.0 {
        for x in 0..max_pos.0 {
            let mut bot = BeamCheckBot::new(bot_code, (x, y))?;
            if bot.run()? {
                affected_area += 1;
                // print!("#");
            } else {
                // print!(".");
            }
        }
        // println!();
    }

    Ok(affected_area)
}

fn part_two(bot_code: &str, desired_area_len: usize) -> Result<(isize, isize)> {
    let coord_modifier = isize::try_from(desired_area_len - 1)?;
    let last_x_start = 0;
    for y in coord_modifier..2000 {
        for x in last_x_start..2000 {
            let mut bot = BeamCheckBot::new(bot_code, (x, y))?;
            if bot.run()? {
                // last_x_start = x - 2;
                // Check "opposite corner"
                let opposite_corner = (x + coord_modifier, y - coord_modifier);
                let mut bot = BeamCheckBot::new(bot_code, opposite_corner)?;
                if bot.run()? {
                    return Ok((x, y - coord_modifier));
                }
                continue;
            }
        }
    }
    anyhow::bail!("Failed to find square with requested len")
}

fn _part_two(bot_code: &str, desired_area_len: usize) -> Result<()> {
    'y_loop: for y in 0..isize::MAX {
        let mut first_x_affected = None;
        let mut consecutive_x_len = 0;
        'x_loop: for x in 0..isize::MAX {
            if first_x_affected.is_none() {
                break 'x_loop;
            }
            //
            let mut bot = BeamCheckBot::new(bot_code, (x, y))?;
            if bot.run()? {
                consecutive_x_len += 1;
                if first_x_affected.is_none() {
                    first_x_affected = Some(x);
                }
            } else if consecutive_x_len > 0 {
                break 'x_loop;
            }
        }

        if let Some(first_x_affected) = first_x_affected
            && consecutive_x_len == desired_area_len
        {
            // Find the len downwards

            for yy in y + 1..y + (desired_area_len as isize) + 1 {
                let mut bot = BeamCheckBot::new(bot_code, (first_x_affected, yy))?;
                if !bot.run()? {
                    break 'y_loop;
                }
            }

            println!(
                "Found at start {}, {} = {}",
                first_x_affected,
                y,
                first_x_affected * 1000 + y
            );
            return Ok(());
        }
    }

    Ok(())
}

struct BeamCheckBot {
    computer: IntcodeComputer,
}

impl BeamCheckBot {
    fn new(program: &str, target_pos: (isize, isize)) -> Result<Self> {
        Ok(Self {
            computer: IntcodeComputer::new(program, [target_pos.0, target_pos.1])?,
        })
    }

    fn run(&mut self) -> Result<bool> {
        loop {
            match self.computer.run()? {
                RunStatus::Halt => {
                    let out = self.computer.next_output().context("No result")?;
                    return Ok(out == 1);
                }
                RunStatus::WaitForInput => {
                    anyhow::bail!("Drone should not request input while in flight")
                }
                RunStatus::OutputValue => (),
            }
        }
    }
}
