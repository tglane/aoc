use std::collections::VecDeque;
use regex::Regex;

#[derive(Clone)]
struct Coordinate {
    x: i32,
    y: i32,
    z: i32,
}

fn parse_input() -> VecDeque<Vec<Coordinate>> {
    let lines = include_str!("../in.txt")
        .split("\n\n")
        .filter(|line| *line != "")
        .map(|scanner_line| -> Vec<Coordinate> {
            scanner_line
                .split("\n")
                .filter(|line| !Regex::new(r"^---*").unwrap().is_match(line) && *line != "")
                .map(|coord_str| -> Coordinate {
                    let mut coords = coord_str.split(',');
                    Coordinate {
                        x: coords.next().unwrap().parse::<i32>().unwrap(),
                        y: coords.next().unwrap().parse::<i32>().unwrap(),
                        z: coords.next().unwrap().parse::<i32>().unwrap(),
                    }
                })
                .collect::<Vec<Coordinate>>()
        })
        .collect::<VecDeque<Vec<Coordinate>>>();
    lines
}

fn main() {
    let mut input = parse_input();

    let known_orientation = vec![Coordinate { x: 0, y: 0, z: 0 }; 1];
    let known_beacons = vec![input.pop_front().unwrap(); 1];
}
