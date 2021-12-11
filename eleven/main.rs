use std::fs::File;
use std::io::{BufRead, BufReader, Error};

#[derive(Clone)]
struct Octopus {
    energy: u64,
    flashed: bool,
}

fn parse_input(filename: &str) -> Result<Vec<Vec<Octopus>>, Error> {
    let reader = BufReader::new(File::open(filename)?);

    let out = reader
        .lines()
        .map(|line| -> Vec<Octopus> {
            line.unwrap()
                .chars()
                .map(|ch| -> Octopus {
                    Octopus { energy: ch.to_digit(10).unwrap() as u64, flashed: false }
                })
                .collect()
        })
        .collect::<Vec<Vec<Octopus>>>();
    Ok(out)
}

fn for_every_oct(oct: &mut Vec<Vec<Octopus>>, op: &mut dyn FnMut(&mut Octopus)) {
    for x in 0..oct.len() {
        for y in 0..oct[x].len() {
            op(&mut oct[x][y]);
        }
    }
}

fn flash(mut oct: &mut Vec<Vec<Octopus>>, x: i64, y: i64) {
    if !oct[x as usize][y as usize].flashed && oct[x as usize][y as usize].energy > 9 {
        oct[x as usize][y as usize].flashed = true;

        // Inc energy of adjacent oct
        let x_min = std::cmp::max(0, x - 1);
        let x_max = std::cmp::min(x + 1, oct.len() as i64 - 1);
        let y_min = std::cmp::max(0, y - 1);
        let y_max = std::cmp::min(y + 1, oct[0].len() as i64 - 1);
        for x_inc in x_min..x_max + 1 {
            for y_inc in y_min..y_max + 1 {
                if x_inc == x && y_inc == y {
                    continue;
                }

                oct[x_inc as usize][y_inc as usize].energy += 1;
                flash(&mut oct, x_inc, y_inc);
            }
        }
    }
}

fn one(mut input: Vec<Vec<Octopus>>, steps: u64) {
    let mut flashes = 0_u64;
    for _ in 0..steps {
        for_every_oct(&mut input, &mut |oct: &mut Octopus| oct.energy += 1);

        for x in 0..input.len() {
            for y in 0..input[x].len() {
                flash(&mut input, x as i64, y as i64);
            }
        }

        for_every_oct(&mut input, &mut |oct: &mut Octopus| {
            if oct.flashed {
                oct.flashed = false;
                oct.energy = 0;
                flashes += 1;
            }
        });
    }

    println!("ONE: Flashes in {} steps: {}", steps, flashes);
}

fn two(mut input: Vec<Vec<Octopus>>) {
    let mut flashes = 0_u64;
    let mut iterations = 0_u64;
    while flashes != (input.len() * input[0].len()) as u64 {
        iterations += 1;
        flashes = 0;
        for_every_oct(&mut input, &mut |oct: &mut Octopus| oct.energy += 1);

        for x in 0..input.len() {
            for y in 0..input[x].len() {
                flash(&mut input, x as i64, y as i64);
            }
        }

        for_every_oct(&mut input, &mut |oct: &mut Octopus| {
            if oct.flashed {
                oct.flashed = false;
                oct.energy = 0;
                flashes += 1;
            }
        });
    }

    println!("TWO: Steps until all octs flash: {}", iterations);
}

fn main() {
    let input = parse_input("in.txt")
        .expect("Failed to parse input");

    one(input.clone(), 100);
    two(input);
}
