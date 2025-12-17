use crate::Day;
use anyhow::{Context, Result};
use std::{ops::RangeInclusive, path::Path};

pub(crate) struct DayNine {
    input: String,
}

impl Day for DayNine {
    fn new<P: AsRef<Path>>(path: P) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            input: std::fs::read_to_string(path)?,
        })
    }

    fn part_one(&self) -> Result<()> {
        let points = parse_input(self.input.as_str())?;
        let max_surface = largest_rectangle(&points).context("No points in input")?;
        println!("Day 9 - Part 1: Max surface area: {}", max_surface.area);
        Ok(())
    }

    fn part_two(&self) -> Result<()> {
        let points = parse_input(self.input.as_str()).unwrap();
        let max_surface_in_poly = largest_rectangle_bounded(&points).unwrap();
        println!(
            "Day 9 - Part 2: Max surface area in polygon: {}",
            max_surface_in_poly.area
        );
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
}

enum Orientation {
    Horizontal, // x stays same
    Vertical,   // y stays same
}

#[derive(Debug)]
struct Line<'p> {
    a: &'p Point,
    b: &'p Point,
}

impl<'p> Line<'p> {
    fn new(a: &'p Point, b: &'p Point) -> Self {
        Self { a, b }
    }

    fn orientation(&self) -> Orientation {
        if self.a.y == self.b.y {
            Orientation::Horizontal
        } else if self.a.x == self.b.x {
            Orientation::Vertical
        } else {
            panic!("Should not be diagonal");
        }
    }

    fn horizontal_range(&self) -> RangeInclusive<isize> {
        let start = self.a.x.min(self.b.x);
        let end = self.a.x.max(self.b.x);
        start..=end
    }

    fn vertical_range(&self) -> RangeInclusive<isize> {
        let start = self.a.y.min(self.b.y);
        let end = self.a.y.max(self.b.y);
        start..=end
    }

    fn intersects(&self, other: Line<'_>) -> bool {
        match (self.orientation(), other.orientation()) {
            (Orientation::Horizontal, Orientation::Vertical) => {
                self.horizontal_range().contains(&other.b.x)
                    && other.vertical_range().contains(&self.b.y)
            }
            (Orientation::Vertical, Orientation::Horizontal) => {
                self.vertical_range().contains(&other.b.y)
                    && other.horizontal_range().contains(&self.b.x)
            }
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Surface {
    top_left: Point,
    top_right: Point,
    bottom_right: Point,
    bottom_left: Point,
    area: usize,
}

impl Surface {
    fn new(a: &Point, b: &Point) -> Self {
        let x_diff = b.x.abs_diff(a.x) + 1;
        let y_diff = b.y.abs_diff(a.y) + 1;

        let min_x = a.x.min(b.x);
        let min_y = a.y.min(b.y);
        let max_x = a.x.max(b.x);
        let max_y = a.y.max(b.y);

        let top_left = Point { x: min_x, y: min_y };
        let bottom_right = Point { x: max_x, y: max_y };

        let top_right = Point { x: max_x, y: min_y };
        let bottom_left = Point { x: min_x, y: max_y };

        Self {
            top_left,
            top_right,
            bottom_right,
            bottom_left,
            area: x_diff * y_diff,
        }
    }

    fn points(&self) -> [&Point; 4] {
        [
            &self.top_left,
            &self.top_right,
            &self.bottom_right,
            &self.bottom_left,
        ]
    }

    fn shrinked(&self) -> Self {
        let mut top_left = self.top_left.clone();
        top_left.x += 1;
        top_left.y += 1;

        let mut bottom_right = self.bottom_right.clone();
        bottom_right.x -= 1;
        bottom_right.y -= 1;

        Self::new(&top_left, &bottom_right)
    }

    fn lines(&self) -> [Line<'_>; 4] {
        [
            Line::new(&self.top_right, &self.top_left),
            Line::new(&self.bottom_right, &self.bottom_left),
            Line::new(&self.top_left, &self.bottom_left),
            Line::new(&self.top_right, &self.bottom_right),
        ]
    }
}

impl PartialOrd for Surface {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Surface {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.area.cmp(&other.area)
    }
}

struct PolygonRef<'p>(&'p [Point]);

impl<'p> PolygonRef<'p> {
    fn contains_point(&self, point: &'p Point) -> bool {
        let mut inside = false;
        let second = self.0.iter().skip(1).chain([&self.0[0]]);
        for (a, b) in self.0.iter().zip(second) {
            if (a.y > point.y) != (b.y > point.y)
                && point.x < (b.x - a.x) * (point.y - a.y) / (b.y - a.y) + a.x
            {
                inside = !inside;
            }
        }
        inside
    }

    fn contains(&self, surface: &Surface) -> bool {
        // Check if points are inside polygon
        for point in surface.points() {
            if !self.contains_point(point) {
                return false;
            }
        }

        // Check if lines cross the boundaries of the polygon
        for line in surface.lines() {
            let second = self.0.iter().skip(1).chain([&self.0[0]]);
            for (a, b) in self.0.iter().zip(second) {
                if line.intersects(Line::new(a, b)) {
                    return false;
                }
            }
        }

        true
    }
}

fn largest_rectangle(points: &[Point]) -> Option<Surface> {
    let mut max_surface: Option<Surface> = None;
    for a in 0..points.len() {
        for b in a + 1..points.len() {
            let surface = Surface::new(&points[a], &points[b]);
            if max_surface.as_ref().is_none_or(|max_surface| max_surface.area <= surface.area)
            {
                max_surface = Some(surface);
            }
        }
    }
    max_surface
}

fn largest_rectangle_bounded(points: &[Point]) -> Option<Surface> {
    let mut surfaces = Vec::with_capacity(points.len().pow(2));
    for a in 0..points.len() {
        for b in a + 1..points.len() {
            let surface = Surface::new(&points[a], &points[b]);
            surfaces.push(surface);
        }
    }
    surfaces.sort_by_key(|s| std::cmp::Reverse(s.area));

    let polygon = PolygonRef(points);

    surfaces
        .into_iter()
        .find(|surface| polygon.contains(&surface.shrinked()))
}

fn parse_input(input: &str) -> Result<Vec<Point>> {
    let points = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(',').ok_or(anyhow::Error::msg(""))?;
            let x = a.parse::<isize>()?;
            let y = b.parse::<isize>()?;
            Ok(Point { x, y })
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(points)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

    #[test]
    fn part_one() {
        let points = parse_input(INPUT).unwrap();
        let max_surface = largest_rectangle(&points).unwrap();
        assert_eq!(max_surface.area, 50);
    }

    #[test]
    fn part_two() {
        let points = parse_input(INPUT).unwrap();
        let max_surface_in_poly = largest_rectangle_bounded(&points).unwrap();
        assert_eq!(max_surface_in_poly.area, 24);
    }
}
