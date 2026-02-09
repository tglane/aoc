use anyhow::{Context, Result};
use itertools::Itertools;
use std::fmt::Display;

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/src/day_8/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;
    const DIMENSIONS: (usize, usize) = (25, 6);

    let sif = SpaceImageFormat::new(&input, DIMENSIONS)?;
    let checksum = sif.checksum().context("Failed to calculate checksum")?;
    println!("Day 8, Part 1: Space image file checksum: {checksum}");

    let image = sif.decode();
    println!("Day 8, Part 2: Decoded image:\n{}", image);

    Ok(())
}

#[derive(Clone, Debug)]
struct Layer {
    data: Vec<u8>,
}

impl Layer {
    fn count_zeros(&self) -> usize {
        self.data.iter().filter(|n| **n == 0).count()
    }

    fn checksum(&self) -> usize {
        // Number of 1 digits multiplied with number of two digits
        let mut ones = 0;
        let mut twos = 0;
        for n in self.data.iter() {
            if *n == 1 {
                ones += 1;
            } else if *n == 2 {
                twos += 1;
            }
        }
        ones * twos
    }
}

#[derive(Debug)]
struct SpaceImageFormat {
    dimensions: (usize, usize),
    layers: Vec<Layer>,
}

impl SpaceImageFormat {
    fn new(data: &str, dimensions: (usize, usize)) -> Result<Self> {
        let layer_len = dimensions.0 * dimensions.1;
        let layers = data
            .trim()
            .chars()
            .chunks(layer_len)
            .into_iter()
            .map(|layer_data| {
                Ok(Layer {
                    data: layer_data
                        .map(|c| c.to_digit(10).map(|n| n as u8).context("Not a number"))
                        .collect::<Result<Vec<_>>>()?,
                })
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { dimensions, layers })
    }

    fn checksum(&self) -> Option<usize> {
        self.layers
            .iter()
            .min_by_key(|layer| layer.count_zeros())
            .map(|min_zero_layer| min_zero_layer.checksum())
    }

    fn decode(&self) -> Image {
        let data_len = self.dimensions.0 * self.dimensions.1;
        let mut decoded_layer = Layer {
            data: vec![2_u8; data_len],
        };
        'pos_loop: for pos in 0..data_len {
            for layer in self.layers.iter() {
                if layer.data[pos] == 0 || layer.data[pos] == 1 {
                    decoded_layer.data[pos] = layer.data[pos];
                    continue 'pos_loop;
                }
            }
        }

        Image {
            dimensions: self.dimensions,
            data: decoded_layer,
        }
    }
}

#[derive(Debug)]
struct Image {
    dimensions: (usize, usize),
    data: Layer,
}

impl Image {
    #[allow(dead_code)]
    fn render(&self) {
        println!("{}", self);
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.dimensions.1 {
            for x in 0..self.dimensions.0 {
                match self.data.data[y * self.dimensions.0 + x] {
                    0 => write!(f, "#")?,
                    1 => write!(f, ".")?,
                    2 => write!(f, " ")?,
                    _ => return Err(std::fmt::Error),
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        const INPUT: &str = "123456789012";
        const DIMENSIONS: (usize, usize) = (3, 2);
        let sif = SpaceImageFormat::new(INPUT, DIMENSIONS).unwrap();
        let checksum = sif.checksum().unwrap();
        assert_eq!(checksum, 1);
    }

    #[test]
    fn part_two() {
        const INPUT: &str = "0222112222120000";
        const DIMENSIONS: (usize, usize) = (2, 2);
        let sif = SpaceImageFormat::new(INPUT, DIMENSIONS).unwrap();
        let decoded = sif.decode();
        assert_eq!(decoded.data.data, vec![0, 1, 1, 0]);
    }
}
