use std::fs::File;
use std::io::{BufRead, BufReader, Error};

struct Command {
    direction: String,
    count: u64,
}

fn file_to_vec(filename: &str) -> Result<Vec<Command>, Error> {
    let line_reader = BufReader::new(File::open(filename)?);
    let out: Vec<Command> = line_reader
        .lines()
        .map(|line| -> Command {
                let unwrapped_line = line.unwrap();
                let splitted_line = unwrapped_line.split(" ");
                let splitted_vec = splitted_line.collect::<Vec<&str>>();
                return Command { direction: splitted_vec[0].to_string(), count: splitted_vec[1].parse::<u64>().unwrap() };
            })
        .collect();

    return Ok(out);
}

fn one(input: &Vec<Command>) {
    let mut depth = 0;
    let mut pos = 0;
    for line in input.iter() {
        match &line.direction[..] {
            "forward" => {
                pos += line.count;
            },
            "down" => {
                depth += line.count;
            }
            "up" => {
                depth -= line.count;
            },
            &_ => println!("No matched command"),
        }
    }

    println!("ONE: Pos: {} - Depth: {} => {}", pos, depth, pos * depth);
}

fn two(input: &Vec<Command>) {
    let mut depth = 0;
    let mut pos = 0;
    let mut aim = 0;
    for line in input.iter() {
        match &line.direction[..] {
            "forward" => {
                pos += line.count;
                depth += aim * line.count;
            },
            "down" => {
                aim += line.count;
            }
            "up" => {
                aim -= line.count;
            },
            &_ => println!("No matched command"),
        }
    }

    println!("TWO: Pos: {} - Depth: {} => {}", pos, depth, pos * depth);
}

fn main() {
    let input = file_to_vec("in.txt")
        .expect("Failed to read file");

    one(&input);
    two(&input);
}
