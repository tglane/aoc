use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn parse_input(filename: &str) -> Result<Vec<Vec<u32>>, Error> {
    let reader = BufReader::new(File::open(filename)?);

    let out: Vec<Vec<u32>> = reader
        .lines()
        .map(|line| { line.unwrap().chars().flat_map(|ch| { ch.to_digit(10).map(u32::from) }).collect() })
        .collect();

    return Ok(out);
}

#[derive(Copy, Clone)]
struct Point {
    x: u64,
    y: u64,
    height: u64,
    in_basin: bool,
}

impl Point {
    fn new() -> Point {
        return Point { x: 0, y: 0, height: 0, in_basin: false };
    }

    fn from_coords(x: u64, y: u64) -> Point {
        return Point { x: x, y: y, height: 0, in_basin: false };
    }
}

fn task(input: &Vec<Vec<u32>>) {
    let mut heightmap = vec![vec![Point::new(); input[0].len()]; input.len()];
    let mut low_points = Vec::<Point>::new();

    let mut sum_of_lowest = 0_u64;
    for x in 0..input.len() {
        for y in 0..input[x].len() {
            heightmap[x][y].x = x as u64;
            heightmap[x][y].y = y as u64;
            heightmap[x][y].height = input[x][y] as u64;

            if y > 0 && input[x][y - 1] <= input[x][y] {
                continue;
            }
            if y != input[x].len() - 1 && input[x][y + 1] <= input[x][y] {
                continue;
            }
            if x > 0 && input[x - 1][y] <= input[x][y] {
                continue;
            }
            if x != input.len() - 1 && input[x + 1][y] <= input[x][y] {
                continue;
            }

            sum_of_lowest += input[x][y] as u64 + 1;
            low_points.push(Point::from_coords(x as u64, y as u64));
        }
    }

    println!("ONE: Sum of low points: {}", sum_of_lowest);

    let mut basins = Vec::<u64>::new();
    for point in low_points.iter() {
        heightmap[point.x as usize][point.y as usize].in_basin = true;
        basins.push(1);
        let index = basins.len() - 1;

        let_it_flow(&mut heightmap, &mut basins, point.x as usize, point.y as usize, index);
    }

    basins.sort();
    assert!(basins.len() >= 3);
    let largest_basins_mult = basins[basins.len() - 1] * basins[basins.len() - 2] * basins[basins.len() - 3];
    println!("TOW: Multiply three largest basins: {}", largest_basins_mult);
}

fn let_it_flow(mut heightmap: &mut Vec<Vec<Point>>, mut basins: &mut Vec<u64>, x: usize, y: usize, basin_index: usize) {
    if y > 0 && !heightmap[x][y - 1].in_basin && heightmap[x][y - 1].height != 9 {
        basins[basin_index] += 1;
        heightmap[x][y - 1].in_basin = true;
        let_it_flow(&mut heightmap, &mut basins, x, y - 1, basin_index);
    }
    if y != heightmap[x].len() - 1 && !heightmap[x][y + 1].in_basin && heightmap[x][y + 1].height != 9 {
        basins[basin_index] += 1;
        heightmap[x][y + 1].in_basin = true;
        let_it_flow(&mut heightmap, &mut basins, x, y + 1, basin_index);
    }
    if x > 0 && !heightmap[x - 1][y].in_basin && heightmap[x - 1][y].height != 9 {
        basins[basin_index] += 1;
        heightmap[x - 1][y].in_basin = true;
        let_it_flow(&mut heightmap, &mut basins, x - 1, y, basin_index);
    }
    if x != heightmap.len() - 1 && !heightmap[x + 1][y].in_basin && heightmap[x + 1][y].height != 9 {
        basins[basin_index] += 1;
        heightmap[x + 1][y].in_basin = true;
        let_it_flow(&mut heightmap, &mut basins, x + 1, y, basin_index);
    }
}

fn main() {
    let input = parse_input("in.txt")
        .expect("Failed to parse input");

    task(&input);
}
