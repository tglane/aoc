use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, PartialEq)]
enum Direction {
    Right,
    Down,
    No,
}

struct Field {
    data: Vec<Vec<Direction>>,
}

impl Field {
    #[allow(dead_code)]
    fn print(&self) {
        for line in self.data.iter() {
            for cell in line.iter() {
                print!("{}", match *cell {
                    Direction::Right => '>',
                    Direction::Down => 'v',
                    Direction::No => '.',
                });
            }
            println!("");
        }
        println!("--------");
    }

    fn step(&mut self) -> usize {
        let mut moved = 0_usize;
        let mut new_data = self.data.clone();

        let max_x = self.data[0].len();
        let max_y = self.data.len();

        // First move all right facing cocumbers
        for y in 0..max_y {
            for x in 0..max_x {
                if self.data[y][x] == Direction::Right {
                    let new_x = (x + 1) % max_x;
                    if self.data[y][new_x] == Direction::No {
                        new_data[y][new_x] = Direction::Right;
                        new_data[y][x] = Direction::No;
                        moved += 1;
                    } else {
                        new_data[y][x] = Direction::Right;
                    }
                }
            }
        }

        // Second move all down facing cocumbers
        for y in 0..self.data.len() {
            for x in 0..self.data[0].len() {
                if self.data[y][x] == Direction::Down {
                    let new_y = (y + 1) % max_y;
                    if new_data[new_y][x] == Direction::No && self.data[new_y][x] != Direction::Down {
                        new_data[new_y][x] = Direction::Down;
                        new_data[y][x] = Direction::No;
                        moved += 1;
                    }
                }
            }
        }

        self.data = new_data;
        moved
    }
}

fn parse_input(filename: &str) -> Result<Field, std::io::Error> {
    let reader = BufReader::new(File::open(filename)?);

    let data = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| -> Vec<Direction> {
            line
                .chars()
                .map(|c| -> Direction {
                    match c {
                        '>' => Direction::Right,
                        'v' => Direction::Down,
                        '.' => Direction::No,
                        _ => panic!("Unexpected input"),
                    }
                })
                .collect::<Vec<Direction>>()
        })
        .collect::<Vec<Vec<Direction>>>();

    Ok(Field { data })
}

fn main() {
    let filename = std::env::args().nth(1).expect("No filename given");
    let mut input = parse_input(&filename).expect("Failed to parse input");

    let mut steps = 1;
    while input.step() > 0 {
        steps += 1;
    }

    println!("First step no move occurs: {}", steps);
}
