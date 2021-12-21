use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn parse_input(filename: &str) -> Result<(usize, usize), Error> {
    let reader = BufReader::new(File::open(filename)?);

    let tmp = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| -> usize {
            let line = line.split(' ');
            line.last().unwrap().parse::<usize>().unwrap()
        })
        .collect::<Vec<_>>();

    Ok((tmp[0], tmp[1]))
}

fn deterministic_dirac(mut pos_one: usize, mut pos_two: usize) {
    let goal = 1000;
    let mut score_one = 0;
    let mut score_two = 0;

    let mut it = 0;
    let mut dice_val = 0;
    while score_one < goal && score_two < goal {
        if it % 2 == 0 {
            // Plyer one
            pos_one = (pos_one + 3 * dice_val + 5) % 10 + 1;
            score_one += pos_one;
        } else {
            // Player two
            pos_two = (pos_two + 3 * dice_val + 5) % 10 + 1;
            score_two += pos_two;
        }

        it += 1;
        dice_val += 3;
    }

    println!("Score One: {} - Score Two: {} - Dice rolled: {}", score_one, score_two, it*3);
    if score_one > score_two {
        println!("ONE: {}", score_two*it*3);
    } else {
        println!("ONE: {}", score_one*it*3);
    }
}

fn main() {
    let filename = std::env::args().nth(1).expect("No filename given");
    let (one, two) = parse_input(&filename).expect("Failed to parse input");
    println!("{}, {}", one, two);

    deterministic_dirac(one, two);
}
