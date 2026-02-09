use anyhow::{Context, Result};
use ordered_float::OrderedFloat;
use std::collections::{HashMap, HashSet};

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/src/day_10/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;

    let asteroids = parse_asteroids(&input);
    let (best_asteroid, visible_from_best) = asteroids
        .iter()
        .map(|asteroid| (asteroid, asteroid.count_visible(&asteroids)))
        .max_by_key(|(_, visible)| *visible)
        .context("No other asteroids visible from any asteroid")?;
    println!(
        "Day 10, Part 1: Max visible other asteroids: {}",
        visible_from_best
    );

    let vaporizer_list = calc_vaporizer_list(best_asteroid, &asteroids);
    let asteroid_200_prod = vaporizer_list[199].x * 100 + vaporizer_list[199].y;
    println!(
        "Day 10, Part 2: Product of coordinates of the 200th asteroid to vaporize: {}",
        asteroid_200_prod
    );

    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn dist(&self, other: &Point) -> f64 {
        let x_delta = self.x - other.x;
        let y_delta = self.y - other.y;
        ((x_delta.pow(2) + y_delta.pow(2)) as f64).sqrt()
    }

    fn angle(&self, other: &Point) -> f64 {
        const RADIAN_TO_DEGREE: f64 = 180.0 / std::f64::consts::PI;
        180.0 - (RADIAN_TO_DEGREE * ((other.x - self.x) as f64).atan2((other.y - self.y) as f64))
    }

    fn is_blocking(&self, target: &Point, to_check: &Point) -> bool {
        let a = self.dist(to_check) + to_check.dist(target);
        let b = self.dist(target);
        (a - b) < 0.00001
    }

    fn count_visible(&self, points: &[Point]) -> usize {
        let mut visible = 0;
        for other in points {
            if other == self {
                continue;
            }

            // Check if there is anything blocking the path to other
            let mut blocked = false;
            for potential_block in points {
                if potential_block == self || potential_block == other {
                    continue;
                }
                if self.is_blocking(other, potential_block) {
                    blocked = true;
                }
            }
            visible += if blocked { 0 } else { 1 };
        }
        visible
    }

    fn get_visible_other(&self, others: &HashSet<Point>) -> Vec<Point> {
        let mut angles = HashMap::<OrderedFloat<f64>, Point>::new();

        for other in others {
            if other == self {
                continue;
            }

            let angle = self.angle(other);
            angles
                .entry(OrderedFloat(angle))
                .and_modify(|p| {
                    if self.is_blocking(p, other) {
                        *p = other.clone();
                    }
                })
                .or_insert(other.clone());
        }

        angles.into_values().collect()
    }
}

fn calc_vaporizer_list(pos: &Point, others: &[Point]) -> Vec<Point> {
    let mut others_set = others.iter().cloned().collect::<HashSet<_>>();

    let mut list_to_remove = Vec::with_capacity(others.len());

    while others_set.len() > 1 {
        // Find next to delete and sort by angle
        let mut next_to_remove = pos.get_visible_other(&others_set);
        next_to_remove.sort_by(|a, b| pos.angle(a).partial_cmp(&pos.angle(b)).unwrap());

        // Remove next from original input
        for asteroid in next_to_remove {
            others_set.remove(&asteroid);
            list_to_remove.push(asteroid);
        }
    }

    list_to_remove
}

fn parse_asteroids(input: &str) -> Vec<Point> {
    let mut asteroids = Vec::<Point>::new();
    for (y, line) in input.lines().enumerate() {
        for (x, field) in line.chars().enumerate() {
            if field == '#' {
                asteroids.push(Point {
                    x: x as isize,
                    y: y as isize,
                });
            }
        }
    }
    asteroids
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_S: &str = r#".#..#
.....
#####
....#
...##
"#;

    const INPUT_M: &str = r#"......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####
"#;

    const INPUT_L: &str = r#".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##
"#;

    #[test]
    fn part_one() {
        let asteroids = parse_asteroids(INPUT_S);
        let max_visible = asteroids
            .iter()
            .map(|a| a.count_visible(&asteroids))
            .max()
            .unwrap();
        assert_eq!(max_visible, 8);

        let asteroids = parse_asteroids(INPUT_M);
        let max_visible = asteroids
            .iter()
            .map(|a| a.count_visible(&asteroids))
            .max()
            .unwrap();
        assert_eq!(max_visible, 33);

        let asteroids = parse_asteroids(INPUT_L);
        let max_visible = asteroids
            .iter()
            .map(|a| a.count_visible(&asteroids))
            .max()
            .unwrap();
        assert_eq!(max_visible, 210);
    }

    #[test]
    fn part_two() {
        let asteroids = parse_asteroids(INPUT_L);
        let best_asteroid = asteroids
            .iter()
            .max_by_key(|asteroid| asteroid.count_visible(&asteroids))
            .unwrap();
        let vaporizer_list = calc_vaporizer_list(&best_asteroid, &asteroids);
        println!("vaporizer_list first ten {:?}", &vaporizer_list[0..10]);
        let solution = vaporizer_list[199].x * 100 + vaporizer_list[199].y;
        assert_eq!(solution, 802)
    }
}
