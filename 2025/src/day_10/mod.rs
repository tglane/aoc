use crate::Day;
use anyhow::{Context, Result};
use std::collections::{HashSet, VecDeque};
use std::path::Path;

pub(crate) struct DayTen {
    input: String,
}

impl Day for DayTen {
    fn new<P: AsRef<Path>>(path: P) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            input: std::fs::read_to_string(path)?,
        })
    }

    fn part_one(&self) -> Result<()> {
        let machine_instructions = self
            .input
            .lines()
            .map(MachineInstruction::try_from)
            .collect::<Result<Vec<_>>>()?;
        let fewest_presses = machine_instructions
            .iter()
            .map(|m| m.solve_instruction().unwrap())
            .sum::<usize>();
        println!("Day 10 - Part 1: Sum of fewest presses: {fewest_presses}");
        Ok(())
    }

    fn part_two(&self) -> Result<()> {
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum LightStatus {
    On,
    Off,
}

impl LightStatus {
    fn toggle(&mut self) {
        *self = match self {
            Self::On => Self::Off,
            Self::Off => Self::On,
        };
    }
}

impl TryFrom<char> for LightStatus {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Self::On),
            '.' => Ok(Self::Off),
            _ => anyhow::bail!("Invalid light status"),
        }
    }
}

#[derive(Debug)]
struct Button {
    lights_to_change: Vec<usize>,
}

impl Button {
    fn from_iter(iter: impl Iterator<Item = usize>) -> Self {
        Self {
            lights_to_change: iter.collect(),
        }
    }

    fn push(&self, lights: &mut [LightStatus]) {
        for light_index in self.lights_to_change.iter() {
            lights[*light_index].toggle();
        }
    }
}

impl TryFrom<&str> for Button {
    type Error = anyhow::Error;

    fn try_from(mut value: &str) -> Result<Self, Self::Error> {
        // Schema: (0,2,3,4)

        value = value
            .strip_prefix('(')
            .context("Button description is missing prefix")?;
        value = value
            .strip_suffix(')')
            .context("Button description is missing suffix")?;

        let lights_to_change = value
            .split(',')
            .map(|s| {
                s.parse::<usize>()
                    .context("Button description can not be parsed into integer")
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Self { lights_to_change })
    }
}

#[derive(Debug)]
struct MachineInstruction {
    desired_lights: Vec<LightStatus>,
    buttons: Vec<Button>,
    joltage_levels: Vec<usize>,
}

impl MachineInstruction {
    fn solve_instruction(&self) -> Option<usize> {
        struct QueueState {
            light_status: Vec<LightStatus>,
            presses_until: usize,
        }

        let mut queue = VecDeque::from([QueueState {
            light_status: vec![LightStatus::Off; self.desired_lights.len()],
            presses_until: 0_usize,
        }]);
        let mut seen = HashSet::new();

        while let Some(curr) = queue.pop_front() {
            if curr.light_status == self.desired_lights {
                return Some(curr.presses_until);
            }

            if seen.contains(&curr.light_status) {
                continue;
            }
            seen.insert(curr.light_status.clone());

            for next_button in self.buttons.iter() {
                let mut next_light_status = curr.light_status.clone();
                next_button.push(&mut next_light_status);
                queue.push_back(QueueState {
                    light_status: next_light_status,
                    presses_until: curr.presses_until + 1,
                });
            }
        }

        None
    }
}

impl TryFrom<&str> for MachineInstruction {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // Schema: [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}

        let mut desired_lights = Vec::new();
        let mut buttons = Vec::new();
        let mut joltage_levels = Vec::new();

        for part in value.split_whitespace() {
            let mut cs = part.chars();
            match cs.next() {
                Some('[') => {
                    for c in cs.take_while(|c| *c != ']') {
                        desired_lights.push(LightStatus::try_from(c)?);
                    }
                }
                Some('(') => {
                    let button = Button::from_iter(
                        cs.take_while(|c| *c != ')')
                            .filter(|c| c.is_numeric())
                            .map(|c| c.to_digit(10).unwrap() as usize),
                    );
                    buttons.push(button);
                }
                Some('{') => {
                    // Ignore joltage config for now
                    for c in cs
                        .take_while(|c| *c != '}')
                        .filter(|c| c.is_numeric())
                        .map(|c| c.to_digit(10).unwrap() as usize)
                    {
                        joltage_levels.push(c);
                    }
                }
                _ => anyhow::bail!("Invalid part"),
            }
        }

        Ok(Self {
            desired_lights,
            buttons,
            joltage_levels,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
"#;

    #[test]
    fn part_one() {
        let machine_instructions = INPUT
            .lines()
            .map(MachineInstruction::try_from)
            .collect::<Result<Vec<_>>>()
            .unwrap();
        let fewest_presses = machine_instructions
            .iter()
            .map(|m| m.solve_instruction().unwrap())
            .sum::<usize>();
        assert_eq!(fewest_presses, 7);
    }

    #[test]
    fn part_two() {}
}
