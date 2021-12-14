use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::collections::HashMap;

fn parse_input(filename: &str) -> Result<(String, HashMap<(char, char), char>), Error> {
    let reader = BufReader::new(File::open(filename)?);
    let mut lines = reader.lines();

    let template = lines.next().unwrap().unwrap();
    lines.next();

    let rules: HashMap<(char, char), char> = lines
        .map(|line| -> ((char, char), char) {
            let line = line.unwrap();
            let mut line = line.split(" -> ");
            let rule_pair = line.next().unwrap();
            ((rule_pair.chars().nth(0).unwrap(), rule_pair.chars().nth(1).unwrap()), line.next().unwrap().chars().next().unwrap())
        })
        .collect();

    Ok((template, rules))
}

fn get_min_max(char_occurences: &HashMap<char, u64>) -> (u64, u64) {
    let max_tup = char_occurences
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap();
    let min_tup = char_occurences
        .iter()
        .min_by(|a, b| a.1.cmp(&b.1))
        .unwrap();
    (*min_tup.1, *max_tup.1)
}

fn solve(sequence: &str, rules: &HashMap<(char, char), char>, steps: u64) {
    let mut char_occurences = sequence
        .chars()
        .fold(HashMap::<char, u64>::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });

    let mut pair_occurences = HashMap::<(char, char), u64>::new();
    for i in 0..sequence.len()-1 {
        *pair_occurences
            .entry((sequence.chars().nth(i).unwrap(), sequence.chars().nth(i+1).unwrap()))
            .or_insert(0) += 1;
    }

    for _ in 0..steps {
        let mut new_pairs = HashMap::<(char, char), u64>::new();
        for pair in pair_occurences.iter() {
            if let Some(rule) = rules.get(pair.0) {
                *char_occurences.entry(*rule).or_insert(0) += pair.1;

                *new_pairs.entry((pair.0.0, *rule)).or_insert(0) += pair.1;
                *new_pairs.entry((*rule, pair.0.1)).or_insert(0) += pair.1;
            } else {
                *new_pairs.entry(*pair.0).or_insert(0) += pair.1;
            }
        }

        pair_occurences = new_pairs;
   }

    let (min, max) = get_min_max(&char_occurences);
    println!("{} - {} = {}", max, min, max - min);
}

fn main() {
    let (template, rules) = parse_input("in.txt")
        .expect("Failed to parse input");

    solve(&template, &rules, 10);
    solve(&template, &rules, 40);
}
