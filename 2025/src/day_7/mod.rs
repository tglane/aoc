use crate::Day;
use anyhow::Result;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
    path::Path,
};

pub(crate) struct DaySeven {
    input: String,
}

impl Day for DaySeven {
    fn new<P: AsRef<Path>>(path: P) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            input: std::fs::read_to_string(path)?,
        })
    }

    fn part_one(&self) -> Result<()> {
        let diagram = TachyonDiagram::try_from(self.input.as_str()).unwrap();
        let splits = diagram.simulate();
        println!("Day 7 - Part 1: Number of beam splits: {splits}");
        Ok(())
    }

    fn part_two(&self) -> Result<()> {
        let diagram = TachyonDiagram::try_from(self.input.as_str()).unwrap();
        let timelines = diagram.simulate_quantum();
        println!("Day 7 - Part 2: Number of beam timelines: {timelines}");
        Ok(())
    }
}

struct TachyonDiagram {
    start: (usize, usize),
    splitters: HashSet<(usize, usize)>,
    max_y: usize,
}

impl TachyonDiagram {
    fn simulate(&self) -> usize {
        let mut seen_beams = Vec::<Beam>::new();
        let mut open_beams = VecDeque::from([Beam::new(self.start)]);
        let mut splitters = HashSet::new();

        while let Some(beam) = open_beams.pop_front() {
            if seen_beams.contains(&beam) {
                continue;
            }

            seen_beams.push(beam);
            let beam = seen_beams.last_mut().unwrap();

            if let Some((a, b)) = beam.simulate(&self.splitters, self.max_y) {
                let a_start = a.start();
                splitters.insert((a_start.0 + 1, a_start.1));

                open_beams.push_back(a);

                open_beams.push_back(b);
            }
        }

        splitters.len()
    }

    fn simulate_quantum(&self) -> usize {
        let mut cache = HashMap::<(usize, usize), usize>::new();
        Self::simulate_quantum_beam(
            Beam::new(self.start),
            &self.splitters,
            self.max_y,
            &mut cache,
        )
    }

    fn simulate_quantum_beam(
        mut beam: Beam,
        splitters: &HashSet<(usize, usize)>,
        max_y: usize,
        cache: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        if let Some(occurence) = cache.get(&beam.start()) {
            return *occurence;
        }

        let mut count = 0;

        if let Some((a, b)) = beam.simulate(splitters, max_y) {
            count += Self::simulate_quantum_beam(a, splitters, max_y, cache);
            count += Self::simulate_quantum_beam(b, splitters, max_y, cache);
        } else {
            return 1;
        }

        cache.insert(beam.start(), count);

        count
    }
}

impl TryFrom<&str> for TachyonDiagram {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut start = (0, 0);
        let mut splitters = HashSet::new();
        let mut max_y = 0;

        for (y, line) in value.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match Field::try_from(c) {
                    Ok(Field::Start) => start = (x, y),
                    Ok(Field::Splitter) => {
                        if y > max_y {
                            max_y = y;
                        }
                        splitters.insert((x, y));
                    }
                    _ => (),
                }
            }
        }

        Ok(Self {
            start,
            splitters,
            max_y: max_y + 1,
        })
    }
}

#[derive(Clone)]
struct Beam {
    start: (usize, usize),
    curr: (usize, usize),
    fields: HashSet<(usize, usize)>,
}

impl Beam {
    fn new(start: (usize, usize)) -> Self {
        Self {
            start,
            curr: start,
            fields: HashSet::from([start]),
        }
    }

    fn start(&self) -> (usize, usize) {
        self.start
    }

    fn peek_next(&self) -> (usize, usize) {
        (self.curr.0, self.curr.1 + 1)
    }

    fn next(&mut self) {
        self.curr = self.peek_next();
        self.fields.insert(self.curr);
    }

    fn simulate(
        &mut self,
        splitters: &HashSet<(usize, usize)>,
        max_y: usize,
    ) -> Option<(Beam, Beam)> {
        loop {
            let potential_next = self.peek_next();
            if potential_next.1 > max_y {
                // Out of map so we dont need to continue the simulation
                return None;
            } else if splitters.contains(&potential_next) {
                // Split the beam
                return Some((
                    Beam::new((potential_next.0 - 1, potential_next.1)),
                    Beam::new((potential_next.0 + 1, potential_next.1)),
                ));
            } else {
                // Grow the beam if no splitter is hit
                self.next();
            }
        }
    }
}

impl PartialEq for Beam {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start
    }
}

#[derive(Copy, Clone, Debug)]
enum Field {
    Start,
    Splitter,
    Beam,
    Empty,
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Start => 'S',
            Self::Splitter => '^',
            Self::Beam => '|',
            Self::Empty => '.',
        };
        write!(f, " {} ", c)
    }
}

impl TryFrom<char> for Field {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'S' => Ok(Self::Start),
            '^' => Ok(Self::Splitter),
            '|' => Ok(Self::Beam),
            '.' => Ok(Self::Empty),
            _ => Err(anyhow::Error::msg("Invalid field value")),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"#;

    #[test]
    fn part_one() {
        let diagram = TachyonDiagram::try_from(INPUT).unwrap();
        let splits = diagram.simulate();
        assert_eq!(splits, 21);
    }

    #[test]
    fn part_two() {
        let diagram = TachyonDiagram::try_from(INPUT).unwrap();
        let timelines = diagram.simulate_quantum();
        assert_eq!(timelines, 40);
    }
}
