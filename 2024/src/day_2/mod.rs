use anyhow::Result;

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
}

fn parse_input(input: &str) -> Result<Vec<Vec<u8>>> {
    let mut reports = Vec::default();
    for line in input.lines().filter(|l| !l.is_empty()) {
        let report = line
            .split_whitespace()
            .map(|s| s.parse::<u8>())
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .collect();
        reports.push(report);
    }
    Ok(reports)
}

fn dist_safe(a: u8, b: u8) -> bool {
    let dist = a.abs_diff(b);
    dist >= 1 && dist <= 3
}

fn direction_safe(a: u8, b: u8, dir: Direction) -> bool {
    match dir {
        Direction::Up => a < b,
        Direction::Down => a > b,
    }
}

fn is_report_safe(report: &[u8]) -> bool {
    let ascending_ok =
        report.is_sorted_by(|a, b| dist_safe(*a, *b) && direction_safe(*a, *b, Direction::Up));
    let descending_ok =
        report.is_sorted_by(|a, b| dist_safe(*a, *b) && direction_safe(*a, *b, Direction::Down));
    ascending_ok || descending_ok
}

fn is_report_safe_with_damper(report: &[u8]) -> bool {
    if is_report_safe(report) {
        return true;
    }

    for i in 0..report.len() {
        let mod_rep = report
            .iter()
            .enumerate()
            .filter(|(idx, _)| *idx != i)
            .map(|(_, n)| *n)
            .collect::<Vec<_>>();

        if is_report_safe(&mod_rep) {
            return true;
        }
    }

    false
}

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/src/day_2/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;
    let reports = parse_input(&input)?;
    let safe_count = reports.iter().filter(|r| is_report_safe(r)).count();
    println!("Day 2, Part 1: Number of safe reports: {safe_count}");
    let safe_count_with_damping = reports
        .iter()
        .filter(|r| is_report_safe_with_damper(r))
        .count();
    println!("Day 2, Part 2: Number of safe reports with error damping: {safe_count_with_damping}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn part_one() {
        let reports = parse_input(INPUT).unwrap();
        let safe_count = reports.iter().filter(|r| is_report_safe(r)).count();
        assert_eq!(safe_count, 2);
    }

    #[test]
    fn part_two() {
        let reports = parse_input(INPUT).unwrap();
        let safe_count = reports
            .iter()
            .filter(|r| is_report_safe_with_damper(r))
            .count();
        assert_eq!(safe_count, 4);
    }
}
