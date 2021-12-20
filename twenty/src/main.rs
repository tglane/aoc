use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::collections::HashSet;

#[derive(Eq, PartialEq)]
struct Pixel {
    x: i32,
    y: i32,
}

impl std::hash::Hash for Pixel {
    fn hash<H>(&self, state: &mut H)
        where H: std::hash::Hasher
    {
        let bin: i64 = ((self.y as i64) << 32) ^ self.x as i64;
        state.write_i64(bin);
        state.finish();
    }
}

fn parse_input(input: &str) -> Result<(Vec<bool>, HashSet<Pixel>), Error> {
    let reader = BufReader::new(File::open(input)?);
    let mut lines = reader.lines();

    // Parse algorithm line
    let line = lines.next();
    let algo = line.unwrap().unwrap().chars().map(|c| if c == '#' { true } else { false }).collect::<Vec<bool>>();
    lines.next();

    // Parse pixel field
    let mut pixels = HashSet::<Pixel>::new();
    for (y, line) in lines.enumerate() {
        for (x, c) in line.unwrap().chars().enumerate() {
            if c == '#' {
                pixels.insert(Pixel { x: x as i32, y: y as i32 });
            }
        }
    }

    Ok((algo, pixels))
}

fn enhance(pixels: &HashSet<Pixel>, algorithm: &[bool], step: u32) -> HashSet<Pixel> {
    let mut new_pixels = HashSet::<Pixel>::new();

    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    for pixel in pixels.iter() {
        min_x = std::cmp::min(min_x, pixel.x);
        max_x = std::cmp::max(max_x, pixel.x);
        min_y = std::cmp::min(min_y, pixel.y);
        max_y = std::cmp::max(max_y, pixel.y);
    }

    for y in min_y-1..=max_y+1 {
        for x in min_x-1..=max_x+1 {
            let mut bin = 0_u16;

            for curr_y in y-1..=y+1 {
                for curr_x in x-1..=x+1 {
                    if curr_x >= min_x && curr_x <= max_x && curr_y >= min_y && curr_y <= max_y {
                        if pixels.contains(&Pixel { x: curr_x, y: curr_y}) {
                            bin = bin << 1 | 1;
                        } else {
                            bin = bin << 1 | 0
                        }
                    } else {
                        if step % 2 == 1 {
                            bin = bin << 1 | 1;
                        } else {
                            bin = bin << 1 | 0;
                        }
                    }
                }
            }

            if algorithm[bin as usize] {
                new_pixels.insert(Pixel {x, y});
            }
        }
    }

    new_pixels
}

#[allow(dead_code)]
fn print_image(pixels: &HashSet<Pixel>) {
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    for pixel in pixels.iter() {
        min_x = std::cmp::min(min_x, pixel.x);
        max_x = std::cmp::max(max_x, pixel.x);
        min_y = std::cmp::min(min_y, pixel.y);
        max_y = std::cmp::max(max_y, pixel.y);
    }

    for y in min_y-1..=max_y+1 {
        for x in min_x-1..=max_x+1 {
            print!("{}", if let Some(_) = pixels.get(&Pixel { x, y }) { '#' } else { '.' });
        }
        println!("");
    }
    println!("-------------------");
}

fn count_lit(pixels: &HashSet<Pixel>) -> usize {
    pixels.len()
}

fn main() {
    let input_filename = std::env::args().nth(1).expect("No filename given");
    let (algo, mut pixels) = parse_input(&input_filename).expect("Failed to parse input");

    let steps = 50;
    for i in 0..steps {
        pixels = enhance(&pixels, &algo, i);
    }
    println!("Lit pixels after {} steps: {}", steps, count_lit(&pixels));
}
