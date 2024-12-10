use anyhow::{bail, Context, Result};
use std::collections::HashSet;

#[derive(Clone, Debug)]
struct Data {
    id: usize,
    size: usize,
}

#[derive(Clone, Debug)]
enum Block {
    Free(usize),
    Data(Data),
}

#[derive(Clone, Debug)]
struct Filesystem(Vec<Block>);

impl Filesystem {
    fn compacting_data(&mut self) {
        let (mut i, mut j) = (0, self.0.len() - 1);

        while i != j {
            let ib = match &self.0[i] {
                Block::Data(_) => {
                    i += 1;
                    continue;
                }
                Block::Free(free) => free.clone(),
            };

            let jb = match &mut self.0[j] {
                Block::Free(_) => {
                    j -= 1;
                    continue;
                }
                Block::Data(data) => data,
            };

            if ib == jb.size {
                // Move the block
                self.0[i] = Block::Data(jb.clone());
                self.0[j] = Block::Free(ib);
                i += 1;
                j -= 1;
            } else if ib > jb.size {
                // Copy data from j into i and split i into data and free block
                let new_ib = jb.clone();
                let new_free_ib = Block::Free(ib - jb.size);
                self.0[j] = Block::Free(jb.size);
                self.0[i] = Block::Data(new_ib);
                i += 1;
                self.0.insert(i, new_free_ib);
                j -= 1;
            } else if ib < jb.size {
                // Copy as much data from j that fits into i and than check the next block
                jb.size -= ib;
                let new_ib = Data {
                    id: jb.id,
                    size: ib,
                };
                self.0[i] = Block::Data(new_ib);
                i += 1;
            }
        }
    }

    fn compacting_files(&mut self) {
        let mut visited = HashSet::new();
        let (mut i, mut j) = (0, self.0.len() - 1);

        while j > 0 {
            if i >= j {
                j -= 1;
                i = 0;
            }

            let ib = match &self.0[i] {
                Block::Data(_) => {
                    i += 1;
                    continue;
                }
                Block::Free(free) => free.clone(),
            };

            let jb = match &mut self.0[j] {
                Block::Free(_) => {
                    j -= 1;
                    continue;
                }
                Block::Data(data) => data,
            };
            if visited.contains(&jb.id) {
                j -= 1;
                i = 0;
                continue;
            }

            if ib == jb.size {
                // Move the block
                visited.insert(jb.id);
                self.0[i] = Block::Data(jb.clone());
                self.0[j] = Block::Free(ib);
                i = 0;
                j -= 1;
            } else if ib > jb.size {
                // Copy data from j into i and split i into data and free block
                let jb = jb.clone();
                visited.insert(jb.id);
                let free_after_ib = Block::Free(ib - jb.size);
                self.0[j] = Block::Free(jb.size);
                self.0[i] = Block::Data(jb);
                self.0.insert(i + 1, free_after_ib);
                i = 0;
                j -= 1;
            } else {
                // File does not fit so we just increase the i cursor
                i += 1;
            }
        }
    }

    fn checksum(&self) -> usize {
        let mut checksum = 0;

        let mut i = 0;
        for block in &self.0 {
            match block {
                Block::Data(data) => {
                    for _ in 0..data.size {
                        checksum += i * data.id;
                        i += 1;
                    }
                }
                Block::Free(size) => i += size,
            }
        }

        checksum
    }
}

impl TryFrom<&str> for Filesystem {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut id = 0;
        let blocks = value
            .trim()
            .chars()
            .map(|c| c.to_digit(10).context(""))
            .enumerate()
            .map(|(idx, size)| {
                if let Ok(size) = size {
                    if idx % 2 == 0 {
                        let block = Block::Data(Data {
                            id,
                            size: size.try_into()?,
                        });
                        id += 1;
                        Ok(block)
                    } else {
                        Ok(Block::Free(size.try_into()?))
                    }
                } else {
                    bail!("")
                }
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Self(blocks))
    }
}

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/src/day_9/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;

    let mut fs = Filesystem::try_from(input.as_str()).unwrap();
    fs.compacting_data();
    println!(
        "Day 9, Part 1: Checksum after compacting: {}",
        fs.checksum()
    );

    let mut fs = Filesystem::try_from(input.as_str()).unwrap();
    fs.compacting_files();
    println!(
        "Day 9, Part 2: Checksum after compacting complete files: {}",
        fs.checksum()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn part_one() {
        let mut fs = Filesystem::try_from(INPUT).unwrap();
        fs.compacting_data();
        let checksum = fs.checksum();
        assert_eq!(checksum, 1928);
    }

    #[test]
    fn part_two() {
        let mut fs = Filesystem::try_from(INPUT).unwrap();
        fs.compacting_files();
        let checksum = fs.checksum();
        assert_eq!(checksum, 2858);
    }
}
