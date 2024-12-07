use regex::Regex;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

use anyhow::{Context, Result};

fn parse_input(input: &str) -> Result<(HashMap<usize, HashSet<usize>>, Vec<Vec<usize>>)> {
    let mut rules = HashMap::<usize, HashSet<usize>>::default();
    let mut pages = Vec::default();

    let rule_re = Regex::new("^(?<first>\\d+)\\|(?<sec>\\d+)$")?;
    let page_re = Regex::new("^((?<a>\\d+),)+(?<b>\\d+)$")?;

    let lines = input.lines();
    for line in lines {
        if rule_re.is_match(line) {
            let cap = rule_re.captures(line).context("Capture rules failed")?;
            let (_, [first, sec]) = cap.extract();
            rules
                .entry(first.parse()?)
                .or_default()
                .insert(sec.parse()?);
        } else if page_re.is_match(line) {
            let v: Vec<usize> = line
                .trim()
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            pages.push(v);
        }
    }

    Ok((rules, pages))
}

fn is_valid_order(page_list: &[usize], rules: &HashMap<usize, HashSet<usize>>) -> bool {
    let mut visited = HashSet::<usize>::new();

    for page in page_list {
        if let Some(only_after) = rules.get(page) {
            for oa in only_after {
                if visited.contains(oa) {
                    return false;
                }
            }
        }

        visited.insert(*page);
    }

    true
}

fn compare(a: &usize, b: &usize, rules: &HashMap<usize, HashSet<usize>>) -> Ordering {
    match rules.get(&a) {
        Some(r) => {
            if r.contains(&b) {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        }
        None => match rules.get(&b) {
            Some(r) => {
                if r.contains(&a) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }
            None => Ordering::Equal,
        },
    }
}

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/src/day_5/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;
    let (rules, mut pages) = parse_input(&input).unwrap();

    let mut page_numbers = 0;
    for pl in pages.iter() {
        if is_valid_order(pl, &rules) {
            page_numbers += pl[pl.len() / 2];
        }
    }
    println!("Day 5, Part 1: Page numbers: {page_numbers}");

    let mut page_numbers_incorrect = 0;
    for pl in pages.iter_mut() {
        if !is_valid_order(pl, &rules) {
            pl.sort_by(|a, b| compare(a, b, &rules));
            page_numbers_incorrect += pl[pl.len() / 2];
        }
    }
    println!("Day 5, Part 2: Page numbers of sorted incorrect entries: {page_numbers_incorrect}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn part_one() {
        let (rules, pages) = parse_input(INPUT).unwrap();
        let mut page_numbers = 0;
        for pl in pages.iter() {
            if is_valid_order(pl, &rules) {
                page_numbers += pl[pl.len() / 2];
            }
        }

        assert_eq!(page_numbers, 143);
    }

    #[test]
    fn part_two() {
        let (rules, mut pages) = parse_input(INPUT).unwrap();
        let mut page_numbers = 0;
        for pl in pages.iter_mut() {
            if !is_valid_order(pl, &rules) {
                pl.sort_by(|a, b| compare(a, b, &rules));
                page_numbers += pl[pl.len() / 2];
            }
        }

        assert_eq!(page_numbers, 123);
    }
}
