use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::collections::HashMap;

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
    const GOAL: usize = 1000;
    let mut score_one = 0;
    let mut score_two = 0;

    let mut it = 0;
    let mut dice_val = 0;
    while score_one < GOAL && score_two < GOAL {
        if it % 2 == 0 {
            // Player one
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

// State = (pos_one, score_one, pos_two, score_two)
type State = (usize, usize, usize, usize);

fn quantum_dirac(pos_one: usize, pos_two: usize) {
    let mut universe_store = HashMap::<State, usize>::from([((pos_one-1, 0, pos_two-1, 0), 1)]);

    let (mut wins_one, mut wins_two) = (0_usize, 0_usize);
    while !universe_store.is_empty() {
        for i in 1_usize..=2_usize {
            let mut new_universe_store = HashMap::<State, usize>::new();

            for (state, count) in universe_store.iter() {
                // Roll the dice!
                for first_d in 1..=3 {
                    for sec_d in 1..=3 {
                        for third_d in 1..=3 {
                            if i == 1 {
                                // Player 1
                                let new_pos = (state.0 + first_d + sec_d + third_d) % 10;
                                let new_score = state.1 + new_pos + 1;

                                if new_score >= 21 {
                                    wins_one += *count;
                                } else {
                                    new_universe_store
                                        .entry((new_pos, new_score, state.2, state.3))
                                        .and_modify(|c| *c += *count)
                                        .or_insert(*count);
                                }
                            } else {
                                // Player 2
                                let new_pos = (state.2 + first_d + sec_d + third_d) % 10;
                                let new_score = state.3 + new_pos + 1;

                                if new_score >= 21 {
                                    wins_two += *count;
                                } else {
                                    new_universe_store
                                        .entry((state.0, state.1, new_pos, new_score))
                                        .and_modify(|c| *c += *count)
                                        .or_insert(*count);
                                }
                            }
                        }
                    }
                }
            }

            universe_store = new_universe_store;
        }
    }

    println!("Wins one: {} - Wins two: {}", wins_one, wins_two);
    println!("Most wins are {}", std::cmp::max(wins_one, wins_two));
}

fn main() {
    let filename = std::env::args().nth(1).expect("No filename given");
    let (one, two) = parse_input(&filename).expect("Failed to parse input");

    deterministic_dirac(one, two);

    quantum_dirac(one, two);
}
