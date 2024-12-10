use anyhow::Result;
use std::collections::{HashMap, HashSet};

fn parse_input(input: &str) -> Result<(HashMap<char, HashSet<(isize, isize)>>, (isize, isize))> {
    let mut antennas = HashMap::<char, HashSet<(isize, isize)>>::new();
    let mut bounds = (0_isize, 0_isize);

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                antennas
                    .entry(c)
                    .or_default()
                    .insert((x.try_into()?, y.try_into()?));
            }
            bounds.0 = bounds.0.max(x.try_into()?);
        }
        bounds.1 = bounds.1.max(y.try_into()?);
    }

    Ok((antennas, (bounds.0 + 1, bounds.1 + 1)))
}

fn in_bounds(pos: &(isize, isize), bounds: &(isize, isize)) -> bool {
    if pos.0 < 0 || pos.1 < 0 {
        return false;
    }
    if pos.0 >= bounds.0 || pos.1 >= bounds.1 {
        return false;
    }
    true
}

fn gen_antinodes(a: &(isize, isize), b: &(isize, isize)) -> ((isize, isize), (isize, isize)) {
    let np1 = (a.0 - (b.0 - a.0), a.1 - (b.1 - a.1));
    let np2 = (b.0 - (a.0 - b.0), b.1 - (a.1 - b.1));
    (np1, np2)
}

fn distinct_anitnodes(
    antennas: &HashMap<char, HashSet<(isize, isize)>>,
    bounds: &(isize, isize),
) -> usize {
    let mut antinodes = HashSet::new();

    for (_freq, positions) in antennas.iter() {
        for (i, a_pos) in positions.iter().enumerate() {
            for (j, b_pos) in positions.iter().enumerate() {
                if i == j {
                    continue;
                }
                let (first, sec) = gen_antinodes(a_pos, b_pos);

                if in_bounds(&first, bounds) {
                    antinodes.insert(first);
                }
                if in_bounds(&sec, bounds) {
                    antinodes.insert(sec);
                }
            }
        }
    }

    antinodes.len()
}

fn harmonic_antinodes(
    antennas: &HashMap<char, HashSet<(isize, isize)>>,
    bounds: &(isize, isize),
) -> usize {
    let mut antinodes = HashSet::new();

    for (_freq, positions) in antennas.iter() {
        for (i, a_pos) in positions.iter().enumerate() {
            for (j, b_pos) in positions.iter().enumerate() {
                if i == j {
                    continue;
                }
                let v = (b_pos.0 - a_pos.0, b_pos.1 - a_pos.1);

                for i in 1..=isize::MAX {
                    let n = (a_pos.0 + v.0 * i, a_pos.1 + v.1 * i);
                    if in_bounds(&n, bounds) {
                        antinodes.insert(n);
                    } else {
                        break;
                    }
                }

                for i in 1..=isize::MAX {
                    let n = (b_pos.0 - v.0 * i, b_pos.1 - v.1 * i);
                    if in_bounds(&n, bounds) {
                        antinodes.insert(n);
                    } else {
                        break;
                    }
                }
            }
        }
    }

    antinodes.len()
}

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/src/day_8/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;
    let (antennas, bounds) = parse_input(&input)?;

    let distinct_antinodes = distinct_anitnodes(&antennas, &bounds);
    println!("Day 8, Part 1: Distinct antinode positions: {distinct_antinodes}");

    let harmonic_antinodes = harmonic_antinodes(&antennas, &bounds);
    println!("Day 8, Part 2: Harmonic antinode positions: {harmonic_antinodes}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn part_one() {
        let (antennas, bounds) = parse_input(INPUT).unwrap();
        println!("bounds: {bounds:?} -- a: {antennas:?}");

        assert_eq!(distinct_anitnodes(&antennas, &bounds), 14);
    }

    #[test]
    fn part_two() {
        let (antennas, bounds) = parse_input(INPUT).unwrap();
        assert_eq!(harmonic_antinodes(&antennas, &bounds), 34);
    }
}
