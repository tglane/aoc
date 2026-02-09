use crate::intcode::IntcodeComputer;
use anyhow::{Context, Result};
use itertools::Itertools;

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/src/day_7/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;

    let max_thruster_output = find_input_for_max_thrust(&input)?;
    println!("Day 7, Part 1: Max possible thruster output: {max_thruster_output}");

    let max_thruster_output = find_input_for_max_thrust_with_feedback(&input)?;
    println!(
        "Day 7, Part 2: Max possible thruster output with feedback loop: {max_thruster_output}"
    );

    Ok(())
}

fn find_input_for_max_thrust(program: &str) -> Result<isize> {
    let max_thruster_output = [0_isize, 1, 2, 3, 4]
        .iter()
        .permutations(5)
        .map(|phase_seq| -> Result<isize> {
            // Create amplifier computers from phase sequence
            // Each amplifier uses the next element of the phase sequence as its first input
            // parameter and the output value of the previous amplifier as its second input
            // parameter.
            let amplifiers = phase_seq
                .iter()
                .map(|input| IntcodeComputer::new(program, [**input]));

            let mut prev_output = 0;

            for amplifier in amplifiers {
                let mut amplifier = amplifier?;
                amplifier.push_input(prev_output);

                amplifier.run()?;

                prev_output = amplifier.next_output().context("No output found")?;
            }

            Ok(prev_output)
        })
        .filter_map(|r| r.ok())
        .max();

    max_thruster_output.context("No maximum thruster output could be determined")
}

fn find_input_for_max_thrust_with_feedback(program: &str) -> Result<isize> {
    let max_thruster_output = [5_isize, 6, 7, 8, 9]
        .iter()
        .permutations(5)
        .map(|phase_seq| -> Result<isize> {
            // Create amplifier computers from phase sequence
            // Each amplifier uses the next element of the phase sequence as its first input
            // parameter and the output value of the previous amplifier as its second input
            // parameter.
            let mut amplifiers = phase_seq
                .iter()
                .map(|input| IntcodeComputer::new(program, [**input]))
                .collect::<Result<Vec<_>>>()?;

            let mut prev_output = 0;

            for (idx, _) in (0..amplifiers.len()).enumerate().cycle() {
                let amplifier = &mut amplifiers[idx];
                amplifier.push_input(prev_output);

                let amp_status = amplifier.run()?;
                if amp_status.output_ready() {
                    prev_output = amplifier.next_output().context("No output available")?;
                } else if amp_status.is_halted() && idx == amplifiers.len() - 1 {
                    break;
                }
            }

            Ok(prev_output)
        })
        .filter_map(|r| r.ok())
        .max();

    max_thruster_output.context("No maximum thruster output could be determined")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let max_thruster_output =
            find_input_for_max_thrust("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0").unwrap();
        assert_eq!(max_thruster_output, 43210);

        let max_thruster_output = find_input_for_max_thrust(
            "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0",
        )
        .unwrap();
        assert_eq!(max_thruster_output, 54321);
    }

    #[test]
    fn part_two() {
        let max_thruster_output = find_input_for_max_thrust_with_feedback(
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
        )
        .unwrap();
        assert_eq!(max_thruster_output, 139629729);

        let max_thruster_output = find_input_for_max_thrust_with_feedback(
            "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10",
        )
        .unwrap();
        assert_eq!(max_thruster_output, 18216);
    }
}
