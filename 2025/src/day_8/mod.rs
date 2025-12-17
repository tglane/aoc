use crate::Day;
use anyhow::Result;
use std::{collections::HashSet, hash::Hash, path::Path};

pub(crate) struct DayEight {
    input: String,
}

impl Day for DayEight {
    fn new<P: AsRef<Path>>(path: P) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            input: std::fs::read_to_string(path)?,
        })
    }

    fn part_one(&self) -> Result<()> {
        let points = self
            .input
            .lines()
            .map(Point3D::try_from)
            .collect::<Result<Vec<_>>>()?;
        let clusters = cluster_closest_points(&points, Some(1000));
        let size_of_three_largest = clusters
            .iter()
            .map(|cluster| cluster.len())
            .take(3)
            .product::<usize>();
        println!("Day 8 - Part 1: Size of three largest clusters: {size_of_three_largest}");
        Ok(())
    }

    fn part_two(&self) -> Result<()> {
        let points = self
            .input
            .lines()
            .map(Point3D::try_from)
            .collect::<Result<Vec<_>>>()?;
        let _clusters = cluster_closest_points(&points, None);
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point3D {
    x: isize,
    y: isize,
    z: isize,
}

impl Point3D {
    fn dist(&self, other: &Point3D) -> isize {
        let x_diff = (self.x - other.x).pow(2);
        let y_diff = (self.y - other.y).pow(2);
        let z_diff = (self.z - other.z).pow(2);
        x_diff + y_diff + z_diff
    }
}

impl TryFrom<&str> for Point3D {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut nums = value.splitn(3, ',').map(|s| s.parse::<isize>().ok());
        let p = Point3D {
            x: nums
                .next()
                .flatten()
                .ok_or(anyhow::Error::msg("Invalid number"))?,
            y: nums
                .next()
                .flatten()
                .ok_or(anyhow::Error::msg("Invalid number"))?,
            z: nums
                .next()
                .flatten()
                .ok_or(anyhow::Error::msg("Invalid number"))?,
        };
        if nums.next().is_some() {
            anyhow::bail!("More than 3 coordinates given")
        } else {
            Ok(p)
        }
    }
}

fn calc_dists<'a>(points: &'a [Point3D]) -> Vec<Dist<'a>> {
    let mut dists = HashSet::new();

    for a in 0..points.len() {
        for b in 0..points.len() {
            if a == b {
                continue;
            }
            let dist = Dist {
                dist: points[a].dist(&points[b]),
                a: &points[a],
                b: &points[b],
            };
            let reverse = Dist {
                dist: dist.dist,
                a: dist.b,
                b: dist.a,
            };
            if !dists.contains(&reverse) {
                dists.insert(dist);
            }
        }
    }
    dists.into_iter().collect()
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Dist<'a> {
    dist: isize,
    a: &'a Point3D,
    b: &'a Point3D,
}

fn cluster_closest_points(
    points: &[Point3D],
    max_points_to_cluster: Option<usize>,
) -> Vec<HashSet<Point3D>> {
    let mut dists = calc_dists(points);
    dists.sort_by_key(|dist| dist.dist);

    let mut clusters = points
        .iter()
        .map(|p| HashSet::from([p.clone()]))
        .collect::<Vec<_>>();

    for dist in dists
        .iter()
        .take(max_points_to_cluster.unwrap_or(usize::MAX))
    {
        let a_cluster = clusters
            .iter()
            .enumerate()
            .find(|(_, cluster)| cluster.contains(dist.a))
            .map(|(i, _)| i);
        let b_cluster = clusters
            .iter()
            .enumerate()
            .find(|(_, cluster)| cluster.contains(dist.b))
            .map(|(i, _)| i);

        // Since every point is already in a cluster (at least its own cluster) we only care for
        // the merging case when both are not in the same cluster. In the other case they already
        // are in the same cluster and we do not need to do anything.
        match (a_cluster, b_cluster) {
            (Some(a_cluster), Some(b_cluster)) if a_cluster != b_cluster => {
                // Merge cluster
                if a_cluster < b_cluster {
                    let removed_b = clusters.swap_remove(b_cluster);
                    clusters[a_cluster].extend(removed_b);
                } else {
                    let removed_a = clusters.swap_remove(a_cluster);
                    clusters[b_cluster].extend(removed_a);
                }
            }
            _ => (),
        }

        if clusters.len() == 1 {
            // Finished creating one single giant cluster that we can return
            println!(
                "Day 8 - Part 2: Multiply X coordinates of last points: {}",
                dist.a.x * dist.b.x
            );
            break;
        }
    }

    // Sort clusters by size before returning
    clusters.sort_by_key(|cluster| std::cmp::Reverse(cluster.len()));
    clusters
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
"#;

    #[test]
    fn part_one() {
        let points = INPUT
            .lines()
            .map(Point3D::try_from)
            .collect::<Result<Vec<_>>>()
            .unwrap();
        let clusters = cluster_closest_points(&points, Some(10));
        let size_of_three_largest = clusters
            .iter()
            .map(|cluster| cluster.len())
            .take(3)
            .fold(1, |acc, cluster_len| acc * cluster_len);
        assert_eq!(size_of_three_largest, 40);
    }

    #[test]
    fn part_two() {
        let points = INPUT
            .lines()
            .map(Point3D::try_from)
            .collect::<Result<Vec<_>>>()
            .unwrap();
        let clusters = cluster_closest_points(&points, None);
        assert_eq!(clusters.len(), 1);
    }
}
