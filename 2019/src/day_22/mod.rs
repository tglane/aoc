use anyhow::{Context, Result};

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/src/day_22/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;

    let shuffles = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(Technique::try_from)
        .collect::<Result<Vec<_>>>()
        .unwrap();

    let input = (0..=10006).collect::<Vec<i64>>();
    let index_of_2019 = shuffles
        .iter()
        .fold(input, |mut input, t| {
            t.apply(&mut input);
            input
        })
        .iter()
        .enumerate()
        .find_map(|(idx, card)| if *card == 2019 { Some(idx) } else { None })
        .context("No card 2019 found in deck")?;
    println!("Day 22, Part 1: Position of card 2019: {index_of_2019}");

    let deck_size = 119315717514047_i128;
    let iterations = 101741582076661_i128;
    let x = part_two(&shuffles, deck_size, iterations, 2020);
    println!("Day 22, Part 2: Card at position 2020: {x}");

    Ok(())
}

fn part_two(shuffles: &[Technique], deck_size: i128, iterations: i128, pos: i128) -> i128 {
    let (mut a, mut b) = (1_i128, 0_i128);
    for shuffle in shuffles.iter() {
        (a, b) = shuffle.apply_as_linear_func(deck_size, a, b);
    }
    // println!("A: {a}, B: {b}");

    let pow_result = mod_pow(1 - a, deck_size - 2, deck_size);
    let r = ((b * pow_result) % deck_size).rem_euclid(deck_size);
    // println!("R: {r}");

    ((pos - r) * mod_pow(a, iterations * (deck_size - 2), deck_size) + r).rem_euclid(deck_size)
}

fn mod_pow(base: i128, mut exp: i128, m: i128) -> i128 {
    let mut result = 1;
    let mut base = (base % m + m) % m;
    while exp > 0 {
        if exp % 2 == 1 {
            result = ((result * base) % m + m) % m;
        }
        base = ((base * base) % m + m) % m;
        exp /= 2;
    }
    result
}

#[derive(Debug, Clone)]
enum Technique {
    DealIntoNewStack,
    Cut(i128),
    DealWithIncrement(i128),
}

impl Technique {
    fn apply(&self, input: &mut [i64]) {
        match self {
            Self::DealIntoNewStack => input.reverse(),
            Self::Cut(n) => {
                if *n > 0 {
                    let mut tmp = input[..*n as usize].to_vec();
                    input.rotate_left(*n as usize);
                    let len = input.len();
                    input[len - *n as usize..].swap_with_slice(&mut tmp);
                } else {
                    let len = input.len();
                    let n = n.unsigned_abs() as usize;
                    let mut tmp = input[len - n..].to_vec();
                    input.rotate_right(n);
                    input[..n].swap_with_slice(&mut tmp);
                }
            }
            Self::DealWithIncrement(n) => {
                let mut pos = 0;
                let mut tmp = input.to_vec();
                for i in 0..input.len() {
                    tmp[pos] = input[i];
                    pos = (pos + *n as usize) % input.len();
                }
                input.copy_from_slice(&tmp);
            }
        }
    }

    fn apply_as_linear_func(&self, m: i128, a: i128, b: i128) -> (i128, i128) {
        match self {
            Self::DealIntoNewStack => ((-a).rem_euclid(m), (m - 1 - b) % m),
            Self::Cut(x) => (a, (b - x) % m),
            Self::DealWithIncrement(x) => (a * x % m, b * x % m),
        }
    }
}

impl TryFrom<&str> for Technique {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.starts_with("deal into") {
            return Ok(Self::DealIntoNewStack);
        } else if value.starts_with("cut") {
            let (_, v) = value.split_once(' ').context("")?;
            return Ok(Self::Cut(v.parse()?));
        } else if value.starts_with("deal with") {
            let v = value.split(' ').next_back().context("")?;
            return Ok(Self::DealWithIncrement(v.parse()?));
        }
        anyhow::bail!("Invalid technique description")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_s() {
        const INPUT: &str = r#"deal with increment 7
deal into new stack
deal into new stack
"#;
        let shuffles = INPUT
            .lines()
            .filter(|l| !l.is_empty())
            .map(Technique::try_from)
            .collect::<Result<Vec<_>>>()
            .unwrap();

        let input = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let output = shuffles.into_iter().fold(input, |mut input, t| {
            t.apply(&mut input);
            input
        });
        assert_eq!(output, [0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);
    }

    #[test]
    fn part_one_m() {
        const INPUT: &str = r#"cut 6
deal with increment 7
deal into new stack
"#;
        let shuffles = INPUT
            .lines()
            .filter(|l| !l.is_empty())
            .map(Technique::try_from)
            .collect::<Result<Vec<_>>>()
            .unwrap();

        let input = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let output = shuffles.into_iter().fold(input, |mut input, t| {
            t.apply(&mut input);
            input
        });
        assert_eq!(output, [3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);
    }

    #[test]
    fn part_one_m2() {
        const INPUT: &str = r#"deal with increment 7
deal with increment 9
cut -2
"#;
        let shuffles = INPUT
            .lines()
            .filter(|l| !l.is_empty())
            .map(Technique::try_from)
            .collect::<Result<Vec<_>>>()
            .unwrap();

        let input = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let output = shuffles.into_iter().fold(input, |mut input, t| {
            t.apply(&mut input);
            input
        });
        assert_eq!(output, [6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);
    }

    #[test]
    fn part_one_l() {
        const INPUT: &str = r#"
deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1
"#;
        let shuffles = INPUT
            .lines()
            .filter(|l| !l.is_empty())
            .map(Technique::try_from)
            .collect::<Result<Vec<_>>>()
            .unwrap();

        let input = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let output = shuffles.into_iter().fold(input, |mut input, t| {
            t.apply(&mut input);
            input
        });
        assert_eq!(output, [9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);
    }

    #[test]
    fn part_two() {}
}
