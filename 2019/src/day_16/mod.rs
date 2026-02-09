use anyhow::Result;

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/src/day_16/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?
    .trim()
    .chars()
    .map(|c| {
        c.to_digit(10)
            .map(|n| n as i64)
            .ok_or(anyhow::Error::msg("No a number"))
    })
    .collect::<Result<Vec<i64>>>()?;

    let multiple_input = input
        .iter()
        .cycle()
        .take(10000 * input.len())
        .cloned()
        .collect::<Vec<_>>();

    let output = flawed_frequency_transmission(input, 100);
    println!(
        "Day 14, Part 1: First 8 digits of the FFT decoding test: {:?}",
        &output[..8]
    );

    let output = flawed_frequency_transmission_with_offset(multiple_input, 100, 7, 8);
    println!("Day 16, Part 2: Decoded FFT message: {:?}", output);

    Ok(())
}

fn flawed_frequency_transmission(mut input: Vec<i64>, phases: u64) -> Vec<i64> {
    for _ in 0..phases {
        input = fft_phase(&input);
    }
    input
}

fn fft_phase(input: &[i64]) -> Vec<i64> {
    let base_pattern = [0_i64, 1, 0, -1];

    let mut output = Vec::with_capacity(input.len());
    for i in 1..=input.len() {
        let next_out = input
            .iter()
            .zip(repeated_element(base_pattern.iter(), i).cycle().skip(1))
            .map(|(input, pattern)| *input * *pattern)
            .sum::<i64>();
        output.push(next_out.abs() % 10);
    }
    output
}

fn flawed_frequency_transmission_with_offset(
    input: Vec<i64>,
    phases: u64,
    message_offset_digits: usize,
    message_len: usize,
) -> Vec<i64> {
    let message_offset = {
        let mut message_offset = 0_usize;
        for x in &input[0..message_offset_digits] {
            message_offset = message_offset * 10 + *x as usize
        }
        message_offset
    };
    assert!(message_offset > input.len() / 2);

    let mut tmp_data = input[message_offset..].to_vec();

    for _ in 0..phases {
        let mut sum = 0;
        for j in (0..tmp_data.len()).rev() {
            sum += tmp_data[j];
            tmp_data[j] = sum % 10;
        }
    }
    tmp_data.resize(message_len, 0);
    tmp_data
}

fn repeated_element<T: Clone>(
    it: impl Iterator<Item = T> + Clone,
    cnt: usize,
) -> impl Iterator<Item = T> + Clone {
    it.flat_map(move |n| std::iter::repeat_n(n, cnt))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_S: &str = "12345678";
    const INPUT_M: &str = "80871224585914546619083218645595";
    const INPUT_L: &str = "19617804207202209144916044189917";

    #[test]
    fn part_one() {
        let output = flawed_frequency_transmission(
            INPUT_S
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i64)
                .collect(),
            4,
        );
        assert_eq!(output, vec![0, 1, 0, 2, 9, 4, 9, 8]);

        let output = flawed_frequency_transmission(
            INPUT_M
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i64)
                .collect(),
            100,
        );
        assert_eq!(&output[..8], vec![2, 4, 1, 7, 6, 1, 7, 6]);

        let output = flawed_frequency_transmission(
            INPUT_L
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i64)
                .collect(),
            100,
        );
        assert_eq!(&output[..8], vec![7, 3, 7, 4, 5, 4, 1, 8]);
    }

    #[test]
    fn part_two() {
        const INPUT_XL: &str = "03036732577212944063491565474664";
        let input_repeat = INPUT_XL
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i64)
            .cycle()
            .take(10000 * INPUT_XL.len())
            .collect();
        let output = flawed_frequency_transmission_with_offset(input_repeat, 100, 7, 8);
        assert_eq!(output, vec![8, 4, 4, 6, 2, 0, 2, 6]);

        const INPUT_XXL: &str = "02935109699940807407585447034323";
        let input_repeat = INPUT_XXL
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i64)
            .cycle()
            .take(10000 * INPUT_XXL.len())
            .collect();
        let output = flawed_frequency_transmission_with_offset(input_repeat, 100, 7, 8);
        assert_eq!(output, vec![7, 8, 7, 2, 5, 2, 7, 0]);

        const INPUT_XXXL: &str = "03081770884921959731165446850517";
        let input_repeat = INPUT_XXXL
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i64)
            .cycle()
            .take(10000 * INPUT_XXXL.len())
            .collect();
        let output = flawed_frequency_transmission_with_offset(input_repeat, 100, 7, 8);
        assert_eq!(output, vec![5, 3, 5, 5, 3, 7, 3, 1]);
    }
}
