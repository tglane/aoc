use anyhow::{Context, Result};
use std::collections::HashMap;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.trim().chars().collect()).collect()
}

fn update_coord(coord: (usize, usize), modifier: &(isize, isize)) -> Result<(usize, usize)> {
    let i = (coord.0 as isize).checked_add(modifier.0).context("")?;
    let j = (coord.1 as isize).checked_add(modifier.1).context("")?;
    Ok((usize::try_from(i)?, usize::try_from(j)?))
}

fn count_xmas(field: &Vec<Vec<char>>) -> Result<usize> {
    let mut xmas = 0;

    let check_pos = |coord: (usize, usize), target: char| {
        if let Some(i) = field.get(coord.0) {
            if let Some(j) = i.get(coord.1) {
                return *j == target;
            }
        }
        false
    };

    let directions = vec![
        (0, 1),
        (0, -1),
        (1, 0),
        (1, 1),
        (1, -1),
        (-1, 0),
        (-1, 1),
        (-1, -1),
    ];

    for i in 0..field.len() {
        for j in 0..field[i].len() {
            if field[i][j] != 'X' {
                continue;
            }

            'outer: for dir in &directions {
                let (mut mi, mut mj) = (i, j);
                for step in 0..3 {
                    (mi, mj) = if let Ok(tup) = update_coord((mi, mj), dir) {
                        tup
                    } else {
                        continue 'outer;
                    };
                    let target = match step {
                        0 => 'M',
                        1 => 'A',
                        2 => 'S',
                        _ => unreachable!(),
                    };

                    if !check_pos((mi, mj), target) {
                        continue 'outer;
                    }
                }
                xmas += 1;
            }
        }
    }

    Ok(xmas)
}

fn count_x_mas(field: &Vec<Vec<char>>) -> Result<usize> {
    let check_pos = |coord: (usize, usize), target: char| {
        if let Some(i) = field.get(coord.0) {
            if let Some(j) = i.get(coord.1) {
                return *j == target;
            }
        }
        false
    };

    let mut a_pos = HashMap::<(usize, usize), usize>::new();
    let directions = vec![(1, 1), (1, -1), (-1, 1), (-1, -1)];

    for i in 0..field.len() {
        for j in 0..field[i].len() {
            if field[i][j] != 'M' {
                continue;
            }

            'outer: for dir in &directions {
                let (mut mi, mut mj) = (i, j);
                let mut a = None;
                for step in 0..2 {
                    (mi, mj) = if let Ok(tup) = update_coord((mi, mj), dir) {
                        tup
                    } else {
                        continue 'outer;
                    };
                    let target = match step {
                        0 => 'A',
                        1 => 'S',
                        _ => unreachable!(),
                    };

                    if !check_pos((mi, mj), target) {
                        continue 'outer;
                    } else if target == 'A' {
                        a = Some((mi, mj));
                    }
                }
                *a_pos.entry(a.expect("Ensured by if before")).or_default() += 1;
            }
        }
    }

    let xmas = a_pos.iter().filter(|(_pos, cnt)| **cnt == 2).count();
    Ok(xmas)
}

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/src/day_4/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;
    let field = parse_input(&input);
    let xmas_count = count_xmas(&field)?;
    println!("Day 4, Part 1: XMAS count: {xmas_count}");
    let x_mas_count = count_x_mas(&field)?;
    println!("Day 4, Part 1: X-MAS count: {x_mas_count}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn part_one() {
        let field = parse_input(INPUT);
        let count = count_xmas(&field).unwrap();
        assert_eq!(count, 18);
    }

    #[test]
    fn part_two() {
        let field = parse_input(INPUT);
        let count = count_x_mas(&field).unwrap();
        assert_eq!(count, 9);
    }
}
