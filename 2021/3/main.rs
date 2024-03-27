use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn file_to_vec(filename: &str) -> Result<Vec<Vec<u64>>, Error> {
    let line_reader = BufReader::new(File::open(filename)?);
    let out: Vec<Vec<u64>> = line_reader
        .lines()
        .map(|line| { line.unwrap().chars().flat_map(|ch| { ch.to_digit(10).map(u64::from) }).collect() })
        .collect();

    return Ok(out);
}

fn binary_array_to_num(bin_arr: &[u64]) -> u64 {
    let mut num: u64 = 0;
    for i in 0..bin_arr.len() {
        if bin_arr[bin_arr.len() - 1 - i] == 1 {
            num += 2_u64.pow(i as u32);
        }
    }
    return num;
}

#[derive(Copy, Clone)]
struct PosCounter {
    zeros: u64,
    ones: u64,
}

fn to_pos_counter(input: &Vec<Vec<u64>>) -> Vec<PosCounter> {
    let mut counter = vec![PosCounter { zeros: 0, ones: 0}; input[0].len()];
    for line in input.iter() {
        for i in 0..line.len() {
            if line[i] == 0 {
                counter[i].zeros += 1;
            } else if line[i] == 1 {
                counter[i].ones += 1;
            }
        }
    }
    return counter;
}

fn one(input: &Vec<Vec<u64>>) {
    let counter = to_pos_counter(&input);

    let mut gamma = vec![0; input[0].len()];
    let mut epsilon = vec![0; input[0].len()];
    for i in 0..counter.len() {
        if counter[i].zeros > counter[i].ones {
            gamma[i] = 0;
            epsilon[i] = 1;
        } else {
            gamma[i] = 1;
            epsilon[i] = 0;
        }
    }

    // println!("{:?}", gamma);
    // println!("{:?}", epsilon);

    let gamma_num = binary_array_to_num(&gamma);
    let epsilon_num = binary_array_to_num(&epsilon);

    println!("ONE: Gamma {} * Epsilon {} = {}", gamma_num, epsilon_num, gamma_num * epsilon_num);
}

fn two(input: &Vec<Vec<u64>>) {
    // Create copies for each value that can be permutated separately
    let mut ox_in = input.clone();
    let mut co2_in = input.clone();

    for i in 0..input[0].len() {
        if ox_in.len() > 1 {
            let counter_ox = to_pos_counter(&ox_in);

            // Keep most common bit for oxygen => 0
            if counter_ox[i].zeros > counter_ox[i].ones {
                ox_in.retain(|elem| elem[i] == 0);
            } else if counter_ox[i].zeros < counter_ox[i].ones {
                ox_in.retain(|elem| elem[i] == 1);
            } else {
                ox_in.retain(|elem| elem[i] == 1);
            }
        }
        if co2_in.len() > 1 {
            let counter_co2 = to_pos_counter(&co2_in);

            // Keep least common bit for co2 => 1
            if counter_co2[i].zeros > counter_co2[i].ones {
                co2_in.retain(|elem| elem[i] == 1);
            } else if counter_co2[i].zeros < counter_co2[i].ones {
                co2_in.retain(|elem| elem[i] == 0);
            } else {
                co2_in.retain(|elem| elem[i] == 0);
            }
        }
    }

    // println!("{:?}", ox_in);
    // println!("{:?}", co2_in);

    let ox = binary_array_to_num(&ox_in[0]);
    let co2 = binary_array_to_num(&co2_in[0]);

    println!("TWO: Oxygen {} * CO2 {} = {}", ox, co2, ox * co2);
}

fn main() {
    let input = file_to_vec("in.txt")
        .expect("Failed to read file");

    one(&input);
    two(&input);
}
