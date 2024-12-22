use anyhow::Result;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::iter::{once, repeat_n};

lazy_static! {
    static ref NUM_PAD: HashMap<char, (isize, isize)> = HashMap::from([
        ('7', (0, 0)),
        ('8', (0, 1)),
        ('9', (0, 2)),
        ('4', (1, 0)),
        ('5', (1, 1)),
        ('6', (1, 2)),
        ('1', (2, 0)),
        ('2', (2, 1)),
        ('3', (2, 2)),
        ('#', (3, 0)),
        ('0', (3, 1)),
        ('A', (3, 2)),
    ]);
    static ref KEY_PAD: HashMap<char, (isize, isize)> = HashMap::from([
        ('#', (0, 0)),
        ('^', (0, 1)),
        ('A', (0, 2)),
        ('<', (1, 0)),
        ('v', (1, 1)),
        ('>', (1, 2)),
    ]);
}

fn empty_pad_field(pad: &HashMap<char, (isize, isize)>, pos: &(isize, isize)) -> bool {
    if let Some(empty_pos) = pad.get(&'#') {
        empty_pos == pos
    } else {
        true
    }
}

fn minimal_seq_len(
    pad: &HashMap<char, (isize, isize)>,
    code: String,
    robots: isize,
) -> Result<usize> {
    let mut cache = HashMap::<(String, isize), usize>::new();
    minimal_seq_len_inner(pad, code, robots, &mut cache)
}

fn minimal_seq_len_inner(
    pad: &HashMap<char, (isize, isize)>,
    code: String,
    robots: isize,
    cache: &mut HashMap<(String, isize), usize>,
) -> Result<usize> {
    let key = (code, robots);
    if let Some(val) = cache.get(&key) {
        return Ok(*val);
    }

    if robots == 0 {
        let len = key.0.len();
        cache.insert(key, len);
        return Ok(len);
    }

    let mut min_len = 0;
    let mut start = 'A';
    for letter in key.0.chars() {
        min_len += gen_seq_from_letter_to_letter(pad, start, letter)
            .into_iter()
            .map(|p| minimal_seq_len_inner(&KEY_PAD, p.clone(), robots - 1, cache).unwrap())
            .min()
            .unwrap_or(0);
        start = letter;
    }

    cache.insert(key, min_len);
    Ok(min_len)
}

fn gen_seq_from_letter_to_letter(
    pad: &HashMap<char, (isize, isize)>,
    start: char,
    end: char,
) -> Vec<String> {
    if start == end {
        return vec!["A".into()];
    }

    let (rs, cs) = pad.get(&start).unwrap();
    let (re, ce) = pad.get(&end).unwrap();
    let dr = re - rs;
    let dc = ce - cs;

    let rows = if dr >= 0 {
        repeat_n('v', dr as usize)
    } else {
        repeat_n('^', (-dr) as usize)
    };
    let cols = if dc >= 0 {
        repeat_n('>', dc as usize)
    } else {
        repeat_n('<', (-dc) as usize)
    };

    if dr == 0 {
        vec![cols.chain(once('A')).collect()]
    } else if dc == 0 {
        vec![rows.chain(once('A')).collect()]
    } else if empty_pad_field(pad, &(*rs, *ce)) {
        vec![rows.chain(cols).chain(once('A')).collect()]
    } else if empty_pad_field(pad, &(*re, *cs)) {
        vec![cols.chain(rows).chain(once('A')).collect()]
    } else {
        vec![
            rows.clone().chain(cols.clone()).chain(once('A')).collect(),
            cols.chain(rows).chain(once('A')).collect(),
        ]
    }
}

fn parse_input(input: &str) -> Result<Vec<String>> {
    Ok(input.lines().map(String::from).collect())
}

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/src/day_21/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;
    let codes = parse_input(&input)?;

    let complexity = codes.iter().fold(0, |acc, l| {
        let digits = l[..l.len() - 1].parse::<usize>().unwrap();
        let x = minimal_seq_len(&NUM_PAD, l.clone(), 3).unwrap();
        acc + x * digits
    });
    println!("Day 21, Part 1: Complexity with 3 robots: {complexity}");

    let complexity = codes.iter().fold(0, |acc, l| {
        let digits = l[..l.len() - 1].parse::<usize>().unwrap();
        let x = minimal_seq_len(&NUM_PAD, l.clone(), 26).unwrap();
        acc + x * digits
    });
    println!("Day 21, Part 2: Complexity with 26 robots: {complexity}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "029A
980A
179A
456A
379A";

    #[test]
    fn part_one() {
        let codes = parse_input(INPUT).unwrap();
        let result = codes.iter().fold(0, |acc, l| {
            let digits = l[..l.len() - 1].parse::<usize>().unwrap();
            let x = minimal_seq_len(&NUM_PAD, l.clone(), 3).unwrap();
            acc + x * digits
        });
        assert_eq!(result, 126384);
    }

    #[test]
    fn part_two() {
        let codes = parse_input(INPUT).unwrap();
        let result = codes.iter().fold(0, |acc, l| {
            let digits = l[..l.len() - 1].parse::<usize>().unwrap();
            let x = minimal_seq_len(&NUM_PAD, l.clone(), 26).unwrap();
            acc + x * digits
        });
        assert_eq!(result, 154115708116294);
    }
}
