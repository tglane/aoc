use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::collections::HashMap;

fn parse_input(filename: &str) -> Result<Vec<String>, Error> {
    let reader = BufReader::new(File::open(filename)?);
    let out = reader
        .lines()
        .map(|line| line.unwrap())
        .collect();
    Ok(out)
}

fn one(input: &mut Vec<String>) -> u64 {
    let closing_brackets: HashMap<char, char> = HashMap::<char, char>::from([
        (')', '('),
        (']', '['),
        ('}', '{'),
        ('>', '<'),
    ]);

    let mut error_count = 0_u64;
    input.retain(|line| {
        let mut tokens = Vec::<char>::new();
        for ch in line.chars() {
            match ch {
                '(' | '[' | '{' | '<' => {
                    tokens.push(ch);
                },
                ')' | ']' | '}' | '>' => {
                    if tokens.pop().unwrap() != closing_brackets[&ch] {
                        error_count += match ch {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => 0,
                        };
                        return false;
                    }
                },
                _ => (),
            }
        }
        return true;
    });

    error_count
}

fn two(input: &mut Vec<String>) -> u64 {
    let opposite_brackets: HashMap<char, char> = HashMap::<char, char>::from([
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>'),
    ]);

    let mut completion_counts = Vec::<u64>::new();
    for line in input.iter_mut() {
        // Get list of opening tokens
        let mut tokens = Vec::<char>::new();
        for ch in line.chars() {
            match ch {
                '(' | '[' | '{' | '<' => {
                    tokens.push(ch);
                },
                ')' | ']' | '}' | '>' => {
                    tokens.pop().unwrap();
                },
                _ => (),
            }
        }

        // Append missing tokens to close open tokens
        let mut completion_count = 0_u64;
        for token in tokens.iter().rev() {
            let completion_char = opposite_brackets[&token];
            line.push(completion_char);
            completion_count = (completion_count * 5) + match completion_char {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => 0,
            };
        }
        completion_counts.push(completion_count);
    }

    completion_counts.sort();
    completion_counts[completion_counts.len() / 2]
}

fn main() {
    let mut input = parse_input("in.txt")
        .expect("Failed to parse input");

    let error_count = one(&mut input);
    println!("ONE: Error count: {}", error_count);

    let completion_count = two(&mut input);
    println!("TWO: Completion count: {}", completion_count);
}
