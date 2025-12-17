use crate::Day;
use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet, VecDeque};
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
        let fewest_presses = machine_instructions.iter().fold(0, |acc, m| {
            m.set_desired_lights().map(|presses| acc + presses).unwrap_or(acc)
        });
        println!("Day 10 - Part 1: Sum of fewest presses to configure lights: {fewest_presses}");
        Ok(())
    }

    fn part_two(&self) -> Result<()> {
        let machine_instructions = self
            .input
            .lines()
            .map(MachineInstruction::try_from)
            .collect::<Result<Vec<_>>>()?;
        let fewest_presses = machine_instructions.iter().fold(0, |acc, m| {
            m.set_joltage_levels().map(|presses| acc + presses).unwrap_or(acc)
        });
        println!(
            "Day 10 - Part 2: Sum of fewest presses to configure joltage levels: {fewest_presses}"
        );
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

#[derive(Debug, PartialEq, Eq)]
struct Button {
    lights_to_change: Vec<usize>,
}

impl Button {
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
    fn set_desired_lights(&self) -> Option<usize> {
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

    pub fn set_joltage_levels(&self) -> Option<usize> {
        let num_vars = self.joltage_levels.len();

        // Pre-calculate all 2^n combinations of button presses (the 'parity' combinations)
        // This is safe because Advent of Code typically keeps the number of buttons small
        let mut combinations = Vec::new();
        let num_buttons = self.buttons.len();

        for i in 0..(1 << num_buttons) {
            let mut combo_effect = vec![0; num_vars];
            let mut presses = 0;
            for j in 0..num_buttons {
                if (i >> j) & 1 == 1 {
                    presses += 1;
                    for &light in &self.buttons[j].lights_to_change {
                        combo_effect[light] += 1;
                    }
                }
            }
            combinations.push((combo_effect, presses));
        }

        let mut cache = HashMap::new();
        self.solve_recursive(&self.joltage_levels, &combinations, &mut cache)
    }

    fn solve_recursive(
        &self,
        current_target: &[usize],
        combinations: &Vec<(Vec<usize>, usize)>,
        cache: &mut HashMap<Vec<usize>, Option<usize>>
    ) -> Option<usize> {
        // Base case: all counters are zero
        if current_target.iter().all(|&x| x == 0) {
            return Some(0);
        }

        if let Some(&cached) = cache.get(current_target) {
            return cached;
        }

        let mut min_presses = None;

        for (combo_effect, combo_presses) in combinations {
            // Check if this combination matches the parity of every counter
            let mut possible = true;
            for i in 0..current_target.len() {
                if current_target[i] < combo_effect[i] || (current_target[i] % 2 != combo_effect[i] % 2) {
                    possible = false;
                    break;
                }
            }

            if possible {
                // Determine the next state: (Target - Combo) / 2
                let next_state: Vec<usize> = current_target.iter()
                    .zip(combo_effect.iter())
                    .map(|(t, c)| (t - c) / 2)
                    .collect();

                if let Some(sub_presses) = self.solve_recursive(&next_state, combinations, cache) {
                    let total = 2 * sub_presses + combo_presses;
                    if min_presses.is_none() || total < min_presses.unwrap() {
                        min_presses = Some(total);
                    }
                }
            }
        }

        cache.insert(current_target.to_vec(), min_presses);
        min_presses
    }
}

impl TryFrom<&str> for MachineInstruction {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
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
                    let mut lights_to_change = Vec::new();
                    let mut num = 0_usize;
                    while let Some(c) = cs.next()
                        && c != ')'
                    {
                        if c != ',' && c.is_numeric() {
                            num = num * 10
                                + c.to_digit(10)
                                    .context("Expected number in joltage pattern")?
                                    as usize;
                        } else if c == ',' {
                            lights_to_change.push(num);
                            num = 0;
                        } else {
                            anyhow::bail!("Unexpected character in button pattern");
                        }
                    }
                    lights_to_change.push(num);

                    buttons.push(Button { lights_to_change });
                }
                Some('{') => {
                    let mut num = 0_usize;
                    while let Some(c) = cs.next()
                        && c != '}'
                    {
                        if c != ',' && c.is_numeric() {
                            num = num * 10
                                + c.to_digit(10)
                                    .context("Expected number in joltage pattern")?
                                    as usize;
                        } else if c == ',' {
                            joltage_levels.push(num);
                            num = 0;
                        } else {
                            anyhow::bail!("Unexpected character in joltage pattern");
                        }
                    }
                    joltage_levels.push(num);
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
        let fewest_presses = machine_instructions.iter().fold(0, |acc, m| {
            m.set_desired_lights().map(|presses| acc + presses).unwrap_or(acc)
        });
        assert_eq!(fewest_presses, 7);
    }

    #[test]
    fn part_two() {
        let machine_instructions = INPUT
            .lines()
            .map(MachineInstruction::try_from)
            .collect::<Result<Vec<_>>>()
            .unwrap();
        let fewest_presses = machine_instructions.iter().fold(0, |acc, m| {
            m.set_joltage_levels().map(|presses| acc + presses).unwrap_or(acc)
        });
        assert_eq!(fewest_presses, 33);
    }
}
