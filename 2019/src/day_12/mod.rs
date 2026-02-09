use anyhow::{Context, Result};
use std::cmp::Ordering;

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/src/day_12/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;

    let mut moons = input
        .lines()
        .map(Moon::try_from)
        .collect::<Result<Vec<_>>>()
        .unwrap();
    simulate_moon_system(&mut moons, 1000);
    let total_energy = calc_total_energy(&moons);
    println!(
        "Day 12, Part 1: Total energy after 1000 steps: {}",
        total_energy
    );

    let mut moons = input
        .lines()
        .map(Moon::try_from)
        .collect::<Result<Vec<_>>>()
        .unwrap();
    let (period_x, period_y, period_z) = calc_periods(&mut moons).unwrap();
    let steps_until_all_at_start = lcm(period_x, lcm(period_y, period_z));
    println!(
        "Day 12, Part 2: Steps until system repeats: {}",
        steps_until_all_at_start
    );

    Ok(())
}

#[derive(Debug)]
struct Vector {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug)]
struct Moon {
    pos: Vector,
    vel: Vector,
}

impl Moon {
    fn apply_gravity(&mut self, other: &mut Moon) {
        match self.pos.x.cmp(&other.pos.x) {
            Ordering::Less => {
                self.vel.x += 1;
                other.vel.x -= 1;
            }
            Ordering::Greater => {
                self.vel.x -= 1;
                other.vel.x += 1;
            }
            Ordering::Equal => (),
        }

        match self.pos.y.cmp(&other.pos.y) {
            Ordering::Less => {
                self.vel.y += 1;
                other.vel.y -= 1;
            }
            Ordering::Greater => {
                self.vel.y -= 1;
                other.vel.y += 1;
            }
            Ordering::Equal => (),
        }

        match self.pos.z.cmp(&other.pos.z) {
            Ordering::Less => {
                self.vel.z += 1;
                other.vel.z -= 1;
            }
            Ordering::Greater => {
                self.vel.z -= 1;
                other.vel.z += 1;
            }
            Ordering::Equal => (),
        }
    }

    fn next_move(&mut self) {
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;
        self.pos.z += self.vel.z;
    }

    fn potential_energy(&self) -> i64 {
        self.pos.x.abs() + self.pos.y.abs() + self.pos.z.abs()
    }

    fn kinetic_energy(&self) -> i64 {
        self.vel.x.abs() + self.vel.y.abs() + self.vel.z.abs()
    }
}

impl TryFrom<&str> for Moon {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut c_parts = value[1..value.len() - 1]
            .split(", ")
            .map(|c| -> Option<i64> {
                let (_, val) = c.split_once('=')?;
                val.parse::<i64>().ok()
            });

        Ok(Self {
            pos: Vector {
                x: c_parts
                    .next()
                    .flatten()
                    .context("Invalid coordinate value")?,
                y: c_parts
                    .next()
                    .flatten()
                    .context("Invalid coordinate value")?,
                z: c_parts
                    .next()
                    .flatten()
                    .context("Invalid coordinate value")?,
            },
            vel: Vector { x: 0, y: 0, z: 0 },
        })
    }
}

fn simulate_moon_system(moons: &mut [Moon], steps: usize) {
    for _ in 0..steps {
        moon_system_step(moons);
    }
}

fn moon_system_step(moons: &mut [Moon]) {
    // Apply all gravities
    for a in 0..moons.len() {
        for b in a + 1..moons.len() {
            let (moon_a, moon_b) = get_pair_mut(moons, a, b).unwrap();
            moon_a.apply_gravity(moon_b);
        }
    }

    // Move all moons by their updated velocity
    for moon in moons.iter_mut() {
        moon.next_move();
    }
}

fn calc_total_energy(moons: &[Moon]) -> i64 {
    moons.iter().fold(0, |energy, moon| {
        energy + (moon.potential_energy() * moon.kinetic_energy())
    })
}

fn calc_periods(moons: &mut [Moon]) -> Option<(usize, usize, usize)> {
    let mut period_x: Option<usize> = None;
    let mut period_y: Option<usize> = None;
    let mut period_z: Option<usize> = None;

    let start_x = moons
        .iter()
        .map(|m| (m.pos.x, m.vel.x))
        .collect::<Vec<(i64, i64)>>();
    let start_y = moons
        .iter()
        .map(|m| (m.pos.y, m.vel.y))
        .collect::<Vec<(i64, i64)>>();
    let start_z = moons
        .iter()
        .map(|m| (m.pos.z, m.vel.z))
        .collect::<Vec<(i64, i64)>>();

    let mut steps = 0;
    while period_x.is_none() || period_y.is_none() || period_z.is_none() {
        moon_system_step(moons);
        steps += 1;

        if period_x.is_none() {
            let curr_x = moons
                .iter()
                .map(|m| (m.pos.x, m.vel.x))
                .collect::<Vec<(i64, i64)>>();
            if curr_x == start_x {
                period_x = Some(steps);
            }
        }
        if period_y.is_none() {
            let curr_y = moons
                .iter()
                .map(|m| (m.pos.y, m.vel.y))
                .collect::<Vec<(i64, i64)>>();
            if curr_y == start_y {
                period_y = Some(steps);
            }
        }
        if period_z.is_none() {
            let curr_z = moons
                .iter()
                .map(|m| (m.pos.z, m.vel.z))
                .collect::<Vec<(i64, i64)>>();
            if curr_z == start_z {
                period_z = Some(steps);
            }
        }
    }

    match (period_x, period_y, period_z) {
        (Some(x), Some(y), Some(z)) => Some((x, y, z)),
        _ => None,
    }
}

fn get_pair_mut<T>(slice: &mut [T], i: usize, j: usize) -> Option<(&mut T, &mut T)> {
    let (first, second) = (std::cmp::min(i, j), std::cmp::max(i, j));

    if i == j || second >= slice.len() {
        return None;
    }

    let (_, tmp) = slice.split_at_mut(first);
    let (x, rest) = tmp.split_at_mut(1);
    let (_, y) = rest.split_at_mut(second - first - 1);
    let pair = if i < j {
        (&mut x[0], &mut y[0])
    } else {
        (&mut y[0], &mut x[0])
    };

    Some(pair)
}

fn gcd(mut x: usize, mut y: usize) -> usize {
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

fn lcm(x: usize, y: usize) -> usize {
    x * y / gcd(x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>
"#;

    #[test]
    fn part_one() {
        let mut moons = INPUT
            .lines()
            .map(Moon::try_from)
            .collect::<Result<Vec<_>>>()
            .unwrap();
        simulate_moon_system(&mut moons, 100);
        let total_energy = calc_total_energy(&moons);
        assert_eq!(total_energy, 1940);
    }

    #[test]
    fn part_two() {
        let mut moons = INPUT
            .lines()
            .map(Moon::try_from)
            .collect::<Result<Vec<_>>>()
            .unwrap();

        let (period_x, period_y, period_z) = calc_periods(&mut moons).unwrap();
        let steps_until_all_at_start = lcm(period_x, lcm(period_y, period_z));
        assert_eq!(steps_until_all_at_start, 4686774924);
    }
}
