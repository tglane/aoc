use crate::Day;
use anyhow::Result;
use std::path::Path;

pub(crate) struct DayThree {
    input: String,
}

impl Day for DayThree {
    fn new<P: AsRef<Path>>(path: P) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            input: std::fs::read_to_string(path)?,
        })
    }

    fn part_one(&self) -> Result<()> {
        let max_jolts = parse_input(&self.input)?
            .into_iter()
            .map(|bank| bank.max_jolts_with(2))
            .fold(0_usize, |acc, jolt| acc + jolt.unwrap_or(0));
        println!("Day 3 - Part 1: Max jolts with two batteries per bank: {max_jolts:?}");
        Ok(())
    }

    fn part_two(&self) -> Result<()> {
        let max_jolts = parse_input(&self.input)?
            .into_iter()
            .map(|bank| bank.max_jolts_with(12))
            .fold(0_usize, |acc, jolt| acc + jolt.unwrap_or(0));
        println!("Day 3 - Part 2: Max jolts with 12 batteries per bank: {max_jolts:?}");
        Ok(())
    }
}

struct BatteryBank(Vec<usize>);

impl BatteryBank {
    fn max_jolts_with(&self, batteries: usize) -> Option<usize> {
        let self_ref = BatteryBankRef::from(self);
        self_ref.max_jolts_with(batteries)
    }
}

struct BatteryBankRef<'t>(&'t [usize]);

impl<'t> From<&'t BatteryBank> for BatteryBankRef<'t> {
    fn from(value: &'t BatteryBank) -> Self {
        Self(&value.0)
    }
}

impl<'t> BatteryBankRef<'t> {
    fn max_jolts_with(&self, batteries: usize) -> Option<usize> {
        if batteries > self.0.len() {
            return None;
        }

        if batteries == 1 {
            self.0.iter().max().cloned()
        } else {
            let mut max = self.0[0];
            let mut max_idx = 0;
            for (idx, batt) in self.0[..self.0.len() - (batteries - 1)].iter().enumerate() {
                if *batt > max {
                    max = *batt;
                    max_idx = idx;
                }
            }

            max *= 10_usize.pow(batteries as u32 - 1);

            let next_bank = BatteryBankRef(&self.0[max_idx + 1..]);
            Some(max + next_bank.max_jolts_with(batteries - 1)?)
        }
    }
}

fn parse_input(input: &str) -> Result<Vec<BatteryBank>> {
    input
        .lines()
        .map(|line| {
            let batteries = line
                .chars()
                .map(|c| {
                    c.to_digit(10)
                        .map(|c| c as usize)
                        .ok_or(anyhow::Error::msg(""))
                })
                .collect::<Result<Vec<_>>>()?;
            Ok(BatteryBank(batteries))
        })
        .collect::<Result<Vec<_>>>()
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = r#"987654321111111
811111111111119
234234234234278
818181911112111
"#;

    #[test]
    fn part_one() {
        let max_jolts = parse_input(INPUT)
            .unwrap()
            .into_iter()
            .map(|bank| bank.max_jolts_with(2))
            .fold(0_usize, |acc, jolt| acc + jolt.unwrap_or(0));
        assert_eq!(max_jolts, 357);
    }

    #[test]
    fn part_two() {
        let max_jolts = parse_input(INPUT)
            .unwrap()
            .into_iter()
            .map(|bank| bank.max_jolts_with(12))
            .fold(0_usize, |acc, jolt| acc + jolt.unwrap_or(0));
        assert_eq!(max_jolts, 3121910778619);
    }
}
