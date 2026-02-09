use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet, VecDeque};

const DIRS: [Vec2D; 4] = [
    Vec2D { x: 1, y: 0 },
    Vec2D { x: -1, y: 0 },
    Vec2D { x: 0, y: 1 },
    Vec2D { x: 0, y: -1 },
];

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/src/day_18/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;

    let maze = Maze::try_from(input.as_str()).unwrap();
    let shortest_len = maze
        .steps_to_collect_keys()
        .context("No path to collect all keys found")?;
    println!("Day 18, Part 1: Shortest path to collect all keys: {shortest_len}");

    let mod_mazes = maze.partition();
    let shortest_len = mod_mazes
        .iter()
        .filter_map(|maze| maze.steps_to_collect_keys())
        .sum::<usize>();
    println!("Day 18, Part 2: Shortest path to collect all keys with four robots: {shortest_len}");

    Ok(())
}

#[derive(Debug)]
struct Maze {
    tiles: HashMap<Vec2D, Tile>,
    pos: Vec2D,
}

impl Maze {
    /// Split the maze into 4 new mazes by creating 4 new start positions around the original one
    /// and putting walls between them.
    ///
    /// Example ('@' marks the initial position inside the maze):
    /// #######       #######
    /// #a.#Cd#       #a.#Cd#
    /// ##...##       ##@#@##
    /// ##.@.##  -->  #######
    /// ##...##       ##@#@##
    /// #cB#Ab#       #cB#Ab#
    /// #######       #######
    ///
    fn partition(self) -> Vec<Self> {
        let keys = self.keys().collect::<Vec<_>>();
        self.pos
            .split()
            .into_iter()
            .map(|pos| {
                let mut tiles = self.tiles.clone();
                let org_pos = self.pos.clone();

                for dir in DIRS.into_iter() {
                    tiles.remove(&org_pos.add(dir));
                }
                tiles.remove(&org_pos);

                let mut maze = Maze { tiles, pos };

                // Remove unreachable keys
                for (key_pos, _) in keys.iter() {
                    if !maze.is_reachable(key_pos) {
                        maze.tiles.remove(key_pos);
                    }
                }

                // Remove doors for which no keys are in the maze
                let keys = maze.keys().map(|(_, k)| k).collect::<HashSet<_>>();

                for (_, tile) in maze.tiles.iter_mut() {
                    match tile {
                        Tile::Door(door) if !keys.contains(door) => {
                            *tile = Tile::Free;
                        }
                        _ => (),
                    }
                }

                maze
            })
            .collect::<Vec<_>>()
    }

    fn is_reachable(&self, tile: &Vec2D) -> bool {
        let mut queue = VecDeque::from([self.pos.clone()]);
        let mut seen = HashSet::new();

        while let Some(pos) = queue.pop_front() {
            if seen.contains(&pos) {
                continue;
            }
            seen.insert(pos.clone());

            if pos == *tile {
                return true;
            }

            for dir in DIRS.into_iter() {
                // Check the next tiles
                let next_pos = pos.add(dir);
                if self.tiles.contains_key(&next_pos) {
                    queue.push_back(next_pos);
                }
            }
        }

        false
    }

    fn steps_to_collect_keys(&self) -> Option<usize> {
        let initial_state = VisitorState {
            seen_keys: 0,
            pos: self.pos.clone(),
        };
        let mut queue = VecDeque::from([initial_state.clone()]);
        let mut cache = HashMap::<VisitorState, usize>::from([(initial_state, 0)]);

        while let Some(state) = queue.pop_front() {
            if let Some(steps_to) = cache.get(&state).cloned() {
                // If state already contains all keys we are done
                if !state.keys_missing(self) {
                    return Some(steps_to);
                }

                // Check each next direction
                for next_state in state.next(self) {
                    cache.entry(next_state.clone()).or_insert_with(|| {
                        queue.push_back(next_state.clone());
                        steps_to + 1
                    });
                }
            }
        }

        None
    }

    fn keys(&self) -> impl Iterator<Item = (Vec2D, u32)> {
        self.tiles.iter().filter_map(|(k, v)| match v {
            Tile::Key(key) => Some((k.clone(), *key)),
            _ => None,
        })
    }
}

impl TryFrom<&str> for Maze {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut tiles = HashMap::new();
        let mut pos = None;
        for (y, line) in value.lines().enumerate() {
            for (x, tile) in line.chars().enumerate() {
                if tile == '@' {
                    pos = Some(Vec2D {
                        x: x as i64,
                        y: y as i64,
                    });
                }
                if let Ok(tile) = Tile::try_from(tile) {
                    tiles.insert(
                        Vec2D {
                            x: x as i64,
                            y: y as i64,
                        },
                        tile,
                    );
                }
            }
        }
        Ok(Self {
            tiles,
            pos: pos.context("No position found")?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct VisitorState {
    seen_keys: u32,
    pos: Vec2D,
}

impl VisitorState {
    fn keys_missing(&self, maze: &Maze) -> bool {
        for (_, key) in maze.keys() {
            if (self.seen_keys >> key) & 1 != 1 {
                return true;
            }
        }
        false
    }

    fn next(&self, maze: &Maze) -> Vec<Self> {
        DIRS.into_iter()
            .filter_map(|dir| {
                let next_pos = self.pos.add(dir);

                match maze.tiles.get(&next_pos) {
                    Some(Tile::Free) => Some(VisitorState {
                        seen_keys: self.seen_keys,
                        pos: next_pos,
                    }),
                    Some(Tile::Key(key)) => {
                        if self.seen_keys & (1 << *key) > 0 {
                            Some(VisitorState {
                                seen_keys: self.seen_keys,
                                pos: next_pos,
                            })
                        } else {
                            Some(VisitorState {
                                seen_keys: self.seen_keys | 1 << key,
                                pos: next_pos,
                            })
                        }
                    }
                    Some(Tile::Door(door)) => {
                        if self.seen_keys & (1 << *door) > 0 {
                            Some(VisitorState {
                                seen_keys: self.seen_keys,
                                pos: next_pos,
                            })
                        } else {
                            None
                        }
                    }
                    None => None, // Ignore non-existing tiles
                }
            })
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Vec2D {
    x: i64,
    y: i64,
}

impl Vec2D {
    fn add(&self, other: Vec2D) -> Vec2D {
        Vec2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn split(&self) -> Vec<Self> {
        vec![
            Self {
                x: self.x + 1,
                y: self.y + 1,
            },
            Self {
                x: self.x + 1,
                y: self.y - 1,
            },
            Self {
                x: self.x - 1,
                y: self.y - 1,
            },
            Self {
                x: self.x - 1,
                y: self.y + 1,
            },
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Free,
    Door(u32),
    Key(u32),
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            '#' => anyhow::bail!("Ignoring walls"),
            '.' | '@' => Ok(Self::Free),
            c if c.is_alphabetic() && c.is_lowercase() => Ok(Self::Key((c as u32) - 97)),
            c if c.is_alphabetic() && c.is_uppercase() => Ok(Self::Door((c as u32) - 65)),
            c => anyhow::bail!("Invalid tile value: {c}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_S: &str = r#"#########
#b.A.@.a#
#########
"#;

    const INPUT_M: &str = r#"########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################
"#;

    const INPUT_L: &str = r#"#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################
"#;

    const INPUT_W: &str = r#"########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################
"#;

    const INPUT_MULTIPLE_S: &str = r#"#######
#a.#Cd#
##...##
##.@.##
##...##
#cB#Ab#
#######
"#;

    #[test]
    fn part_one() {
        let maze = Maze::try_from(INPUT_S).unwrap();
        let shortest_len = maze.steps_to_collect_keys().unwrap();
        assert_eq!(shortest_len, 8);

        let maze = Maze::try_from(INPUT_M).unwrap();
        let shortest_len = maze.steps_to_collect_keys().unwrap();
        assert_eq!(shortest_len, 132);

        let maze = Maze::try_from(INPUT_W).unwrap();
        let shortest_len = maze.steps_to_collect_keys().unwrap();
        assert_eq!(shortest_len, 81);

        let maze = Maze::try_from(INPUT_L).unwrap();
        let shortest_len = maze.steps_to_collect_keys().unwrap();
        assert_eq!(shortest_len, 136);
    }

    #[test]
    fn part_two() {
        let maze = Maze::try_from(INPUT_MULTIPLE_S).unwrap();
        let mod_mazes = maze.partition();
        let shortest_len = mod_mazes
            .iter()
            .filter_map(|maze| maze.steps_to_collect_keys())
            .sum::<usize>();
        assert_eq!(shortest_len, 8);
    }
}
