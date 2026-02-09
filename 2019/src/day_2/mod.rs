use crate::intcode::IntcodeComputer;
use anyhow::{Context, Result};

pub const OUTPUT_ADDR: usize = 0;
pub const NOUN_ADDR: usize = 1;
pub const VERB_ADDR: usize = 2;

fn find_noun_verb(computer: &IntcodeComputer, output: isize) -> Option<(isize, isize)> {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut c = computer.clone();
            *c.at_mut(NOUN_ADDR).ok()? = noun;
            *c.at_mut(VERB_ADDR).ok()? = verb;
            if c.run().is_ok() && *c.at(OUTPUT_ADDR).unwrap() == output {
                return Some((noun, verb));
            }
        }
    }
    None
}

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/src/day_2/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;

    let mut computer = IntcodeComputer::new(&input, [])?;

    let (noun, verb) = find_noun_verb(&computer, 19690720).context("No nound and verb found")?;

    *computer.at_mut(NOUN_ADDR)? = 12;
    *computer.at_mut(VERB_ADDR)? = 2;
    computer.run()?;

    println!(
        "Day 2, Part 1: Value at position 0 after running: {}",
        computer.at(OUTPUT_ADDR)?
    );

    println!(
        "Day 2, Part 2: Noun ({noun}) and verb ({verb}): {}",
        100 * noun + verb
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1,9,10,3,2,3,11,0,99,30,40,50";

    #[test]
    fn part_one() {
        let mut computer = IntcodeComputer::new(INPUT, []).unwrap();
        computer.run().unwrap();

        assert_eq!(
            computer.get_memory(),
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
    }
}
