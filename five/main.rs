use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::collections::HashMap;

struct Point {
    x: i64,
    y: i64,
}

#[allow(dead_code)]
struct Input {
    operations: Vec<(Point, Point)>,
    max_x: i64,
    max_y: i64,
}

fn parse_input(filename: &str) -> Result<Input, Error> {
    // Input format for each line: x1, y1 -> x2, y2
    let line_reader = BufReader::new(File::open(filename)?);

    let mut max_x = 0;
    let mut max_y = 0;
    let operations = line_reader
        .lines()
        .map(|line| -> (Point, Point) {
            let unwrapped = line.unwrap();
            let split = unwrapped.split(" -> ").collect::<Vec<&str>>();

            let mut first_iter = split[0].splitn(2, ',');
            let mut sec_iter = split[1].splitn(2, ',');

            let first_point = Point{ x: first_iter.next().unwrap().parse::<i64>().unwrap(), y: first_iter.next().unwrap().parse::<i64>().unwrap() };
            let sec_point = Point{ x: sec_iter.next().unwrap().parse::<i64>().unwrap(), y: sec_iter.next().unwrap().parse::<i64>().unwrap() };

            max_x = std::cmp::max(std::cmp::max(first_point.x, sec_point.x), max_x);
            max_y = std::cmp::max(std::cmp::max(first_point.y, sec_point.y), max_y);

            return (first_point, sec_point);
        })
        .collect::<Vec<(Point, Point)>>();

    return Ok(Input{ operations: operations, max_x: max_x, max_y: max_y });
}

fn one(input: &Input) {
    let mut heat_map = HashMap::<(i64, i64), u32>::new();

    for line in input.operations.iter() {
        if line.0.x == line.1.x || line.0.y == line.1.y {
            let start_x = std::cmp::min(line.0.x, line.1.x);
            let end_x = std::cmp::max(line.0.x, line.1.x);
            for x in start_x..end_x+1 {
                let start_y = std::cmp::min(line.0.y, line.1.y);
                let end_y = std::cmp::max(line.0.y, line.1.y);
                for y in start_y..end_y+1 {
                    heat_map.entry((x, y)).and_modify(|e| *e += 1).or_insert(1);
                }
            }
        }
    }

    heat_map.retain(|_, val| *val >= 2);
    // for (key, val) in heat_map.iter() {
    //     println!("({},{}) -> {}", key.0, key.1, val);
    // }
    println!("ONE: {} Entries with a value greater thatn 2", heat_map.len());
}

fn two(input: &Input) {
    let mut heat_map = HashMap::<(i64, i64), u32>::new();

    let diagnoal = |first: &Point, sec: &Point| -> bool {
        return (first.x - sec.x).abs() == (first.y - sec.y).abs();
    };

    for line in input.operations.iter() {
        if line.0.x == line.1.x && line.0.y != line.1.y {
            // Vertical line
            let x = line.0.x;
            let min = std::cmp::min(line.0.y, line.1.y);
            let max = std::cmp::max(line.0.y, line.1.y);
            for y in min..=max {
                heat_map
                    .entry((x, y))
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
        } else if line.0.x != line.1.x && line.0.y == line.1.y {
            // Horizontal line
            let y = line.0.y;
            let min = std::cmp::min(line.0.x, line.1.x);
            let max = std::cmp::max(line.0.x, line.1.x);
            for x in min..=max {
                heat_map
                    .entry((x, y))
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
        } else if diagnoal(&line.0, &line.1) {
            // Diagonal line
            let dx = (line.1.x - line.0.x).signum();
            let dy = (line.1.y - line.0.y).signum();
            for i in 0..=(line.1.x - line.0.x).abs() {
                heat_map
                    .entry((line.0.x + i * dx, line.0.y + i * dy))
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
        }
    }

    heat_map.retain(|_, val| *val >= 2);
    // for (key, val) in heat_map.iter() {
    //     println!("({},{}) -> {}", key.0, key.1, val);
    // }
    println!("TWO: {} Entries with a value greater thatn 2", heat_map.len());
}

fn main() {
    let input = parse_input("in.txt")
        .expect("Failed to parse input");

    one(&input);
    two(&input);
}
