use crate::intcode::IntcodeComputer;
use anyhow::Result;

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/src/day_5/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;

    let mut computer = IntcodeComputer::new(&input, [1])?;
    computer.run()?;
    println!(
        "Day 5, Part 1: Computer output: {:?}",
        computer.next_output()
    );

    let mut computer = IntcodeComputer::new(&input, [5])?;
    computer.run()?;
    println!(
        "Day 5, Part 2: Computer output: {:?}",
        computer.next_output()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        const INPUT: &str = "1002,4,3,4,33";
        let mut computer = IntcodeComputer::new(INPUT, []).unwrap();
        computer.run().unwrap();
        assert_eq!(computer.get_memory(), vec![1002, 4, 3, 4, 99]);
    }

    #[test]
    fn part_two() {
        const INPUT: &str = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";

        let mut computer = IntcodeComputer::new(INPUT, [5]).unwrap();
        computer.run().unwrap();
        assert_eq!(computer.next_output(), Some(999));

        let mut computer = IntcodeComputer::new(INPUT, [8]).unwrap();
        computer.run().unwrap();
        assert_eq!(computer.next_output(), Some(1000));

        let mut computer = IntcodeComputer::new(INPUT, [10]).unwrap();
        computer.run().unwrap();
        assert_eq!(computer.next_output(), Some(1001));
    }

    #[test]
    fn part_two_jump_test() {
        const INPUT1: &str = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
        let mut computer = IntcodeComputer::new(INPUT1, [5]).unwrap();
        computer.run().unwrap();
        assert_eq!(computer.next_output(), Some(1));
        let mut computer = IntcodeComputer::new(INPUT1, [0]).unwrap();
        computer.run().unwrap();
        assert_eq!(computer.next_output(), Some(0));

        const INPUT2: &str = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
        let mut computer = IntcodeComputer::new(INPUT2, [5]).unwrap();
        computer.run().unwrap();
        assert_eq!(computer.next_output(), Some(1));
        let mut computer = IntcodeComputer::new(INPUT2, [0]).unwrap();
        computer.run().unwrap();
        assert_eq!(computer.next_output(), Some(0));
    }

    #[test]
    fn part_two_equality_test() {
        const INPUT1: &str = "3,9,8,9,10,9,4,9,99,-1,8";
        let mut computer = IntcodeComputer::new(INPUT1, [8]).unwrap();
        computer.run().unwrap();
        assert_eq!(computer.next_output(), Some(1));
        let mut computer = IntcodeComputer::new(INPUT1, [23]).unwrap();
        computer.run().unwrap();
        assert_eq!(computer.next_output(), Some(0));

        const INPUT2: &str = "3,3,1108,-1,8,3,4,3,99";
        let mut computer = IntcodeComputer::new(INPUT2, [8]).unwrap();
        computer.run().unwrap();
        assert_eq!(computer.next_output(), Some(1));
        let mut computer = IntcodeComputer::new(INPUT2, [2223]).unwrap();
        computer.run().unwrap();
        assert_eq!(computer.next_output(), Some(0));
    }

    #[test]
    fn part_two_less_test() {
        const INPUT1: &str = "3,9,7,9,10,9,4,9,99,-1,8";
        let mut computer = IntcodeComputer::new(INPUT1, [7]).unwrap();
        computer.run().unwrap();
        assert_eq!(computer.next_output(), Some(1));
        let mut computer = IntcodeComputer::new(INPUT1, [8]).unwrap();
        computer.run().unwrap();
        assert_eq!(computer.next_output(), Some(0));
        let mut computer = IntcodeComputer::new(INPUT1, [10]).unwrap();
        computer.run().unwrap();
        assert_eq!(computer.next_output(), Some(0));

        const INPUT2: &str = "3,3,1107,-1,8,3,4,3,99";
        let mut computer = IntcodeComputer::new(INPUT2, [7]).unwrap();
        computer.run().unwrap();
        assert_eq!(computer.next_output(), Some(1));
        let mut computer = IntcodeComputer::new(INPUT2, [8]).unwrap();
        computer.run().unwrap();
        assert_eq!(computer.next_output(), Some(0));
        let mut computer = IntcodeComputer::new(INPUT2, [10]).unwrap();
        computer.run().unwrap();
        assert_eq!(computer.next_output(), Some(0));
    }
}
