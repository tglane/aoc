use std::collections::VecDeque;

#[derive(Clone)]
struct SnailfishNumber {
    value: i64,
    depth: usize,
}

impl SnailfishNumber {
    fn from(val: i64, dp: usize) -> Self {
        SnailfishNumber { value: val, depth: dp }
    }
}

fn explode(number: &mut Vec<SnailfishNumber>) -> bool {
    for i in 0..number.len()-1 {
        // Explode node that is nested inside of 4 nodes
        if number[i].depth == 5 && number[i+1].depth == 5 {
            if i > 0 {
                // Add left node value [i] to value to the left [i-1]
                number[i-1].value += number[i].value;
            }
            if i < number.len()-2 {
                // Add right node value [i+1] to value to the right [i+2]
                number[i+2].value += number[i+1].value;
            }

            // Emplace both values of the node with single value of 0 at a lower depth
            number[i] = SnailfishNumber::from(0, number[i+1].depth-1);
            number.remove(i+1);
            return true;
        }
    }
    false
}

fn split(number: &mut Vec<SnailfishNumber>) -> bool {
    for i in 0..number.len() {
        if number[i].value > 9 {
            // Split a node with value greater or equal to ten
            let left_val = number[i].value/2;
            let right_val = (number[i].value+1)/2;
            let new_depth = number[i].depth+1;

            number[i] = SnailfishNumber::from(left_val, new_depth);
            number.insert(i+1, SnailfishNumber::from(right_val, new_depth));
            return true;
        }
    }
    false
}

fn add(num_a: &[SnailfishNumber], num_b: &[SnailfishNumber]) -> Vec<SnailfishNumber> {
    let mut new_num = Vec::<SnailfishNumber>::new();
    new_num.extend_from_slice(&num_a);
    new_num.extend_from_slice(&num_b);

    for node in new_num.iter_mut() {
        node.depth += 1;
    }

    new_num
}

fn magnitude(mut number: Vec<SnailfishNumber>) -> i64 {
    while number.len() > 1 {
        for i in 0..number.len() {
            if i < number.len() - 1 && number[i].depth == number[i+1].depth {
                number[i] = SnailfishNumber::from(3*number[i].value + 2*number[i+1].value, number[i].depth-1);
                number.remove(i+1);
                break;
            }
        }
    }
    number[0].value
}

#[allow(dead_code)]
fn print_number(number: &Vec<SnailfishNumber>) {
    for node in number.iter() {
        println!("Val: {}, Depth: {}", node.value, node.depth);
    }
    println!("--------");
}

fn reduce(number: &mut Vec<SnailfishNumber>) {
    loop {
        if explode(number) {
            continue;
        }
        if !split(number) {
            return;
        }
    }
}

fn parse_input() -> VecDeque<Vec<SnailfishNumber>> {
    let mut lines = include_str!("../in.txt")
        .split('\n')
        .collect::<Vec<&str>>();
    lines.pop();

    let numbers = lines
        .iter()
        .map(|line| -> Vec<SnailfishNumber> {
            let mut number = Vec::<SnailfishNumber>::new();
            let mut depth = 0;

            for ch in line.chars() {
                match ch {
                    '[' => {
                        // Curr number gets new left node -> increase tree depth
                        depth += 1;
                    },
                    ',' => {
                        // Wait for second node of this number
                    },
                    ']' => {
                        // Curr number parsed -> decrease tree depth
                        depth -= 1;
                    },
                    val => {
                        // Set value of curr node
                        number.push(SnailfishNumber::from(val.to_digit(10).unwrap() as i64, depth as usize));
                    },
                }
            }

            number
        })
        .collect::<VecDeque<Vec<SnailfishNumber>>>();
    numbers
}

fn one(mut numbers: VecDeque<Vec<SnailfishNumber>>) {
    let mut curr_number = numbers.pop_front().unwrap();
    reduce(&mut curr_number);
    while numbers.len() > 0 {
        curr_number = add(&curr_number, &numbers.pop_front().unwrap());
        reduce(&mut curr_number);
    }
    let mag = magnitude(curr_number);
    println!("ONE: Magnitude = {}", mag);
}

fn two(numbers: &VecDeque<Vec<SnailfishNumber>>) {
    let mut max_magnitude = i64::MIN;
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i == j { continue };
            let mut new_num = add(&numbers[i], &numbers[j]);
            reduce(&mut new_num);
            let mag = magnitude(new_num);
            if mag > max_magnitude {
                max_magnitude = mag;
            }
        }
    }
    println!("TWO: Largest magnitude = {}", max_magnitude);
}

fn main() {
    let numbers = parse_input();

    one(numbers.clone());
    two(&numbers);
}
