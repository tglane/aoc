use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn file_to_vec(filename: &str) -> Result<Vec<u64>, Error> {
    let line_reader = BufReader::new(File::open(filename)?);
    let out: Vec<u64> = line_reader
        .lines()
        .map(|line| line.unwrap().parse::<u64>().unwrap())
        .collect();

    return Ok(out);
}

fn one(input: &Vec<u64>) {
    let mut increased: u64 = 0;
    let mut decreased: u64 = 0;
    for i in 1..input.len() {
        println!("Comparing {} with {}", input[i], input[i - 1]);

        if input[i] > input[i - 1] {
            increased += 1;
        } else {
            decreased += 1;
        }
    }

    println!("ONE: Increased: {} -- Decreased: {}", increased, decreased);
}

fn two(input: &Vec<u64>) {
    let mut increased: u64 = 0;
    let mut decreased: u64 = 0;
    let mut last_window = input[0] + input[1] + input[2];
    for i in 1..input.len() - 2 {
        let curr_window = last_window - input[i - 1] + input[i + 2];

        println!("Comparing {} with {}", curr_window, last_window);

        if curr_window > last_window {
            increased += 1;
        } else {
            decreased += 1;
        }

        last_window = curr_window;
    }

    println!("TWO: Increased: {} -- Decreased: {}", increased, decreased);
}

fn main() {
    let input = file_to_vec("in.txt")
        .expect("Failed to read file");

    one(&input);
    two(&input);
}
