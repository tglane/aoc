use anyhow::{bail, Context, Result};

fn parse_input(input: &str) -> Result<(Vec<u32>, Vec<u32>)> {
    let (mut first, mut sec) = (Vec::default(), Vec::default());
    for line in input.lines() {
        let mut ls = line.split_whitespace().filter(|s| !s.is_empty());
        let f = ls.nth(0).context("Index 0 not found")?.parse::<u32>()?;
        let s = ls.nth(0).context("Index 1 not not found")?.parse::<u32>()?;
        first.push(f);
        sec.push(s);
    }
    Ok((first, sec))
}

fn list_distance(mut a: Vec<u32>, mut b: Vec<u32>) -> Result<u32> {
    if a.len() != b.len() {
        bail!("Lists lenght does not match");
    }
    a.sort();
    b.sort();
    let mut distance = 0;
    for i in 0..a.len() {
        distance += a[i].abs_diff(b[i]);
    }
    Ok(distance)
}

fn similarity_score(a: &[u32], b: &[u32]) -> Result<u32> {
    let mut similarity = 0;
    for num in a {
        similarity += num * b.iter().filter(|n| *n == num).count() as u32;
    }
    Ok(similarity)
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    let (a, b) = parse_input(&input)?;
    let similarity = similarity_score(&a, &b)?;
    let distance = list_distance(a, b)?;
    println!("Part 1: Total distance between lists: {distance}");
    println!("Part 2: Similarity score of the lists: {similarity}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3
"#;

    #[test]
    fn part_one() {
        let (a, b) = parse_input(INPUT).unwrap();
        assert_eq!(list_distance(a, b).unwrap(), 11);
    }

    #[test]
    fn part_two() {
        let (a, b) = parse_input(INPUT).unwrap();
        assert_eq!(similarity_score(&a, &b).unwrap(), 31);
    }
}
