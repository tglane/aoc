use anyhow::Result;
use std::collections::{HashSet, VecDeque};
use std::hash::Hash;

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/src/day_24/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;

    let scan = Scan::try_from(input.as_str()).unwrap();
    let recurring_scan = detect_recurring_scan(scan);
    println!(
        "Day 24, Part 1: Biodiversity of first recurring scan: {}",
        recurring_scan.biodiversity()
    );

    let scan = Scan::try_from(input.as_str()).unwrap();
    let bugs_after = bugs_after(scan, 200);
    println!(
        "Day 24, Part 2: Bugs after 200 iterations on the infinite grid: {}",
        bugs_after
    );

    Ok(())
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Field {
    Empty,
    Bug,
}

impl TryFrom<char> for Field {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            '.' => Ok(Field::Empty),
            '#' => Ok(Field::Bug),
            c => anyhow::bail!("Failed to parse {c}"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Scan {
    map: Vec<Vec<Field>>,
}

impl Scan {
    const OFFSETS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    fn inner(&self) -> &Vec<Vec<Field>> {
        &self.map
    }

    fn x_len(&self) -> usize {
        self.map[0].len()
    }

    fn y_len(&self) -> usize {
        self.map.len()
    }

    fn bugs(&self) -> usize {
        self.map
            .iter()
            .map(|line| {
                line.iter()
                    .filter(|field| matches!(field, Field::Bug))
                    .count()
            })
            .sum()
    }

    fn biodiversity(&self) -> usize {
        let mut biodiversity = 0;
        for (y, line) in self.map.iter().enumerate() {
            for (x, field) in line.iter().enumerate() {
                if matches!(field, Field::Bug) {
                    let exp = y * self.map[0].len() + x;
                    biodiversity += 2_usize.pow(exp as u32);
                }
            }
        }
        biodiversity
    }

    fn next(&self) -> Self {
        let mut new_map = vec![vec![Field::Empty; 5]; 5];

        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                let adj_bugs = Self::OFFSETS.iter().fold(0, |adj, (x_delta, y_delta)| {
                    let nx = x as isize + x_delta;
                    let ny = y as isize + y_delta;
                    if self
                        .map
                        .get(ny as usize)
                        .and_then(|line| line.get(nx as usize))
                        .is_some_and(|field| matches!(*field, Field::Bug))
                    {
                        return adj + 1;
                    }
                    adj
                });

                let new_field = match self.map[y][x] {
                    Field::Empty if adj_bugs == 1 || adj_bugs == 2 => Field::Bug,
                    Field::Bug if adj_bugs != 1 => Field::Empty,
                    field => field,
                };
                new_map[y][x] = new_field;
            }
        }

        Self { map: new_map }
    }
}

impl TryFrom<&str> for Scan {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let map = value
            .lines()
            .map(|line| {
                line.chars()
                    .map(Field::try_from)
                    .collect::<Result<Vec<_>>>()
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { map })
    }
}

#[derive(Clone, Debug)]
struct InfiniteScan {
    layers: VecDeque<Scan>,
    inner_recursion_point: (usize, usize),
}

impl InfiniteScan {
    fn new(initial: Scan) -> Self {
        let inner_recursion_point = (initial.map.len() / 2, initial.map[0].len() / 2);
        let mut this = Self {
            layers: VecDeque::from([initial]),
            inner_recursion_point,
        };
        this.add_empty_layers();
        this.add_empty_layers();
        this
    }

    fn bugs(&self) -> usize {
        self.layers.iter().map(|layer| layer.bugs()).sum()
    }

    fn next(&self) -> Self {
        let mut next = self.clone();
        next.add_empty_layers();

        for i in 1..self.layers.len() - 1 {
            let layer = self.layers[i].inner();

            let mut new_layer = vec![vec![Field::Empty; 5]; 5];

            for y in 0..layer.len() {
                for x in 0..layer[y].len() {
                    if x == self.inner_recursion_point.0 && y == self.inner_recursion_point.1 {
                        continue;
                    }

                    let mut adj_bugs = Scan::OFFSETS.iter().fold(0, |adj, (x_delta, y_delta)| {
                        let nx = x as isize + x_delta;
                        let ny = y as isize + y_delta;
                        if layer
                            .get(ny as usize)
                            .and_then(|line| line.get(nx as usize))
                            .is_some_and(|field| matches!(*field, Field::Bug))
                        {
                            return adj + 1;
                        }
                        adj
                    });

                    // Check for bugs in outer/inner layer "neighbours"
                    if x == 0
                        && matches!(
                            self.layers[i - 1].inner()[self.inner_recursion_point.1]
                                [self.inner_recursion_point.0 - 1],
                            Field::Bug
                        )
                    {
                        adj_bugs += 1;
                    }
                    if x == layer[y].len() - 1
                        && matches!(
                            self.layers[i - 1].inner()[self.inner_recursion_point.1]
                                [self.inner_recursion_point.0 + 1],
                            Field::Bug
                        )
                    {
                        adj_bugs += 1;
                    }
                    if y == 0
                        && matches!(
                            self.layers[i - 1].inner()[self.inner_recursion_point.1 - 1]
                                [self.inner_recursion_point.0],
                            Field::Bug
                        )
                    {
                        adj_bugs += 1;
                    }
                    if y == layer.len() - 1
                        && matches!(
                            self.layers[i - 1].inner()[self.inner_recursion_point.1 + 1]
                                [self.inner_recursion_point.0],
                            Field::Bug
                        )
                    {
                        adj_bugs += 1;
                    }
                    if x == self.inner_recursion_point.0 && y == self.inner_recursion_point.1 - 1 {
                        for inner in &self.layers[i + 1].map[0] {
                            if matches!(inner, Field::Bug) {
                                adj_bugs += 1;
                            }
                        }
                    }
                    if x == self.inner_recursion_point.0 && y == self.inner_recursion_point.1 + 1 {
                        for inner in &self.layers[i + 1].map[4] {
                            if matches!(inner, Field::Bug) {
                                adj_bugs += 1;
                            }
                        }
                    }
                    if x == self.inner_recursion_point.0 - 1 && y == self.inner_recursion_point.1 {
                        for yy in 0..layer.len() {
                            if matches!(self.layers[i + 1].inner()[yy][0], Field::Bug) {
                                adj_bugs += 1;
                            }
                        }
                    }
                    if x == self.inner_recursion_point.0 + 1 && y == self.inner_recursion_point.1 {
                        for yy in 0..layer.len() {
                            if matches!(self.layers[i + 1].inner()[yy][4], Field::Bug) {
                                adj_bugs += 1;
                            }
                        }
                    }

                    let new_field = match layer[y][x] {
                        Field::Empty if adj_bugs == 1 || adj_bugs == 2 => Field::Bug,
                        Field::Bug if adj_bugs != 1 => Field::Empty,
                        field => field,
                    };
                    new_layer[y][x] = new_field;
                }
            }

            next.layers[i + 1].map = new_layer;
        }

        next
    }

    fn add_empty_layers(&mut self) {
        let current_layer = &self.layers[0];
        let empty_layer = Scan {
            map: vec![vec![Field::Empty; current_layer.x_len()]; current_layer.y_len()],
        };
        self.layers.push_front(empty_layer.clone());
        self.layers.push_back(empty_layer.clone());
    }
}

fn detect_recurring_scan(mut scan: Scan) -> Scan {
    let mut seen = HashSet::<Scan>::new();
    loop {
        if seen.contains(&scan) {
            return scan;
        }
        let next_scan = scan.next();
        seen.insert(std::mem::replace(&mut scan, next_scan));
    }
}

fn bugs_after(scan: Scan, iterations: usize) -> usize {
    let mut infinite_scan = InfiniteScan::new(scan);
    for _ in 0..iterations {
        infinite_scan = infinite_scan.next();
    }
    infinite_scan.bugs()
}

// Bug -> Empty if bugs around != 1
// EMpty -> Bug if bugs around == 1 or == 2

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"....#
#..#.
#..##
..#..
#....
"#;

    #[test]
    fn part_one() {
        let scan = Scan::try_from(INPUT).unwrap();
        let recurring_scan = detect_recurring_scan(scan);
        recurring_scan.print();
        assert_eq!(recurring_scan.biodiversity(), 2129920);
    }

    #[test]
    fn part_two() {
        let scan = Scan::try_from(INPUT).unwrap();
        let bugs_after = bugs_after(scan, 10);
        assert_eq!(bugs_after, 99);
    }
}
