use anyhow::{Context, Result};
use std::collections::HashMap;

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/src/day_3/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;

    let wires = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(Wire::try_from)
        .collect::<Result<Vec<Wire>>>()
        .unwrap();
    let closest_intersection_dist =
        find_closest_intersection(&wires[0], &wires[1], Point { x: 0, y: 0 })
            .context("No intersection found")?;
    println!("Day 3, Part 1: Closest intersection distance is {closest_intersection_dist}");

    let lowest_latency = find_low_latency_intersection(&wires[0], &wires[1], Point { x: 0, y: 0 })
        .context("No intersection found")?;
    println!("Day 3, Part 2: Lowest latency intersection is {lowest_latency}");

    Ok(())
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl TryFrom<&str> for Direction {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value {
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            "R" => Ok(Self::Right),
            "L" => Ok(Self::Left),
            _ => anyhow::bail!("Invalid direction encoding"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn next(&self, dir: Direction, amount: isize) -> Self {
        match dir {
            Direction::Up => Self {
                x: self.x,
                y: self.y + amount,
            },
            Direction::Down => Self {
                x: self.x,
                y: self.y - amount,
            },
            Direction::Right => Self {
                x: self.x + amount,
                y: self.y,
            },
            Direction::Left => Self {
                x: self.x - amount,
                y: self.y,
            },
        }
    }

    fn distance(&self, other: &Point) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Segment {
    start: Point,
    end: Point,
}

impl Segment {
    fn len(&self) -> usize {
        self.start.distance(&self.end)
    }

    fn intersects(&self, other: &Segment) -> Option<Point> {
        let denominator = ((self.end.x - self.start.x) * (other.end.y - other.start.y))
            - ((self.end.y - self.start.y) * (other.end.x - other.start.x));

        if denominator == 0 {
            return None;
        }

        let px = (((self.start.x * self.end.y - self.start.y * self.end.x)
            * (other.start.x - other.end.x))
            - ((self.start.x - self.end.x)
                * (other.start.x * other.end.y - other.start.y * other.end.x)))
            / denominator;

        let py = (((self.start.x * self.end.y - self.start.y * self.end.x)
            * (other.start.y - other.end.y))
            - ((self.start.y - self.end.y)
                * (other.start.x * other.end.y - other.start.y * other.end.x)))
            / denominator;

        // Check if the intersection is inside the segments
        let intersection = Point { x: px, y: py };
        if intersection.distance(&self.start) + intersection.distance(&self.end) != self.len()
            || intersection.distance(&other.start) + intersection.distance(&other.end)
                != other.len()
        {
            return None;
        }

        Some(intersection)
    }
}

#[derive(Clone, Debug)]
struct Wire {
    segments: Vec<Segment>,
}

impl TryFrom<&str> for Wire {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // Each wire starts in (0,0)
        let mut segments = Vec::new();
        let mut curr = Point { x: 0, y: 0 };
        for step in value.split(',') {
            let (dir, amount) = step.split_at(1);
            let next = curr.next(Direction::try_from(dir)?, amount.parse::<isize>()?);
            segments.push(Segment {
                start: curr.clone(),
                end: next.clone(),
            });
            curr = next;
        }
        Ok(Self { segments })
    }
}

fn find_closest_intersection(a: &Wire, b: &Wire, center: Point) -> Option<usize> {
    let mut closest = None;

    for a_seg in a.segments.iter() {
        for b_seg in b.segments.iter() {
            // Get possible intersection point
            if let Some(intersection) = a_seg.intersects(b_seg) {
                // Check if intersection is closer to center than current closest

                if intersection == center {
                    continue;
                }

                let intersection_dist = center.distance(&intersection);
                if let Some(curr_closest) = closest.as_mut() {
                    *curr_closest = std::cmp::min(intersection_dist, *curr_closest);
                } else {
                    closest = Some(intersection_dist);
                }
            }
        }
    }

    closest
}

fn find_low_latency_intersection(a: &Wire, b: &Wire, center: Point) -> Option<usize> {
    let mut lowest_latency = None;

    let mut a_latency_before = HashMap::<Segment, usize>::new();
    let mut latency_sum = 0;
    for a_seg in a.segments.iter() {
        a_latency_before.insert(a_seg.clone(), latency_sum);
        latency_sum += a_seg.len();
    }

    let mut b_latency_before = HashMap::<Segment, usize>::new();
    latency_sum = 0;
    for b_seg in b.segments.iter() {
        b_latency_before.insert(b_seg.clone(), latency_sum);
        latency_sum += b_seg.len();
    }

    for a_seg in a.segments.iter() {
        for b_seg in b.segments.iter() {
            // Get possible intersection point
            if let Some(intersection) = a_seg.intersects(b_seg) {
                if intersection == center {
                    continue;
                }

                let a_latency = a_latency_before.get(a_seg).cloned().unwrap_or(usize::MAX)
                    + a_seg.start.distance(&intersection);
                let b_latency = b_latency_before.get(b_seg).cloned().unwrap_or(usize::MAX)
                    + b_seg.start.distance(&intersection);

                if let Some(lowest_latency) = lowest_latency.as_mut() {
                    *lowest_latency = std::cmp::min(*lowest_latency, a_latency + b_latency);
                } else {
                    lowest_latency = Some(a_latency + b_latency);
                }
            }
        }
    }

    lowest_latency
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT2: &str = r#"R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"#;
    const INPUT1: &str = r#"R8,U5,L5,D3
U7,R6,D4,L4"#;

    #[test]
    fn part_one() {
        let wires = INPUT1
            .split('\n')
            .map(Wire::try_from)
            .collect::<Result<Vec<Wire>>>()
            .unwrap();
        let closest_intersection_dist =
            find_closest_intersection(&wires[0], &wires[1], Point { x: 0, y: 0 }).unwrap();
        assert_eq!(closest_intersection_dist, 6);

        let wires = INPUT2
            .split('\n')
            .map(Wire::try_from)
            .collect::<Result<Vec<Wire>>>()
            .unwrap();
        let closest_intersection_dist =
            find_closest_intersection(&wires[0], &wires[1], Point { x: 0, y: 0 }).unwrap();
        assert_eq!(closest_intersection_dist, 159);
    }

    #[test]
    fn part_two() {
        let wires = INPUT1
            .split('\n')
            .map(Wire::try_from)
            .collect::<Result<Vec<Wire>>>()
            .unwrap();
        let lowest_latency =
            find_low_latency_intersection(&wires[0], &wires[1], Point { x: 0, y: 0 }).unwrap();
        assert_eq!(lowest_latency, 30);

        let wires = INPUT2
            .split('\n')
            .map(Wire::try_from)
            .collect::<Result<Vec<Wire>>>()
            .unwrap();
        let lowest_latency =
            find_low_latency_intersection(&wires[0], &wires[1], Point { x: 0, y: 0 }).unwrap();
        assert_eq!(lowest_latency, 610);
    }
}
