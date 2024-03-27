use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use scan_fmt::scan_fmt;

struct TargetArea {
    x_range: (i32, i32),
    y_range: (i32, i32),
}

#[allow(dead_code)]
struct Velocity {
    x: i32,
    y: i32,
}

fn parse_input(filename: &str) -> Result<TargetArea, Error> {
    let reader = BufReader::new(File::open(filename)?);
    let line = reader.lines().next().unwrap().unwrap();
    let (x_min, x_max, y_min, y_max) = scan_fmt!(&line, "target area: x={}..{}, y={}..{}", i32, i32, i32, i32).unwrap();
    Ok(TargetArea { x_range: (x_min, x_max), y_range: (y_min, y_max) })
}

fn get_velocity_values(target_area: &TargetArea) -> (Vec<Velocity>, i32) {
    let mut velocities = Vec::<Velocity>::new();
    let mut max_height = 0;

    for x in 0..=target_area.x_range.1 {
        for y in target_area.y_range.0..10*(target_area.y_range.1-target_area.y_range.0) {
            if let Some(height) = simulate_trajectory((0, 0), x, y, &target_area) {
                velocities.push(Velocity { x, y });
                if height > max_height {
                    max_height = height;
                }
            }
        }
    }

    (velocities, max_height)
}

fn simulate_trajectory(start: (i32, i32), mut x_vel: i32, mut y_vel: i32, target: &TargetArea) -> Option<i32> {
    let mut x = start.0;
    let mut y = start.1;

    let mut max_height = 0;
    while y >= target.y_range.0 && !(x_vel == 0 && (x < target.x_range.0 || x > target.x_range.1)) {
        if x >= target.x_range.0 && x <= target.x_range.1 && y >= target.y_range.0 && y <= target.y_range.1 {
            return Some(max_height);
        }

        max_height = std::cmp::max(y, max_height);

        x += x_vel;
        y += y_vel;

        y_vel -= 1;
        x_vel = (x_vel - 1).max(0);
    }

    None
  }

fn main() {
    let target_area = parse_input("in.txt")
        .expect("Failed to parse input");

    let (velocities, max) = get_velocity_values(&target_area);
    println!("Number of possible velocity values = {} - Max = {}", velocities.len(), max);
    // for vel in velocities.iter() {
    //     println!("{},{}", vel.x, vel.y);
    // }
}
