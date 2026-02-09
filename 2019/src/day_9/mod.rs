use crate::intcode::IntcodeComputer;
use anyhow::{Context, Result};

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/src/day_9/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;

    let mut computer = IntcodeComputer::new(&input, [1])?;
    while !computer.run()?.is_halted() {}
    let boost_code = computer.next_output().context("No output found")?;
    println!("Day 9, Part 1: BOOST code: {boost_code}");

    let mut computer = IntcodeComputer::new(&input, [2])?;
    while !computer.run()?.is_halted() {}
    let distress_signal_coords = computer.next_output().context("No output found")?;
    println!(
        "Day 9, Part 2: Distress signal coordinates: {}",
        distress_signal_coords
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    #[test]
    fn part_one() {
        const INPUT1: &str = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let mut computer = IntcodeComputer::new(INPUT1, []).unwrap();
        while !computer.run().unwrap().is_halted() {}
        let output = computer.consume_output();
        assert_eq!(
            output,
            VecDeque::from([
                109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99
            ])
        );

        const INPUT2: &str = "1102,34915192,34915192,7,4,7,99,0";
        let mut computer = IntcodeComputer::new(INPUT2, []).unwrap();
        while !computer.run().unwrap().is_halted() {}
        let output = computer.consume_output();
        assert_eq!(output, VecDeque::from([1219070632396864]));

        const INPUT3: &str = "104,1125899906842624,99";
        let mut computer = IntcodeComputer::new(INPUT3, []).unwrap();
        while !computer.run().unwrap().is_halted() {}
        let output = computer.consume_output();
        assert_eq!(output, VecDeque::from([1125899906842624]));
    }
}
