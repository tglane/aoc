use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::collections::HashMap;

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

fn parse_input(filename: &str) -> Result<Vec<u32>, Error> {
    let reader = BufReader::new(File::open(filename)?);

    let out = reader
        .lines().next().unwrap().unwrap()
        .split(',').map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    return Ok(out);
}

fn naive(mut input: Vec<u32>, days: u32) {
    for _ in 1..days+1 {
        for i in 0..input.len() {
            if input[i] == 0 {
                input[i] = 6;
                input.push(8);
            } else {
                input[i] -= 1;
            }
        }
    }

    println!("[Navive] Len after {} iterations: {}", days, input.len());
}

fn advanced(input: Vec<u32>, days: u32) {
    // Create map to count elements by their days left to produce a new element
    let mut counter: HashMap<u32, u64> = hashmap![0 => 0, 1 => 0, 2 => 0, 3 => 0, 4 => 0, 5 => 0, 6 => 0, 7 => 0, 8 => 0];
    for item in input.iter() {
        counter.entry(*item).and_modify(|elem| *elem += 1);
    }

    // Apply changes per iteratiion/day
    for _ in 0..days {
        let new_elems = counter[&0];
        counter.entry(0).and_modify(|elem| *elem = 0);

        for i in 1..8+1 {
            // Soll: counter[i-1] = counter[i];
            let mutate = counter[&i];
            counter.entry(i - 1).and_modify(|elem| *elem = mutate);
        }

        counter.entry(8).and_modify(|elem| *elem = new_elems);
        counter.entry(6).and_modify(|elem| *elem += new_elems);
    }

    // Get the final count of elements from all buckets
    let mut count = 0;
    for (_, val) in counter.iter() {
        count += val;
    }

    println!("[Advanced] Len after {} iterations: {}", days, count);
}

fn main() {
    let input = parse_input("in.txt")
        .expect("Failed to parse input");

    naive(input.clone(), 80);
    advanced(input.clone(), 80);
    advanced(input.clone(), 256);
}
