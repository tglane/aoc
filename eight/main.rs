use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::collections::HashMap;

#[allow(dead_code)]
struct Note {
    signal: Vec<String>,
    output: Vec<String>,
}

fn parse_input(filename: &str) -> Result<Vec<Note>, Error> {
    let reader = BufReader::new(File::open(filename)?);

    let out: Vec<Note> = reader
        .lines()
        .map(|line| -> Note {
            let unwrapped = line.unwrap();
            let mut first_split = unwrapped.split(" | ");
            let signals = first_split.next().unwrap().split_whitespace().map(str::to_string).collect::<Vec<String>>();
            let output = first_split.next().unwrap().split_whitespace().map(str::to_string).collect::<Vec<String>>();

            return Note { signal: signals, output: output };
        })
        .collect();

    Ok(out)
}

fn one(input: &[Note]) {
    let mut counter = 0;
    for note in input.iter() {
        for sig in note.output.iter() {
            match sig.len() {
                2 => counter += 1,
                3 => counter += 1,
                4 => counter += 1,
                7 => counter += 1,
                _ => (),
            }
        }
    }

    println!("ONE: Apperance of 1, 4, 7 and 8 in the notes is {}", counter);
}

fn two(input: &[Note]) {
    let mut counter = 0_i64;
    for note in input.iter() {
        let mapping = generate_digit_map(&note.signal);

        for (k, v) in mapping.iter() {
            println!("{} => {}", k, v);
        }

        for pat in note.output.iter() {
            println!("Pat: {}", pat);
            counter += mapping[pat] as i64;
        }
    }
    println!("TWO: Sum of all outputs is {}", counter);
}

fn generate_digit_map(keys: &[String]) -> HashMap<String, i32> {
    let check_if_five = |pattern: &str, four_pattern: &str| -> bool {
        let mut count = 0;
        for ch in four_pattern.chars() {
            if pattern.contains(ch) {
                count += 1;
            }
        }
        if count == 3 {
            return true;
        } else {
            return false;
        }
    };

    let mut mapping = HashMap::<i32, String>::new();

    for pat in keys.iter() {
        match pat.len() {
            2 => { mapping.insert(1, pat.to_string()); },
            3 => { mapping.insert(7, pat.to_string()); },
            4 => { mapping.insert(4, pat.to_string()); },
            7 => { mapping.insert(8, pat.to_string()); },
            _ => (),
        }
    }

    for pat in keys.iter() {
        match pat.len() {
            6 => {
                if mapping.get(&4).unwrap().chars().all(|ch| pat.contains(ch)) {
                    mapping.insert(9, pat.to_string());
                } else if mapping.get(&1).unwrap().chars().all(|ch| pat.contains(ch)) {
                    mapping.insert(0, pat.to_string());
                } else {
                    mapping.insert(6, pat.to_string());
                }
            },
            5 => {
                if mapping.get(&1).unwrap().chars().all(|ch| pat.contains(ch)) {
                    mapping.insert(3, pat.to_string());
                } else if check_if_five(&pat, &mapping.get(&4).unwrap()) {
                    mapping.insert(5, pat.to_string());
                } else {
                    mapping.insert(2, pat.to_string());
                }
            },
            _ => (),
        }
    }

    let mut str_to_int = HashMap::<String, i32>::new();
    for (k, v) in mapping.iter() {
        str_to_int.insert(v.clone(), *k);
    }
    return str_to_int;
}

fn main() {
    let input = parse_input("in.txt")
        .expect("Failed to parse input");

    one(&input);
    two(&input);
}
